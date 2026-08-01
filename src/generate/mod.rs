//! Lifecycle orchestration: construct → stage → verify → place.
//!
//! REQ-050 success cleans stage; fail retains stage and prints path.

use std::path::Path;

use crate::catalog::{
    CatalogView, catalog_view_for_units, default_cli_catalog_view, load_embedded_catalog,
};
use crate::fsx::{self, StageHandle};
use crate::plan::{Plan, construct};
use crate::render::render;
use crate::resolve::resolve_composition;
use crate::spec::EffectiveInputs;
use crate::verify::{VerifyHook, VerifyOutcome, phase01_stub_hook};

/// Generate lifecycle error.
#[derive(Debug)]
pub struct GenerateError {
    /// Stable error code.
    pub code: &'static str,
    /// Human message.
    pub message: String,
    /// Stage path retained on failure (if any).
    pub stage_path: Option<std::path::PathBuf>,
}

impl GenerateError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            stage_path: None,
        }
    }

    fn with_stage(mut self, stage: &Path) -> Self {
        self.stage_path = Some(stage.to_path_buf());
        self
    }
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)?;
        if let Some(p) = &self.stage_path {
            write!(f, " (stage retained: {})", p.display())?;
        }
        Ok(())
    }
}

impl std::error::Error for GenerateError {}

/// Successful generate result.
#[derive(Debug, Clone)]
pub struct GenerateResult {
    /// Final plan used (contract-checked).
    pub plan: Plan,
    /// Destination that was placed.
    pub destination: std::path::PathBuf,
}

/// Run generate lifecycle with embedded catalog view for resolved composition.
pub fn generate(inputs: &EffectiveInputs) -> Result<GenerateResult, GenerateError> {
    let catalog = catalog_for_inputs(inputs)?;
    let hook = phase01_stub_hook(inputs.verify);
    generate_with(inputs, &catalog, hook.as_ref())
}

fn catalog_for_inputs(inputs: &EffectiveInputs) -> Result<CatalogView, GenerateError> {
    let composition =
        resolve_composition(inputs).map_err(|e| GenerateError::new(e.code, e.message))?;
    match load_embedded_catalog() {
        Ok(embedded) => catalog_view_for_units(&embedded, &composition.unit_ids)
            .map_err(|e| GenerateError::new(e.code, e.message)),
        Err(_) => Ok(default_cli_catalog_view()),
    }
}

/// Full generate with injectable catalog + verify hook (tests).
pub fn generate_with(
    inputs: &EffectiveInputs,
    catalog: &CatalogView,
    verify: &dyn VerifyHook,
) -> Result<GenerateResult, GenerateError> {
    // 1. Construct (first) — contract plan.
    let plan = construct(inputs, catalog).map_err(|e| GenerateError::new(e.code, e.message))?;

    // 2. Independent re-construct; digest must match before any FS write.
    let plan2 = construct(inputs, catalog).map_err(|e| GenerateError::new(e.code, e.message))?;
    if plan.plan_sha256 != plan2.plan_sha256 {
        return Err(GenerateError::new(
            "generate.plan_contract",
            format!(
                "plan_sha256 mismatch before FS write: {} != {}",
                plan.plan_sha256, plan2.plan_sha256
            ),
        ));
    }

    // 3. Pure render
    let map = render(&plan, catalog).map_err(|e| GenerateError::new(e.code, e.message))?;

    // 4. Stage (first FS write)
    let dest = Path::new(&inputs.destination);
    let stage =
        fsx::stage_render_map(dest, &map).map_err(|e| GenerateError::new(e.code, e.message))?;

    // 5. Verify hook
    match verify.run(&stage.stage_path) {
        VerifyOutcome::Pass => {}
        VerifyOutcome::Fail { message } => {
            return Err(
                GenerateError::new("generate.verify_failed", message).with_stage(&stage.stage_path)
            );
        }
    }

    // 6. Exclusive place
    if let Err(e) = fsx::exclusive_place(&stage) {
        return Err(GenerateError::new(e.code, e.message).with_stage(&stage.stage_path));
    }

    // 7. Success: stage was renamed away; nothing to clean if rename consumed it.
    // If stage somehow remains, clean it.
    if stage.stage_path.exists() {
        let _ = fsx::clean_stage(&stage.stage_path);
    }

    Ok(GenerateResult {
        plan,
        destination: stage.destination,
    })
}

/// Expose stage handle type for tests.
pub type Stage = StageHandle;

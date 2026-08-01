//! MS-005: render / fsx / verify stub / generate lifecycle integration.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use foundry::catalog::stub_catalog;
use foundry::fsx::{self, Admissibility};
use foundry::generate::generate_with;
use foundry::plan::construct;
use foundry::render::render;
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};
use foundry::verify::{AlwaysPass, ForcedFail};

fn tempfile_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn inputs_for_dest(dest: &Path) -> foundry::spec::EffectiveInputs {
    let body = format!(
        r#"
schema = 1
name = "example-cli"
archetype = "cli"
destination = "{}"
profiles = []
verify = "none"
"#,
        dest.display()
    );
    let spec = parse_spec_str(&body, "<t>").unwrap();
    normalize_effective_inputs(spec, CliOverrides::default()).unwrap()
}

#[test]
fn render_pure_deterministic() {
    let inputs = inputs_for_dest(Path::new("./example-cli"));
    let cat = stub_catalog();
    let plan = construct(&inputs, &cat).unwrap();
    let a = render(&plan, &cat).unwrap();
    let b = render(&plan, &cat).unwrap();
    assert_eq!(a, b);
    assert!(a.contains_key("Cargo.toml"));
}

#[test]
fn stage_leaves_dest_untouched() {
    let dir = tempfile_dir("stage");
    let dest = dir.join("proj");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("KEEP"), b"x").unwrap();
    let inputs = inputs_for_dest(&dest);
    let cat = stub_catalog();
    let plan = construct(&inputs, &cat).unwrap();
    let map = render(&plan, &cat).unwrap();
    let stage = fsx::stage_render_map(&dest, &map).unwrap();
    assert!(stage.stage_path.is_dir());
    assert!(stage.stage_path.join("Cargo.toml").is_file());
    assert_eq!(fs::read_to_string(dest.join("KEEP")).unwrap(), "x");
    fsx::clean_stage(&stage.stage_path).unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn place_missing_success_and_cleans_stage() {
    let dir = tempfile_dir("place-missing");
    let dest = dir.join("newproj");
    assert!(!dest.exists());
    let inputs = inputs_for_dest(&dest);
    let cat = stub_catalog();
    let plan = construct(&inputs, &cat).unwrap();
    let map = render(&plan, &cat).unwrap();
    let stage = fsx::stage_render_map(&dest, &map).unwrap();
    let stage_path = stage.stage_path.clone();
    fsx::exclusive_place(&stage).unwrap();
    assert!(dest.join("Cargo.toml").is_file());
    assert!(!stage_path.exists(), "stage cleaned via rename");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn place_empty_dir_ok() {
    let dir = tempfile_dir("place-empty");
    let dest = dir.join("emptyproj");
    fs::create_dir_all(&dest).unwrap();
    assert_eq!(
        fsx::classify_destination(&dest).unwrap(),
        Admissibility::EmptyDir
    );
    let inputs = inputs_for_dest(&dest);
    let result = generate_with(&inputs, &stub_catalog(), &AlwaysPass).unwrap();
    assert!(result.destination.join("src/main.rs").is_file());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn place_refuse_non_empty() {
    let dir = tempfile_dir("refuse");
    let dest = dir.join("full");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join(".git"), b"not empty").unwrap();
    let inputs = inputs_for_dest(&dest);
    let err = generate_with(&inputs, &stub_catalog(), &AlwaysPass).unwrap_err();
    assert_eq!(err.code, "fsx.refuse_non_empty");
    assert!(dest.join(".git").is_file(), "dest unchanged");
    assert!(err.stage_path.is_some());
    assert!(err.stage_path.as_ref().unwrap().exists());
    let _ = fs::remove_dir_all(err.stage_path.as_ref().unwrap());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn place_refuse_file_at_path() {
    let dir = tempfile_dir("file-dest");
    let dest = dir.join("as-file");
    fs::write(&dest, b"i am a file").unwrap();
    let inputs = inputs_for_dest(&dest);
    let err = generate_with(&inputs, &stub_catalog(), &AlwaysPass).unwrap_err();
    assert_eq!(err.code, "fsx.refuse_non_empty");
    assert!(err.message.contains("file_at_path") || err.message.contains("not admissible"));
    let _ = fs::remove_dir_all(err.stage_path.as_ref().unwrap());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn verify_fail_retains_stage_dest_clean() {
    let dir = tempfile_dir("verify-fail");
    let dest = dir.join("vf");
    assert!(!dest.exists());
    let inputs = inputs_for_dest(&dest);
    let hook = ForcedFail {
        message: "injected verify failure".into(),
    };
    let err = generate_with(&inputs, &stub_catalog(), &hook).unwrap_err();
    assert_eq!(err.code, "generate.verify_failed");
    assert!(!dest.exists(), "dest untouched");
    let stage = err.stage_path.as_ref().unwrap();
    assert!(stage.exists(), "stage retained");
    assert!(stage.join("Cargo.toml").is_file());
    let _ = fs::remove_dir_all(stage);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn success_lifecycle_dest_complete_stage_gone() {
    let dir = tempfile_dir("success");
    let dest = dir.join("ok");
    let inputs = inputs_for_dest(&dest);
    let result = generate_with(&inputs, &stub_catalog(), &AlwaysPass).unwrap();
    assert!(result.destination.join("Cargo.toml").is_file());
    assert!(result.destination.join("AGENTS.md").is_file());
    // No leftover stage dirs next to dest
    let parent = result.destination.parent().unwrap();
    for ent in fs::read_dir(parent).unwrap() {
        let name = ent.unwrap().file_name().to_string_lossy().into_owned();
        assert!(
            !name.starts_with(".foundry-stage-"),
            "leftover stage {name}"
        );
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn cli_generate_success() {
    let dir = tempfile_dir("cli-gen");
    let dest = dir.join("cli-proj");
    let spec = dir.join("spec.toml");
    fs::write(
        &spec,
        format!(
            r#"
schema = 1
name = "cli-proj"
archetype = "cli"
destination = "{}"
profiles = []
"#,
            dest.display()
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args(["generate", "--spec", spec.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dest.join("Cargo.toml").is_file());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("foundry generate: ok"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn cli_generate_refuse_nonempty() {
    let dir = tempfile_dir("cli-refuse");
    let dest = dir.join("taken");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("x"), b"1").unwrap();
    let spec = dir.join("spec.toml");
    fs::write(
        &spec,
        format!(
            r#"
schema = 1
name = "taken"
archetype = "cli"
destination = "{}"
profiles = []
"#,
            dest.display()
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args(["generate", "--spec", spec.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("fsx.refuse_non_empty"));
    assert!(stderr.contains("stage retained") || stderr.contains("stage"));
    assert!(dest.join("x").is_file());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn no_merge_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_foundry"))
        .args(["merge", "--help"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

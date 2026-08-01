//! Stage and exclusive place (filesystem).
//!
//! Sibling stage under destination parent; exclusive place (REQ-051).
//! Path jail on materialize (REQ-053).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::plan::{FileMode, assert_path_jailed};
use crate::render::RenderMap;

/// Failure in stage / place filesystem operations.
#[derive(Debug)]
pub struct FsxError {
    /// Stable error code.
    pub code: &'static str,
    /// Human message.
    pub message: String,
}

impl FsxError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for FsxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for FsxError {}

/// Result of staging a render map.
#[derive(Debug, Clone)]
pub struct StageHandle {
    /// Absolute path to the stage directory.
    pub stage_path: PathBuf,
    /// Destination path that will receive the place.
    pub destination: PathBuf,
}

/// Create a sibling stage directory next to `destination` and materialize `map`.
///
/// Stage root policy: `<dest_parent>/.foundry-stage-<dest_basename>-<pid>-<nanos>/`.
/// Destination is never written. Path jail applied to every map key (REQ-053).
pub fn stage_render_map(destination: &Path, map: &RenderMap) -> Result<StageHandle, FsxError> {
    let dest = if destination.is_absolute() {
        destination.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| FsxError::new("fsx.cwd", e.to_string()))?
            .join(destination)
    };

    let parent = dest.parent().ok_or_else(|| {
        FsxError::new(
            "fsx.dest_parent",
            format!("destination has no parent: {}", dest.display()),
        )
    })?;
    fs::create_dir_all(parent).map_err(|e| {
        FsxError::new(
            "fsx.stage_create",
            format!("cannot create dest parent {}: {e}", parent.display()),
        )
    })?;

    let base = dest
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project");
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let stage_name = format!(".foundry-stage-{}-{}-{}", base, std::process::id(), nanos);
    let stage_path = parent.join(stage_name);

    fs::create_dir_all(&stage_path).map_err(|e| {
        FsxError::new(
            "fsx.stage_create",
            format!("cannot create stage {}: {e}", stage_path.display()),
        )
    })?;

    for (rel, file) in map {
        assert_path_jailed(rel).map_err(|e| FsxError::new(e.code, e.message))?;
        let target = stage_path.join(rel);
        // Ensure path stays under stage after join (reject absolute components already).
        if !target.starts_with(&stage_path) {
            return Err(FsxError::new(
                "plan.path_jail",
                format!("materialize path escapes stage: {rel:?}"),
            ));
        }
        if let Some(dir) = target.parent() {
            fs::create_dir_all(dir).map_err(|e| {
                FsxError::new(
                    "fsx.stage_write",
                    format!("cannot create {} for {rel}: {e}", dir.display()),
                )
            })?;
        }
        match file.mode {
            FileMode::Directory => {
                fs::create_dir_all(&target).map_err(|e| {
                    FsxError::new(
                        "fsx.stage_write",
                        format!("cannot mkdir {}: {e}", target.display()),
                    )
                })?;
            }
            FileMode::File | FileMode::Executable => {
                fs::write(&target, &file.bytes).map_err(|e| {
                    FsxError::new(
                        "fsx.stage_write",
                        format!("cannot write {}: {e}", target.display()),
                    )
                })?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mode = if file.mode == FileMode::Executable {
                        0o755
                    } else {
                        0o644
                    };
                    let perms = fs::Permissions::from_mode(mode);
                    fs::set_permissions(&target, perms).map_err(|e| {
                        FsxError::new(
                            "fsx.stage_write",
                            format!("chmod {}: {e}", target.display()),
                        )
                    })?;
                }
            }
        }
    }

    Ok(StageHandle {
        stage_path,
        destination: dest,
    })
}

/// Destination admissibility decision (REQ-051).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Admissibility {
    /// Path does not exist.
    Missing,
    /// Empty directory (zero `read_dir` children).
    EmptyDir,
    /// Must refuse place.
    Refuse {
        /// Stable reason token.
        reason: &'static str,
    },
}

/// Classify destination for exclusive place (REQ-051 emptiness predicate).
pub fn classify_destination(path: &Path) -> Result<Admissibility, FsxError> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Admissibility::Missing),
        Err(e) => Err(FsxError::new(
            "fsx.dest_stat",
            format!("cannot stat {}: {e}", path.display()),
        )),
        Ok(meta) => {
            if meta.file_type().is_symlink() {
                return Ok(Admissibility::Refuse { reason: "symlink" });
            }
            if meta.is_file() {
                return Ok(Admissibility::Refuse {
                    reason: "file_at_path",
                });
            }
            if meta.is_dir() {
                let mut rd = fs::read_dir(path).map_err(|e| {
                    FsxError::new(
                        "fsx.dest_stat",
                        format!("cannot read_dir {}: {e}", path.display()),
                    )
                })?;
                if rd.next().is_none() {
                    Ok(Admissibility::EmptyDir)
                } else {
                    Ok(Admissibility::Refuse {
                        reason: "non_empty",
                    })
                }
            } else {
                Ok(Admissibility::Refuse {
                    reason: "unsupported_file_type",
                })
            }
        }
    }
}

/// Exclusive place: rename stage → destination when admissible (REQ-051).
///
/// Prefer atomic rename same-FS. On EXDEV, fail-closed (no silent copy+swap).
/// On success, stage is gone (renamed away). On refuse, stage is retained.
pub fn exclusive_place(stage: &StageHandle) -> Result<(), FsxError> {
    let dest = &stage.destination;
    let stage_path = &stage.stage_path;

    match classify_destination(dest)? {
        Admissibility::Missing => {
            // parent must exist (created at stage time)
            try_rename(stage_path, dest)
        }
        Admissibility::EmptyDir => {
            // Remove empty dest so rename can take its place.
            fs::remove_dir(dest).map_err(|e| {
                FsxError::new(
                    "fsx.place",
                    format!("cannot remove empty dest {}: {e}", dest.display()),
                )
            })?;
            try_rename(stage_path, dest)
        }
        Admissibility::Refuse { reason } => Err(FsxError::new(
            "fsx.refuse_non_empty",
            format!(
                "destination {} is not admissible ({reason}); refuse exclusive place (REQ-051)",
                dest.display()
            ),
        )),
    }
}

fn try_rename(from: &Path, to: &Path) -> Result<(), FsxError> {
    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(e) if is_exdev(&e) => Err(FsxError::new(
            "fsx.cross_device",
            format!(
                "cross-device rename from {} to {} failed (EXDEV); fail-closed — no copy+swap in v1 Default: {e}",
                from.display(),
                to.display()
            ),
        )),
        Err(e) => Err(FsxError::new(
            "fsx.place",
            format!("rename {} → {}: {e}", from.display(), to.display()),
        )),
    }
}

fn is_exdev(err: &io::Error) -> bool {
    // Unix EXDEV = 18
    err.raw_os_error() == Some(18)
}

/// Remove stage directory tree (success cleanup or explicit).
pub fn clean_stage(stage_path: &Path) -> Result<(), FsxError> {
    if stage_path.exists() {
        fs::remove_dir_all(stage_path).map_err(|e| {
            FsxError::new(
                "fsx.clean_stage",
                format!("cannot remove stage {}: {e}", stage_path.display()),
            )
        })?;
    }
    Ok(())
}

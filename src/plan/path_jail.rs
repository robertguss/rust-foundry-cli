//! Path jail for planned paths (REQ-053) — pure, no FS.

use crate::plan::error::ConstructError;

/// Validate a planned relative path stays inside the project/stage root.
///
/// Hard-fails absolute paths and `..` escapes. This is a pure string/component
/// check on the path text; it cannot see the filesystem. `fsx::stage_render_map`
/// re-validates each joined target stays under the stage root as it writes
/// (defense in depth), but since every stage directory is created fresh by
/// Foundry immediately before materializing files, there are no pre-existing
/// symlinks inside it for a component to escape through.
pub fn assert_path_jailed(path: &str) -> Result<(), ConstructError> {
    if path.is_empty() {
        return Err(ConstructError::new(
            "plan.path_jail",
            "planned path must be non-empty",
        ));
    }
    if path.starts_with('/') || path.starts_with('\\') {
        return Err(ConstructError::new(
            "plan.path_jail",
            format!("absolute planned path rejected: {path:?}"),
        ));
    }
    // Windows-style drive / UNC (product is macOS+Linux only, still reject).
    if path.len() >= 2 && path.as_bytes()[1] == b':' {
        return Err(ConstructError::new(
            "plan.path_jail",
            format!("drive-absolute planned path rejected: {path:?}"),
        ));
    }

    let mut depth: i32 = 0;
    for component in path.split(['/', '\\']) {
        if component.is_empty() || component == "." {
            continue;
        }
        if component == ".." {
            depth -= 1;
            if depth < 0 {
                return Err(ConstructError::new(
                    "plan.path_jail",
                    format!("path escapes project root via '..': {path:?}"),
                ));
            }
            continue;
        }
        depth += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_ok() {
        assert_path_jailed("src/main.rs").unwrap();
        assert_path_jailed("AGENTS.md").unwrap();
        assert_path_jailed("a/b/../c").unwrap(); // stays under root
    }

    #[test]
    fn absolute_rejected() {
        let err = assert_path_jailed("/etc/passwd").unwrap_err();
        assert_eq!(err.code, "plan.path_jail");
    }

    #[test]
    fn parent_escape_rejected() {
        let err = assert_path_jailed("../outside").unwrap_err();
        assert_eq!(err.code, "plan.path_jail");
        let err = assert_path_jailed("foo/../../etc").unwrap_err();
        assert_eq!(err.code, "plan.path_jail");
    }
}

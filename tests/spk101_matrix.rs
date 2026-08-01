//! MS-006.1 / SPK-101: table-driven emptiness / place / lifecycle cases.

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use foundry::catalog::stub_catalog;
use foundry::fsx::{self, Admissibility};
use foundry::generate::generate_with;
use foundry::spec::{CliOverrides, normalize_effective_inputs, parse_spec_str};
use foundry::verify::{AlwaysPass, ForcedFail};

fn sandbox(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "foundry-spk101-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn inputs(dest: &Path) -> foundry::spec::EffectiveInputs {
    let body = format!(
        r#"
schema = 1
name = "spk101"
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

#[derive(Clone, Copy)]
enum DestSetup {
    Missing,
    EmptyDir,
    NonEmptyGit,
    FileAtPath,
    Symlink,
}

fn prepare(root: &Path, setup: DestSetup) -> PathBuf {
    let dest = root.join("dest");
    match setup {
        DestSetup::Missing => {}
        DestSetup::EmptyDir => {
            fs::create_dir_all(&dest).unwrap();
        }
        DestSetup::NonEmptyGit => {
            fs::create_dir_all(&dest).unwrap();
            fs::create_dir_all(dest.join(".git")).unwrap();
        }
        DestSetup::FileAtPath => {
            fs::write(&dest, b"file").unwrap();
        }
        DestSetup::Symlink => {
            let target = root.join("link-target");
            fs::create_dir_all(&target).unwrap();
            symlink(&target, &dest).unwrap();
        }
    }
    dest
}

#[test]
fn spk101_table_driven_admissibility_and_generate() {
    let cases: &[(&str, DestSetup, bool)] = &[
        ("missing", DestSetup::Missing, true),
        ("empty", DestSetup::EmptyDir, true),
        ("nonempty_git", DestSetup::NonEmptyGit, false),
        ("file_at_path", DestSetup::FileAtPath, false),
        ("symlink", DestSetup::Symlink, false),
    ];

    for (name, setup, expect_ok) in cases {
        let root = sandbox(name);
        let dest = prepare(&root, *setup);
        match setup {
            DestSetup::Missing => {
                assert_eq!(
                    fsx::classify_destination(&dest).unwrap(),
                    Admissibility::Missing
                );
            }
            DestSetup::EmptyDir => {
                assert_eq!(
                    fsx::classify_destination(&dest).unwrap(),
                    Admissibility::EmptyDir
                );
            }
            DestSetup::NonEmptyGit => {
                assert!(matches!(
                    fsx::classify_destination(&dest).unwrap(),
                    Admissibility::Refuse {
                        reason: "non_empty"
                    }
                ));
            }
            DestSetup::FileAtPath => {
                assert!(matches!(
                    fsx::classify_destination(&dest).unwrap(),
                    Admissibility::Refuse {
                        reason: "file_at_path"
                    }
                ));
            }
            DestSetup::Symlink => {
                assert!(matches!(
                    fsx::classify_destination(&dest).unwrap(),
                    Admissibility::Refuse { reason: "symlink" }
                ));
            }
        }

        let result = generate_with(&inputs(&dest), &stub_catalog(), &AlwaysPass);
        if *expect_ok {
            let ok = result.expect("generate should succeed");
            assert!(ok.destination.join("Cargo.toml").is_file());
            // success clean: no stage leftover
            let parent = ok.destination.parent().unwrap();
            for ent in fs::read_dir(parent).unwrap() {
                let n = ent.unwrap().file_name().to_string_lossy().into_owned();
                assert!(!n.starts_with(".foundry-stage-"), "leftover stage {n}");
            }
        } else {
            let err = result.expect_err("generate should refuse");
            assert_eq!(err.code, "fsx.refuse_non_empty");
            assert!(err.stage_path.is_some());
            assert!(err.stage_path.as_ref().unwrap().exists());
            // dest not replaced with project tree for refuse cases
            match setup {
                DestSetup::NonEmptyGit => assert!(dest.join(".git").exists()),
                DestSetup::FileAtPath => assert!(dest.is_file()),
                DestSetup::Symlink => {
                    assert!(dest.symlink_metadata().unwrap().file_type().is_symlink())
                }
                _ => {}
            }
            let _ = fs::remove_dir_all(err.stage_path.as_ref().unwrap());
        }
        let _ = fs::remove_dir_all(&root);
    }
}

#[test]
fn spk101_fail_verify_retain_stage_path() {
    let root = sandbox("verify-fail");
    let dest = root.join("d");
    let err = generate_with(
        &inputs(&dest),
        &stub_catalog(),
        &ForcedFail {
            message: "spk101 fail".into(),
        },
    )
    .unwrap_err();
    assert_eq!(err.code, "generate.verify_failed");
    assert!(!dest.exists());
    assert!(err.stage_path.as_ref().unwrap().exists());
    let _ = fs::remove_dir_all(err.stage_path.as_ref().unwrap());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn spk101_same_fs_success() {
    // Default tempfile is same FS for stage parent and dest.
    let root = sandbox("same-fs");
    let dest = root.join("ok");
    let r = generate_with(&inputs(&dest), &stub_catalog(), &AlwaysPass).unwrap();
    assert!(r.destination.join("src/main.rs").is_file());
    let _ = fs::remove_dir_all(&root);
}

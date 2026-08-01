//! MS-003.8: write-free plan/validate sandboxes + override equality (REQ-034/040).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use foundry::catalog::stub_catalog;
use foundry::plan::construct;
use foundry::spec::{CliOverrides, VerifyMode, normalize_effective_inputs, parse_spec_str};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn foundry() -> Command {
    Command::new(env!("CARGO_BIN_EXE_foundry"))
}

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

fn tree_fingerprint(root: &Path) -> Vec<(String, u64, String)> {
    let mut entries = Vec::new();
    if !root.exists() {
        return entries;
    }
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, u64, String)>) {
        for ent in fs::read_dir(dir).unwrap() {
            let ent = ent.unwrap();
            let path = ent.path();
            let rel = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let meta = ent.metadata().unwrap();
            if meta.is_dir() {
                out.push((rel.clone(), 0, "dir".into()));
                walk(base, &path, out);
            } else {
                let bytes = fs::read(&path).unwrap();
                let digest = foundry::plan::content_sha256(&bytes);
                out.push((rel, meta.len(), digest));
            }
        }
    }
    walk(root, root, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

#[test]
fn plan_cli_write_free_destination_sandbox() {
    let dir = tempfile_dir("writefree");
    let dest = dir.join("project-dest");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("preexisting.txt"), b"do-not-touch").unwrap();
    let before = tree_fingerprint(&dest);

    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = foundry()
        .args([
            "plan",
            "--spec",
            spec.to_str().unwrap(),
            "--dest",
            dest.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("plan_sha256:"));
    // Plan records overridden dest in report.
    assert!(
        stdout.contains(dest.to_str().unwrap()) || stdout.contains("project-dest"),
        "expected dest in plan text, got:\n{stdout}"
    );

    let after = tree_fingerprint(&dest);
    assert_eq!(before, after, "destination tree must be unchanged by plan");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn validate_cli_write_free_destination_sandbox() {
    let dir = tempfile_dir("val-writefree");
    let dest = dir.join("dest");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("x"), b"1").unwrap();
    let before = tree_fingerprint(&dest);
    let spec = repo_root().join("examples/minimal-cli.toml");
    let output = foundry()
        .args([
            "validate",
            "--spec",
            spec.to_str().unwrap(),
            "--dest",
            dest.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(before, tree_fingerprint(&dest));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn override_dest_wins_in_plan() {
    let body = r#"
schema = 1
name = "from-toml"
archetype = "cli"
destination = "./toml-dest"
profiles = []
verify = "none"
"#;
    let spec = parse_spec_str(body, "<t>").unwrap();
    let inputs = normalize_effective_inputs(
        spec,
        CliOverrides {
            dest: Some("./cli-dest".into()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(inputs.destination, "./cli-dest");
    let plan = construct(&inputs, &stub_catalog()).unwrap();
    assert_eq!(plan.normalized_spec.destination, "./cli-dest");
    // TOML verify preserved when not overridden.
    assert_eq!(plan.verify, VerifyMode::None);
}

#[test]
fn override_name_and_verify_win() {
    let body = r#"
schema = 1
name = "from-toml"
archetype = "cli"
destination = "./d"
profiles = []
verify = "none"
"#;
    let spec = parse_spec_str(body, "<t>").unwrap();
    let inputs = normalize_effective_inputs(
        spec,
        CliOverrides {
            name: Some("from-cli".into()),
            verify: Some(VerifyMode::Strict),
            ..Default::default()
        },
    )
    .unwrap();
    let plan = construct(&inputs, &stub_catalog()).unwrap();
    assert_eq!(plan.normalized_spec.name, "from-cli");
    assert_eq!(plan.verify, VerifyMode::Strict);
}

#[test]
fn same_effective_inputs_equal_plan_sha256() {
    let body = r#"
schema = 1
name = "a"
archetype = "cli"
destination = "./a"
profiles = []
"#;
    let s1 = parse_spec_str(body, "<t>").unwrap();
    let s2 = parse_spec_str(body, "<t>").unwrap();
    let o = CliOverrides {
        name: Some("shared".into()),
        dest: Some("./shared".into()),
        verify: Some(VerifyMode::Default),
    };
    let i1 = normalize_effective_inputs(s1, o.clone()).unwrap();
    let i2 = normalize_effective_inputs(s2, o).unwrap();
    assert_eq!(i1, i2);
    let p1 = construct(&i1, &stub_catalog()).unwrap();
    let p2 = construct(&i2, &stub_catalog()).unwrap();
    assert_eq!(p1.plan_sha256, p2.plan_sha256);
}

#[test]
fn cli_plan_override_equality_name() {
    let dir = tempfile_dir("ov-eq");
    let spec_path = dir.join("spec.toml");
    fs::write(
        &spec_path,
        r#"
schema = 1
name = "toml-name"
archetype = "cli"
destination = "./toml-dest"
profiles = []
"#,
    )
    .unwrap();

    let out_a = dir.join("a.json");
    let out_b = dir.join("b.json");
    for (out, name) in [(&out_a, "same-name"), (&out_b, "same-name")] {
        let output = foundry()
            .args([
                "plan",
                "--spec",
                spec_path.to_str().unwrap(),
                "--name",
                name,
                "--format",
                "json",
                "--out",
                out.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let va: serde_json::Value = serde_json::from_str(&fs::read_to_string(&out_a).unwrap()).unwrap();
    let vb: serde_json::Value = serde_json::from_str(&fs::read_to_string(&out_b).unwrap()).unwrap();
    assert_eq!(va["plan_sha256"], vb["plan_sha256"]);
    assert_eq!(va["normalized_spec"]["name"], "same-name");
    let _ = fs::remove_dir_all(&dir);
}

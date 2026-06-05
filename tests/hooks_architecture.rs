//! Integration tests for `cairn hook architecture-decision`.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn git_init(root: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("git")
        .current_dir(root)
        .args(["init", "--quiet"])
        .output()?;
    Command::new("git")
        .current_dir(root)
        .args(["config", "user.email", "test@example.com"])
        .output()?;
    Command::new("git")
        .current_dir(root)
        .args(["config", "user.name", "Test"])
        .output()?;
    // Throwaway fixtures must commit without signing; some environments enforce
    // commit signing that fails for ad-hoc repos and would leave HEAD empty,
    // breaking the `git show HEAD:cairn.blueprint` the architecture gate reads.
    Command::new("git")
        .current_dir(root)
        .args(["config", "commit.gpgsign", "false"])
        .output()?;
    Ok(())
}

fn git_commit(root: &std::path::Path, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("git")
        .current_dir(root)
        .args(["add", "."])
        .output()?;
    Command::new("git")
        .current_dir(root)
        .args(["commit", "-m", msg, "--quiet"])
        .output()?;
    Ok(())
}

#[test]
fn test_architecture_gate_fires_on_module_add_without_decision()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("arch-gate-fires")?;
    git_init(&root)?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {}
"#,
    )?;
    git_commit(&root, "initial")?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "architecture-decision"])
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("CH001"));
    assert!(stdout.contains("app.one"));
    assert!(stdout.contains("added"));

    Ok(())
}

#[test]
fn test_architecture_gate_passes_on_module_add_with_decision()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("arch-gate-passes")?;
    git_init(&root)?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {}
"#,
    )?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::write(
        root.join("meta/decisions/add-one.md"),
        "---\naffects: app.one\n---\n\n# Decision\n",
    )?;
    git_commit(&root, "initial")?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "architecture-decision"])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Decision: pass"));

    Ok(())
}

#[test]
fn test_architecture_gate_passes_with_escape_hatch() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("arch-gate-escape")?;
    git_init(&root)?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {}
"#,
    )?;
    git_commit(&root, "initial")?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"# decision: trivial
System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "architecture-decision"])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Decision: pass"));

    Ok(())
}

#[test]
fn test_architecture_gate_ignores_reorder_and_casing() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("arch-gate-ignore")?;
    git_init(&root)?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src/one"
    }
    Module Two "two" id "app.two" {
        path "./src/two"
    }
}
"#,
    )?;
    git_commit(&root, "initial")?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Two "two" id "app.two" {
        path "./src/two"
    }
    Module One "one" id "app.one" {
        path "./src/one"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "architecture-decision"])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Decision: pass"));

    Ok(())
}

#[test]
fn test_architecture_gate_fires_on_reassign_across_containers()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("arch-gate-reassign")?;
    git_init(&root)?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Container A "a" id "app.a" {
        Module One "one" id "app.one" {
            path "./src/one"
        }
    }
}
"#,
    )?;
    git_commit(&root, "initial")?;

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Container A "a" id "app.a" {}
    Container B "b" id "app.b" {
        Module One "one" id "app.one" {
            path "./src/one"
        }
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "architecture-decision"])
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("CH001"));
    assert!(stdout.contains("reassigned"));

    Ok(())
}

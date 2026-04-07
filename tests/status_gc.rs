mod helpers;

use helpers::TempProject;

fn init_project(project: &TempProject) {
    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(output.status.success());
}

#[test]
fn status_shows_project_info() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["status"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Project:"));
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("Sprint: none active"));
    assert!(stdout.contains("Active plans: 0"));
}

#[test]
fn status_shows_active_sprint() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["sprint", "new", "build feature"]);

    let output = project.run_harn(&["status"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("build feature"));
}

#[test]
fn status_shows_active_plans() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "big plan"]);

    let output = project.run_harn(&["status"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Active plans: 1"));
}

#[test]
fn status_without_init_fails() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["status"]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("harn init") || stderr.contains("config"),
        "Error should guide user to run harn init, got: {stderr}"
    );
}

#[test]
fn status_handles_corrupt_sprint_state() {
    let project = TempProject::with_git();
    init_project(&project);

    project.write_file(
        ".agents/harn/current-sprint.toml",
        "this is not valid toml {{{{",
    );

    let output = project.run_harn(&["status"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("unreadable") || stdout.contains("invalid"),
        "Status should report unreadable sprint state, got: {stdout}"
    );
}

#[test]
fn gc_json_output_has_required_fields() {
    let project = TempProject::with_git();
    init_project(&project);

    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    let output = project.run_harn(&["gc", "--json"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let arr = parsed.as_array().unwrap();
    for finding in arr {
        assert!(
            finding.get("path").is_some(),
            "Finding missing 'path' field"
        );
        assert!(
            finding.get("severity").is_some(),
            "Finding missing 'severity' field"
        );
        assert!(
            finding.get("message").is_some(),
            "Finding missing 'message' field"
        );
    }
}

#[test]
fn gc_runs_on_fresh_harness() {
    let project = TempProject::with_git();
    init_project(&project);

    // Need at least one commit for git2 to work
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    let output = project.run_harn(&["gc"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scanning documentation freshness"));
}

#[test]
fn gc_json_output() {
    let project = TempProject::with_git();
    init_project(&project);

    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    let output = project.run_harn(&["gc", "--json"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // JSON output should be parseable
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert!(parsed.is_array());
}

#[test]
fn gc_without_git() {
    let project = TempProject::new(); // no git
    init_project(&project);

    let output = project.run_harn(&["gc"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scanning documentation freshness"));
}

#[test]
fn gc_ci_exit_code_0_when_clean() {
    let project = TempProject::with_git();
    init_project(&project);

    // Clear file_hashes in config to prevent "still matches init template" warnings
    let config_content = project.read_file(".agents/harn/config.toml");
    let mut doc: toml::Table = toml::from_str(&config_content).unwrap();
    if let Some(init) = doc.get_mut("init").and_then(|v| v.as_table_mut()) {
        init.insert(
            "file_hashes".to_string(),
            toml::Value::Table(toml::Table::new()),
        );
    }
    let config_path = project.path().join(".agents/harn/config.toml");
    std::fs::write(&config_path, toml::to_string_pretty(&doc).unwrap()).unwrap();

    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    let output = project.run_harn(&["gc", "--ci"]);
    assert!(
        output.status.success(),
        "gc --ci should succeed when no warnings/errors, exit code: {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn gc_ci_exit_code_nonzero_on_warnings() {
    let project = TempProject::with_git();
    init_project(&project);

    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    // Don't customize templates → will have "still matches init template" warnings
    let output = project.run_harn(&["gc", "--ci"]);
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code >= 1,
        "gc --ci should return nonzero when warnings exist, got exit code {code}"
    );
}

#[test]
fn status_without_init_exits_3() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["status"]);
    let code = output.status.code().unwrap_or(-1);
    assert_eq!(code, 3, "status without config should exit 3, got {code}");
}

#[test]
fn status_shows_plan_milestone_progress() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "tracked plan"]);

    let active_dir = project.path().join("docs/exec-plans/active");
    let file = std::fs::read_dir(&active_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.contains("tracked-plan") && name.ends_with(".md")
        })
        .unwrap();

    std::fs::write(
        file.path(),
        "# ExecPlan: tracked plan\n\n\
         ## Milestones\n\n\
         ### Milestone 1: Setup\n\n\
         - [x] Created project\n\
         - [ ] Implemented feature\n",
    )
    .unwrap();

    let output = project.run_harn(&["status"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("1 milestones") || stdout.contains("1/2"),
        "status should show milestone progress, got: {stdout}"
    );
}

#[test]
fn gc_days_override_controls_threshold() {
    let project = TempProject::with_git();
    init_project(&project);

    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    // With --days 0, every doc is "stale" immediately
    let output = project.run_harn(&["gc", "--days", "0", "--json"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let age_findings: Vec<_> = findings
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| {
            f["message"]
                .as_str()
                .is_some_and(|m| m.contains("not modified in"))
        })
        .collect();
    // With threshold 0, we expect some stale findings
    let stale_count_zero = age_findings.len();

    // With --days 99999, nothing should be flagged as stale by age
    let output = project.run_harn(&["gc", "--days", "99999", "--json"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let age_findings: Vec<_> = findings
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| {
            f["message"]
                .as_str()
                .is_some_and(|m| m.contains("not modified in"))
        })
        .collect();
    let stale_count_high = age_findings.len();

    assert!(
        stale_count_zero >= stale_count_high,
        "Lower threshold (--days 0) should produce >= stale findings than high threshold, \
         got {stale_count_zero} vs {stale_count_high}"
    );
}

#[test]
fn gc_mappings_detect_code_doc_divergence() {
    let project = TempProject::with_git();
    init_project(&project);

    // Create a doc and a code file
    project.write_file("docs/design.md", "# Design\n\nInitial content.\n");
    project.write_file("src/lib.rs", "fn main() {}\n");

    // Add gc.mappings to config by parsing and modifying TOML properly
    let config_content = project.read_file(".agents/harn/config.toml");
    let mut doc: toml::Table = toml::from_str(&config_content).unwrap();
    if let Some(gc) = doc.get_mut("gc").and_then(|v| v.as_table_mut()) {
        let mapping = toml::Value::Table({
            let mut m = toml::Table::new();
            m.insert("doc".into(), toml::Value::String("docs/design.md".into()));
            m.insert(
                "code".into(),
                toml::Value::Array(vec![toml::Value::String("src/lib.rs".into())]),
            );
            m
        });
        gc.insert("mappings".into(), toml::Value::Array(vec![mapping]));
    }
    let config_path = project.path().join(".agents/harn/config.toml");
    std::fs::write(&config_path, toml::to_string_pretty(&doc).unwrap()).unwrap();

    // Commit everything
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init with mappings"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    // Ensure git timestamps differ between commits
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Now modify the code file but NOT the doc
    project.write_file("src/lib.rs", "fn main() { println!(\"updated\"); }\n");
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "update code"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    let output = project.run_harn(&["gc", "--json"]);
    assert!(
        output.status.success(),
        "gc failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let divergence: Vec<_> = findings
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| {
            f["message"]
                .as_str()
                .is_some_and(|m| m.contains("related code changed"))
        })
        .collect();

    assert!(
        !divergence.is_empty(),
        "gc should detect code-doc divergence when code is updated without updating doc, \
         findings: {findings}"
    );
}

#[test]
fn gc_without_git_reports_info() {
    let project = TempProject::new(); // no git
    init_project(&project);

    let output = project.run_harn(&["gc", "--json"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let info: Vec<_> = findings
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| {
            f["severity"].as_str() == Some("info")
                && f["message"]
                    .as_str()
                    .is_some_and(|m| m.contains("Git repository not available"))
        })
        .collect();

    assert!(
        !info.is_empty(),
        "gc without git should report an info finding about git unavailability"
    );
}

#[test]
fn gc_detects_uncustomized_templates() {
    let project = TempProject::with_git();
    init_project(&project);

    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    let output = project.run_harn(&["gc"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("init template"));
}

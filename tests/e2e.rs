mod helpers;

use helpers::TempProject;

/// Full lifecycle: init → plan → sprint → done → status → gc
#[test]
fn full_lifecycle() {
    let project = TempProject::with_git();

    // Init
    let output = project.run_harn(&["init", "--tools", "codex,claude-code", "--stack", "rust"]);
    assert!(output.status.success(), "init failed");

    // Check passes
    let output = project.run_harn(&["check"]);
    assert!(output.status.success(), "check failed after init");

    // Create plan
    let output = project.run_harn(&["plan", "new", "user authentication"]);
    assert!(output.status.success(), "plan new failed");

    // Create sprint linked to plan
    let output = project.run_harn(&[
        "sprint",
        "new",
        "implement login page",
        "--plan",
        "user-authentication",
    ]);
    assert!(output.status.success(), "sprint new failed");

    // Status shows sprint and plan
    let output = project.run_harn(&["status"]);
    assert!(output.status.success(), "status failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("implement login page"));
    assert!(stdout.contains("Active plans: 1"));

    // Sprint done
    let output = project.run_harn(&["sprint", "done"]);
    assert!(output.status.success(), "sprint done failed");

    // Plan complete
    let output = project.run_harn(&["plan", "complete", "user-authentication"]);
    assert!(output.status.success(), "plan complete failed");

    // Status shows clean state
    let output = project.run_harn(&["status"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("none active") || !stdout.contains("implement login"));
    assert!(stdout.contains("Active plans: 0"));

    // Commit everything for gc
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(project.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "full lifecycle"])
        .current_dir(project.path())
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@test.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@test.com")
        .output()
        .unwrap();

    // GC runs
    let output = project.run_harn(&["gc"]);
    assert!(output.status.success(), "gc failed");
}

/// Init → check → modify AGENTS.md → check detects customization
#[test]
fn init_check_modify_check() {
    let project = TempProject::with_git();

    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    // Check warns about uncustomized
    let output = project.run_harn(&["check", "--ci"]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "should warn about uncustomized"
    );

    // Customize AGENTS.md
    let agents = project.read_file("AGENTS.md");
    let customized = agents.replace(
        "TODO: Describe your project in 1-2 sentences.",
        "A real-time chat application with WebSocket support.",
    );
    std::fs::write(project.path().join("AGENTS.md"), customized).unwrap();

    // Customize ARCHITECTURE.md
    let arch = project.read_file("ARCHITECTURE.md");
    let customized_arch = arch.replace(
        "<!-- TODO: Describe what this system does at a high level. 2-3 sentences. -->",
        "A real-time messaging system built with Rust and WebSocket.",
    );
    std::fs::write(project.path().join("ARCHITECTURE.md"), customized_arch).unwrap();

    // Customize criteria
    let criteria = project.read_file("docs/evaluation/criteria.md");
    let customized_criteria = criteria.replace(
        "These criteria define what \"good\" means for this project.",
        "These criteria define quality standards for the chat application.",
    );
    std::fs::write(
        project.path().join("docs/evaluation/criteria.md"),
        customized_criteria,
    )
    .unwrap();

    // Now check should have fewer warnings (some files customized)
    let output = project.run_harn(&["check"]);
    assert!(output.status.success());
}

/// Upgrade preserves user modifications
#[test]
fn upgrade_preserves_customizations() {
    let project = TempProject::with_git();

    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    // Customize AGENTS.md
    let custom = "# My Fully Custom AGENTS.md\n\nThis is completely rewritten.\n";
    std::fs::write(project.path().join("AGENTS.md"), custom).unwrap();

    // Upgrade
    let output = project.run_harn(&["upgrade"]);
    assert!(output.status.success());

    // Original custom content preserved
    let content = project.read_file("AGENTS.md");
    assert_eq!(content, custom);

    // Sidecar created
    assert!(project.file_exists("AGENTS.md.harn-upgrade"));
}

/// Assess command works on fresh and initialized projects
#[test]
fn assess_on_initialized_project() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex,claude-code", "--stack", "rust"]);

    let output = project.run_harn(&["assess"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Harness Maturity Assessment"));
    assert!(stdout.contains("Level 1"));
    assert!(stdout.contains("Level 2"));
    assert!(stdout.contains("Repository Knowledge"));
}

#[test]
fn assess_json_output() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let output = project.run_harn(&["assess", "--json"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();

    assert!(parsed.get("project").is_some(), "missing 'project' field");
    assert!(parsed.get("level").is_some(), "missing 'level' field");
    assert!(parsed.get("level1_pct").is_some(), "missing 'level1_pct'");
    assert!(parsed.get("level2_pct").is_some(), "missing 'level2_pct'");

    let checks = parsed.get("checks").and_then(|v| v.as_array()).unwrap();
    assert!(!checks.is_empty());
    for item in checks {
        assert!(item.get("category").is_some());
        assert!(item.get("status").is_some());
        assert!(item.get("level").is_some());
    }
}

#[test]
fn assess_on_empty_project() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["assess"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Level 0") || stdout.contains("MUST"));
}

/// Multiple plans and sprints
#[test]
fn multiple_plans_and_sprints() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    // Create two plans
    project.run_harn(&["plan", "new", "auth module"]);
    project.run_harn(&["plan", "new", "api layer"]);

    // List shows both
    let output = project.run_harn(&["plan", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("auth-module"));
    assert!(stdout.contains("api-layer"));

    // Sprint on first plan
    project.run_harn(&["sprint", "new", "login endpoint", "--plan", "auth-module"]);

    let output = project.run_harn(&["sprint", "status"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("login endpoint"));

    // Complete sprint
    project.run_harn(&["sprint", "done"]);

    // New standalone sprint (no plan)
    project.run_harn(&["sprint", "new", "quick bugfix"]);
    project.run_harn(&["sprint", "done"]);

    // Complete plans
    project.run_harn(&["plan", "complete", "auth-module"]);
    project.run_harn(&["plan", "complete", "api-layer"]);

    let output = project.run_harn(&["plan", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Completed"));
}

#[test]
fn verbose_and_quiet_conflict() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let output = project.run_harn(&["--verbose", "--quiet", "status"]);
    assert!(
        !output.status.success(),
        "--verbose and --quiet should not be used together"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--verbose") && stderr.contains("--quiet"));
}

#[test]
fn no_color_flag_accepted() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let output = project.run_harn(&["--no-color", "status"]);
    assert!(output.status.success());
}

#[test]
fn config_error_exits_3() {
    let project = TempProject::with_git();
    // No init → no config

    let output = project.run_harn(&["check"]);
    let code = output.status.code().unwrap_or(-1);
    assert_eq!(code, 3, "missing config should exit 3, got {code}");
}

#[test]
fn assess_json_has_proper_schema() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let output = project.run_harn(&["assess", "--json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();

    let level = parsed.get("level").and_then(|v| v.as_u64()).unwrap();
    assert!(level <= 3, "level should be 0-3, got {level}");

    let l1 = parsed.get("level1_pct").and_then(|v| v.as_u64()).unwrap();
    let l2 = parsed.get("level2_pct").and_then(|v| v.as_u64()).unwrap();
    assert!(l1 <= 100, "level1_pct should be 0-100, got {l1}");
    assert!(l2 <= 100, "level2_pct should be 0-100, got {l2}");
}

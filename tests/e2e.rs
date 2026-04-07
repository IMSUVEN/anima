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
    let arr = parsed.as_array().unwrap();
    assert!(!arr.is_empty());

    for item in arr {
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

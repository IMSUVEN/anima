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

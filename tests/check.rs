mod helpers;

use helpers::TempProject;

fn init_project(project: &TempProject) {
    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(
        output.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn check_passes_on_valid_harness() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["check"]);
    assert!(output.status.success());
}

#[test]
fn check_fails_on_missing_agents_md() {
    let project = TempProject::with_git();
    init_project(&project);

    std::fs::remove_file(project.path().join("AGENTS.md")).unwrap();

    let output = project.run_harn(&["check"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AGENTS.md") && stdout.contains("does not exist"));
}

#[test]
fn check_warns_on_uncustomized_template() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["check"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not customized") || stdout.contains("init template"));
}

#[test]
fn check_fix_recreates_missing_dirs() {
    let project = TempProject::with_git();
    init_project(&project);

    std::fs::remove_dir_all(project.path().join("docs/exec-plans/active")).unwrap();
    assert!(!project.file_exists("docs/exec-plans/active"));

    let output = project.run_harn(&["check", "--fix"]);
    assert!(output.status.success());
    assert!(project.file_exists("docs/exec-plans/active"));
}

#[test]
fn check_ci_exit_code_1_on_warnings() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["check", "--ci"]);
    // Fresh harness has uncustomized template warnings → exit 1
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn check_ci_exit_code_2_on_errors() {
    let project = TempProject::with_git();
    init_project(&project);

    std::fs::remove_file(project.path().join("AGENTS.md")).unwrap();

    let output = project.run_harn(&["check", "--ci"]);
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn check_error_gives_nonzero_exit_on_missing_file() {
    let project = TempProject::with_git();
    init_project(&project);

    std::fs::remove_file(project.path().join("AGENTS.md")).unwrap();

    let output = project.run_harn(&["check"]);
    // Errors should produce exit code 2
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn check_without_init_fails() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["check"]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("harn init") || stderr.contains("config"),
        "Error should guide user to run harn init, got: {stderr}"
    );
}

#[test]
fn check_detects_broken_cross_references() {
    let project = TempProject::with_git();
    init_project(&project);

    // Add a broken link to AGENTS.md
    let agents = project.read_file("AGENTS.md");
    let modified = agents.replace(
        "| Workflow templates |",
        "| Broken link | [nonexistent.md](docs/nonexistent.md) |\n| Workflow templates |",
    );
    std::fs::write(project.path().join("AGENTS.md"), modified).unwrap();

    let output = project.run_harn(&["check"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nonexistent.md") && stdout.contains("does not exist"));
}

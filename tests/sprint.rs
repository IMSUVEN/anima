mod helpers;

use helpers::TempProject;

fn init_project(project: &TempProject) {
    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(output.status.success());
}

#[test]
fn sprint_new_creates_contract_and_state() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["sprint", "new", "implement login"]);
    assert!(output.status.success());

    assert!(project.file_exists(".agents/harn/current-sprint.toml"));

    let active_dir = project.path().join("docs/exec-plans/active");
    let sprint_files: Vec<_> = std::fs::read_dir(&active_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("sprint-"))
        .collect();
    assert_eq!(sprint_files.len(), 1);
}

#[test]
fn sprint_new_fails_when_already_active() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["sprint", "new", "first sprint"]);

    let output = project.run_harn(&["sprint", "new", "second sprint"]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already active") || stderr.contains("Sprint already active"),
        "Expected 'already active' in stderr: {stderr}"
    );
}

#[test]
fn sprint_new_with_plan_link() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "user auth"]);
    let output = project.run_harn(&["sprint", "new", "implement login", "--plan", "user-auth"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Linked to plan: user-auth"));

    let state = project.read_file(".agents/harn/current-sprint.toml");
    assert!(state.contains("plan = \"user-auth\""));
}

#[test]
fn sprint_new_with_invalid_plan_fails() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["sprint", "new", "task", "--plan", "nonexistent"]);
    assert!(!output.status.success());
}

#[test]
fn sprint_status_no_active() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["sprint", "status"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No active sprint"));
}

#[test]
fn sprint_status_shows_current() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["sprint", "new", "implement login"]);

    let output = project.run_harn(&["sprint", "status"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("implement login"));
}

#[test]
fn sprint_done_archives_contract() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["sprint", "new", "quick task"]);

    // sprint done in non-interactive mode (piped stdout) skips handoff prompt
    let output = project.run_harn(&["sprint", "done"]);
    assert!(output.status.success());

    // Sprint state removed
    assert!(!project.file_exists(".agents/harn/current-sprint.toml"));

    // Contract moved to completed
    let completed_files: Vec<_> =
        std::fs::read_dir(project.path().join("docs/exec-plans/completed"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("sprint-"))
            .collect();
    assert_eq!(completed_files.len(), 1);
}

#[test]
fn sprint_done_fails_when_contract_missing() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["sprint", "new", "test task"]);

    // Delete the contract file but leave state
    let active_dir = project.path().join("docs/exec-plans/active");
    for entry in std::fs::read_dir(&active_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().to_string_lossy().starts_with("sprint-") {
            std::fs::remove_file(entry.path()).unwrap();
        }
    }

    let output = project.run_harn(&["sprint", "done"]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not found") || stderr.contains("contract"),
        "Should report missing contract, got: {stderr}"
    );

    // State should still exist (not cleared)
    assert!(project.file_exists(".agents/harn/current-sprint.toml"));
}

#[test]
fn sprint_done_when_no_active_fails() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["sprint", "done"]);
    assert!(!output.status.success());
}

#[test]
fn sprint_uses_contract_template() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["sprint", "new", "my task"]);

    let active_dir = project.path().join("docs/exec-plans/active");
    let sprint_file = std::fs::read_dir(&active_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with("sprint-"))
        .unwrap();
    let content = std::fs::read_to_string(sprint_file.path()).unwrap();
    assert!(content.contains("Sprint Contract: my task"));
    assert!(content.contains("## Deliverables"));
    assert!(content.contains("## Acceptance Criteria"));
}

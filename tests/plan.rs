mod helpers;

use helpers::TempProject;

fn init_project(project: &TempProject) {
    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(output.status.success());
}

#[test]
fn plan_new_creates_file() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["plan", "new", "user auth"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created:"));
    assert!(stdout.contains("user-auth"));

    // Check file exists in active dir
    let active_dir = project.path().join("docs/exec-plans/active");
    let files: Vec<_> = std::fs::read_dir(&active_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".md"))
        .collect();
    assert_eq!(files.len(), 1);
    assert!(files[0].contains("user-auth"));
}

#[test]
fn plan_new_with_explicit_slug() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["plan", "new", "payment processing", "--slug", "payments"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("payments"));
}

#[test]
fn plan_new_uses_exec_plan_template() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "my feature"]);

    let active_dir = project.path().join("docs/exec-plans/active");
    let file = std::fs::read_dir(&active_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().ends_with(".md"))
        .unwrap();
    let content = std::fs::read_to_string(file.path()).unwrap();
    assert!(content.contains("ExecPlan: my feature"));
    assert!(content.contains("## Purpose"));
    assert!(content.contains("## Milestones"));
}

#[test]
fn plan_list_shows_active_plans() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "first plan"]);
    project.run_harn(&["plan", "new", "second plan"]);

    let output = project.run_harn(&["plan", "list"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Active plans:"));
    assert!(stdout.contains("first-plan"));
    assert!(stdout.contains("second-plan"));
}

#[test]
fn plan_list_empty() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["plan", "list"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No plans found"));
}

#[test]
fn plan_complete_moves_to_completed() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "feature xyz"]);

    let output = project.run_harn(&["plan", "complete", "feature-xyz"]);
    assert!(output.status.success());

    // Should be in completed, not active
    let active_files: Vec<_> = std::fs::read_dir(project.path().join("docs/exec-plans/active"))
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".md"))
        .collect();
    assert!(active_files.is_empty());

    let completed_files: Vec<_> =
        std::fs::read_dir(project.path().join("docs/exec-plans/completed"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".md"))
            .collect();
    assert_eq!(completed_files.len(), 1);
}

#[test]
fn plan_complete_nonexistent_fails() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["plan", "complete", "nonexistent"]);
    assert!(!output.status.success());
}

#[test]
fn plan_without_init_fails() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["plan", "list"]);
    // plan list without init should either fail or show empty
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success() && stdout.contains("No plans found") || !output.status.success()
    );
}

#[test]
fn plan_list_shows_milestone_progress() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "milestone test"]);

    let active_dir = project.path().join("docs/exec-plans/active");
    let file = std::fs::read_dir(&active_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.contains("milestone-test") && name.ends_with(".md")
        })
        .unwrap();

    let content = "# ExecPlan: milestone test\n\n\
        ## Milestones\n\n\
        ### Milestone 1: Setup\n\n\
        ### Milestone 2: Build\n\n\
        ## Progress\n\n\
        - [x] Created project\n\
        - [ ] Implemented feature\n\
        - [ ] Wrote tests\n";

    std::fs::write(file.path(), content).unwrap();

    let output = project.run_harn(&["plan", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("2 milestones") && stdout.contains("1/3 tasks"),
        "Expected '2 milestones, 1/3 tasks' but got: {stdout}"
    );
}

#[test]
fn plan_complete_blocked_by_active_sprint() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "feature with sprint"]);
    project.run_harn(&[
        "sprint",
        "new",
        "sprint for feature",
        "--plan",
        "feature-with-sprint",
    ]);

    let output = project.run_harn(&["plan", "complete", "feature-with-sprint"]);
    assert!(
        !output.status.success(),
        "plan complete should fail when a linked sprint is active"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("active linked sprint") || stderr.contains("sprint done"),
        "Error should mention active sprint, got: {stderr}"
    );
}

#[test]
fn plan_list_shows_sprint_linkage() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "plan with sprint"]);
    project.run_harn(&[
        "sprint",
        "new",
        "linked sprint",
        "--plan",
        "plan-with-sprint",
    ]);

    let output = project.run_harn(&["plan", "list"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("sprint:") || stdout.contains("linked-sprint"),
        "plan list should show linked sprint, got: {stdout}"
    );
}

#[test]
fn plan_list_shows_created_date() {
    let project = TempProject::with_git();
    init_project(&project);

    project.run_harn(&["plan", "new", "dated plan"]);

    let output = project.run_harn(&["plan", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("created"),
        "plan list should show created date, got: {stdout}"
    );
}

#[test]
fn plan_slug_sequential_fallback() {
    let project = TempProject::with_git();
    init_project(&project);

    // Non-ASCII description with no usable chars
    let output = project.run_harn(&["plan", "new", "用户认证"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("plan-001"));
}

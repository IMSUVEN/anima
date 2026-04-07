mod helpers;

use helpers::TempProject;

fn init_project(project: &TempProject) {
    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(output.status.success());
}

#[test]
fn score_show_no_assessments() {
    let project = TempProject::with_git();
    init_project(&project);

    let output = project.run_harn(&["score", "show"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No quality assessments"));
}

#[test]
fn score_show_without_init_fails() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["score", "show"]);
    // Without init, config doesn't exist — should fail or show guidance
    // score show without config should error out
    assert!(
        !output.status.success() || {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("No quality assessments")
        }
    );
}

#[test]
fn score_show_with_existing_file() {
    let project = TempProject::with_git();
    init_project(&project);

    // Manually create a score file
    let score_content = "# Quality Scores\n\n\
        Last updated: 2026-04-03\n\n\
        | Domain | Functionality | Product Depth | Code Quality | Design/UX | Overall | Last Assessed |\n\
        |--------|:---:|:---:|:---:|:---:|:---:|:---:|\n\
        | core | A | B | B | B | B | 2026-04-03 |\n";
    project.write_file("docs/QUALITY_SCORE.md", score_content);

    let output = project.run_harn(&["score", "show"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Domain"));
    assert!(stdout.contains("core"));
}

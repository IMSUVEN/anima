mod helpers;

use helpers::TempProject;

#[test]
fn init_creates_full_file_tree() {
    let project = TempProject::with_git();
    let output = project.run_harn(&["init", "--tools", "codex,claude-code", "--stack", "rust"]);

    assert!(
        output.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(project.file_exists("AGENTS.md"));
    assert!(project.file_exists("CLAUDE.md"));
    assert!(project.file_exists("ARCHITECTURE.md"));
    assert!(project.file_exists(".agents/harn/config.toml"));
    assert!(project.file_exists("docs/design-docs/index.md"));
    assert!(project.file_exists("docs/design-docs/core-beliefs.md"));
    assert!(project.file_exists("docs/evaluation/criteria.md"));
    assert!(project.file_exists("docs/templates/exec-plan.md"));
    assert!(project.file_exists("docs/templates/sprint-contract.md"));
    assert!(project.file_exists("docs/templates/handoff.md"));

    assert!(project.file_exists("docs/exec-plans/active"));
    assert!(project.file_exists("docs/exec-plans/completed"));
    assert!(project.file_exists("docs/product-specs"));
    assert!(project.file_exists("docs/references"));
}

#[test]
fn init_agents_md_has_substantive_content() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let content = project.read_file("AGENTS.md");
    assert!(content.contains("## Quick Start"));
    assert!(content.contains("## Architecture"));
    assert!(content.contains("## Workflow"));
    assert!(content.contains("## Key Constraints"));
    assert!(content.contains("## Documentation Map"));
    assert!(content.contains("cargo build"));
}

#[test]
fn init_architecture_is_stack_aware() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let content = project.read_file("ARCHITECTURE.md");
    assert!(content.contains("cargo clippy"));
    assert!(content.contains("Common Mistakes"));
}

#[test]
fn init_skips_existing_files() {
    let project = TempProject::with_git();
    project.write_file("AGENTS.md", "# Custom agents file\n");

    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("skip") || stdout.contains("already exists"));

    let content = project.read_file("AGENTS.md");
    assert_eq!(content, "# Custom agents file\n");
}

#[test]
fn init_force_overwrites_existing() {
    let project = TempProject::with_git();
    project.write_file("AGENTS.md", "# Custom agents file\n");

    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust", "--force"]);
    assert!(output.status.success());

    let content = project.read_file("AGENTS.md");
    assert!(content.contains("## Workflow"));
}

#[test]
fn init_dry_run_writes_nothing() {
    let project = TempProject::with_git();

    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust", "--dry-run"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run"));
    assert!(stdout.contains("No files written"));

    assert!(!project.file_exists("AGENTS.md"));
    assert!(!project.file_exists(".agents/harn/config.toml"));
}

#[test]
fn init_detects_rust_stack() {
    let project = TempProject::with_git();
    project.write_file("Cargo.toml", "[package]\nname = \"test\"\n");

    let output = project.run_harn(&["init", "--tools", "codex"]);
    assert!(output.status.success());

    let config = project.read_file(".agents/harn/config.toml");
    assert!(config.contains("stack = \"rust\""));

    let arch = project.read_file("ARCHITECTURE.md");
    assert!(arch.contains("cargo clippy"));
}

#[test]
fn init_detects_node_stack() {
    let project = TempProject::with_git();
    project.write_file("package.json", "{}");

    let output = project.run_harn(&["init", "--tools", "codex"]);
    assert!(output.status.success());

    let config = project.read_file(".agents/harn/config.toml");
    assert!(config.contains("stack = \"node\""));
}

#[test]
fn init_config_has_file_hashes() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let config = project.read_file(".agents/harn/config.toml");
    assert!(config.contains("[init.file_hashes]"));
    assert!(config.contains("\"AGENTS.md\""));
}

#[test]
fn init_codex_only_no_claude_md() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    assert!(project.file_exists("AGENTS.md"));
    assert!(!project.file_exists("CLAUDE.md"));
}

#[test]
fn init_both_tools_creates_both_entry_files() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex,claude-code", "--stack", "rust"]);

    assert!(project.file_exists("AGENTS.md"));
    assert!(project.file_exists("CLAUDE.md"));
}

#[test]
fn init_claude_only_creates_agents_md() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "claude-code", "--stack", "rust"]);

    // AGENTS.md should always be generated regardless of tool selection
    assert!(project.file_exists("AGENTS.md"));
    assert!(project.file_exists("CLAUDE.md"));
}

#[test]
fn init_claude_only_then_check_passes() {
    let project = TempProject::with_git();
    let output = project.run_harn(&["init", "--tools", "claude-code", "--stack", "rust"]);
    assert!(output.status.success());

    let output = project.run_harn(&["check"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("does not exist"),
        "check should pass after claude-only init, got: {stdout}"
    );
}

#[test]
fn init_minimal_creates_essential_files() {
    let project = TempProject::with_git();
    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust", "--minimal"]);
    assert!(output.status.success());

    // Minimal should create the three essential files
    assert!(project.file_exists("AGENTS.md"));
    assert!(project.file_exists("ARCHITECTURE.md"));
    assert!(project.file_exists("docs/evaluation/criteria.md"));

    // Required dirs should exist even in minimal mode
    assert!(project.file_exists("docs/exec-plans/active"));
    assert!(project.file_exists("docs/exec-plans/completed"));
    assert!(project.file_exists("docs/templates"));

    // check should not report missing required files
    let output = project.run_harn(&["check"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("AGENTS.md does not exist"),
        "Required files should exist after minimal init, got: {stdout}"
    );
}

#[test]
fn init_idempotent_second_run_skips() {
    let project = TempProject::with_git();
    project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);

    let output = project.run_harn(&["init", "--tools", "codex", "--stack", "rust"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("skip") || stdout.contains("already exists"));
}

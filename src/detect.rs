use std::path::Path;

use crate::types::{AiTool, Stack};

/// Results from scanning the project directory for environment signals.
#[derive(Debug)]
pub struct DetectionResult {
    pub has_git: bool,
    pub stack: Option<Stack>,
    pub ai_tools: Vec<AiTool>,
    pub has_agents_dir: bool,
    pub has_docs_dir: bool,
}

/// Scan `project_root` for project environment signals.
pub fn detect(project_root: &Path) -> DetectionResult {
    DetectionResult {
        has_git: project_root.join(".git").exists(),
        stack: detect_stack(project_root),
        ai_tools: detect_ai_tools(project_root),
        has_agents_dir: project_root.join(".agents").exists(),
        has_docs_dir: project_root.join("docs").exists(),
    }
}

fn detect_stack(root: &Path) -> Option<Stack> {
    if root.join("Cargo.toml").exists() {
        Some(Stack::Rust)
    } else if root.join("package.json").exists() {
        Some(Stack::Node)
    } else if root.join("pyproject.toml").exists() || root.join("setup.py").exists() {
        Some(Stack::Python)
    } else if root.join("go.mod").exists() {
        Some(Stack::Go)
    } else {
        None
    }
}

fn detect_ai_tools(root: &Path) -> Vec<AiTool> {
    let mut tools = Vec::new();
    if root.join("CLAUDE.md").exists() {
        tools.push(AiTool::ClaudeCode);
    }
    if root.join("AGENTS.md").exists() {
        tools.push(AiTool::Codex);
    }
    tools
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = detect(dir.path());
        assert!(!result.has_git);
        assert!(result.stack.is_none());
        assert!(result.ai_tools.is_empty());
    }

    #[test]
    fn detect_rust_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        let result = detect(dir.path());
        assert_eq!(result.stack, Some(Stack::Rust));
    }

    #[test]
    fn detect_node_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("package.json"), "{}").unwrap();
        let result = detect(dir.path());
        assert_eq!(result.stack, Some(Stack::Node));
    }

    #[test]
    fn detect_python_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("pyproject.toml"), "").unwrap();
        let result = detect(dir.path());
        assert_eq!(result.stack, Some(Stack::Python));
    }

    #[test]
    fn detect_go_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("go.mod"), "module test").unwrap();
        let result = detect(dir.path());
        assert_eq!(result.stack, Some(Stack::Go));
    }

    #[test]
    fn detect_existing_ai_tools() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("CLAUDE.md"), "# test").unwrap();
        let result = detect(dir.path());
        assert!(result.ai_tools.contains(&AiTool::ClaudeCode));
        assert!(!result.ai_tools.contains(&AiTool::Codex));
    }

    #[test]
    fn detect_git_repo() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        let result = detect(dir.path());
        assert!(result.has_git);
    }
}

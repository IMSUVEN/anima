#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/// A temporary project directory for integration tests.
/// Automatically cleaned up when dropped.
pub struct TempProject {
    dir: TempDir,
}

impl TempProject {
    /// Create an empty temp directory.
    pub fn new() -> Self {
        Self {
            dir: TempDir::new().expect("failed to create temp dir"),
        }
    }

    /// Create a temp directory with `git init` already run.
    pub fn with_git() -> Self {
        let project = Self::new();
        let status = Command::new("git")
            .args(["init"])
            .current_dir(project.path())
            .output()
            .expect("failed to run git init");
        assert!(status.status.success(), "git init failed");
        project
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Write a file relative to the project root.
    pub fn write_file(&self, rel_path: &str, contents: &str) {
        let path = self.dir.path().join(rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create parent dirs");
        }
        std::fs::write(&path, contents).expect("failed to write file");
    }

    /// Check if a file exists relative to the project root.
    pub fn file_exists(&self, rel_path: &str) -> bool {
        self.dir.path().join(rel_path).exists()
    }

    /// Read a file relative to the project root.
    pub fn read_file(&self, rel_path: &str) -> String {
        std::fs::read_to_string(self.dir.path().join(rel_path))
            .unwrap_or_else(|_| panic!("failed to read {rel_path}"))
    }

    /// Get the path to the harn binary (built by cargo).
    pub fn harn_bin() -> PathBuf {
        let path = PathBuf::from(env!("CARGO_BIN_EXE_harn"));
        assert!(path.exists(), "harn binary not found at {}", path.display());
        path
    }

    /// Run harn with the given arguments in this project directory.
    pub fn run_harn(&self, args: &[&str]) -> std::process::Output {
        Command::new(Self::harn_bin())
            .args(args)
            .arg("--dir")
            .arg(self.path())
            .output()
            .expect("failed to run harn")
    }
}

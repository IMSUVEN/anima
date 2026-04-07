use std::fs;
use std::path::Path;

use anyhow::Result;
use console::style;

use crate::config::Config;
use crate::util::{extract_md_links, sha256_hex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct CheckResult {
    pub path: String,
    pub message: String,
    pub severity: Severity,
}

pub struct CheckReport {
    pub results: Vec<CheckResult>,
}

impl CheckReport {
    pub fn errors(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .count()
    }

    pub fn warnings(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity == Severity::Warning)
            .count()
    }
}

pub fn run(project_root: &Path, fix: bool, ci: bool) -> Result<u8> {
    let config = Config::load(project_root)?;
    let mut results = Vec::new();

    check_required_files(project_root, &config, &mut results);
    check_required_dirs(project_root, fix, &mut results);
    check_content_substantive(project_root, &config, &mut results);
    check_template_customization(project_root, &config, &mut results);
    check_cross_references(project_root, &mut results);
    check_agents_length(project_root, &mut results);
    check_arch_dependency_direction(project_root, &mut results);
    check_quality_score_exists(project_root, &mut results);

    let report = CheckReport { results };

    print_report(project_root, &report);

    let errors = report.errors();
    let warnings = report.warnings();

    if errors == 0 && warnings == 0 {
        println!("\n{}", style("All checks passed.").green().bold());
    } else {
        println!(
            "\nResult: {} error{}, {} warning{}",
            errors,
            if errors == 1 { "" } else { "s" },
            warnings,
            if warnings == 1 { "" } else { "s" },
        );
        if warnings > 0 {
            println!("Tip: run `harn gc` for time-based staleness analysis.");
        }
    }

    if ci {
        if errors > 0 {
            Ok(2)
        } else if warnings > 0 {
            Ok(1)
        } else {
            Ok(0)
        }
    } else if errors > 0 {
        Ok(2)
    } else {
        Ok(0)
    }
}

fn check_required_files(root: &Path, config: &Config, results: &mut Vec<CheckResult>) {
    for file in &config.check.required_files {
        let full = root.join(file);
        if !full.exists() {
            results.push(CheckResult {
                path: file.clone(),
                message: format!("{file} does not exist. Run `harn init` to create it."),
                severity: Severity::Error,
            });
        }
    }
}

fn check_required_dirs(root: &Path, fix: bool, results: &mut Vec<CheckResult>) {
    let required_dirs = [
        "docs/exec-plans/active",
        "docs/exec-plans/completed",
        "docs/templates",
    ];

    for dir in &required_dirs {
        let full = root.join(dir);
        if !full.exists() {
            if fix {
                if let Err(e) = fs::create_dir_all(&full) {
                    results.push(CheckResult {
                        path: dir.to_string(),
                        message: format!("Could not create {dir}: {e}"),
                        severity: Severity::Error,
                    });
                } else {
                    println!("  {} {} (created by --fix)", style("fixed").cyan(), dir);
                }
            } else {
                results.push(CheckResult {
                    path: dir.to_string(),
                    message: format!("{dir}/ does not exist. Run `harn check --fix` to create it."),
                    severity: Severity::Error,
                });
            }
        }
    }
}

fn check_content_substantive(root: &Path, config: &Config, results: &mut Vec<CheckResult>) {
    for file in &config.check.required_files {
        let full = root.join(file);
        if let Ok(content) = fs::read_to_string(&full) {
            let stripped = content
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
                .count();
            if stripped < 3 {
                results.push(CheckResult {
                    path: file.clone(),
                    message: format!("{file} has very little content (only headers/whitespace)."),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

fn check_template_customization(root: &Path, config: &Config, results: &mut Vec<CheckResult>) {
    for (file, original_hash) in &config.init.file_hashes {
        let full = root.join(file);
        if let Ok(content) = fs::read_to_string(&full) {
            let current_hash = sha256_hex(&content);
            if current_hash == *original_hash {
                results.push(CheckResult {
                    path: file.clone(),
                    message: format!("{file} still matches init template (not customized)."),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

fn check_cross_references(root: &Path, results: &mut Vec<CheckResult>) {
    let agents_path = root.join("AGENTS.md");
    if let Ok(content) = fs::read_to_string(&agents_path) {
        for line in content.lines() {
            for link in extract_md_links(line) {
                if link.starts_with("http://") || link.starts_with("https://") {
                    continue;
                }
                let target = root.join(&link);
                if !target.exists() {
                    results.push(CheckResult {
                        path: "AGENTS.md".to_string(),
                        message: format!("AGENTS.md references {link} which does not exist."),
                        severity: Severity::Error,
                    });
                }
            }
        }
    }
}

const AGENTS_LINE_LIMIT: usize = 150;

fn check_agents_length(root: &Path, results: &mut Vec<CheckResult>) {
    let path = root.join("AGENTS.md");
    if let Ok(content) = fs::read_to_string(&path) {
        let line_count = content.lines().count();
        if line_count > AGENTS_LINE_LIMIT {
            results.push(CheckResult {
                path: "AGENTS.md".to_string(),
                message: format!(
                    "AGENTS.md is {line_count} lines (recommended ≤{AGENTS_LINE_LIMIT}). \
                     Consider moving detailed content to linked documents."
                ),
                severity: Severity::Warning,
            });
        }
    }
}

fn check_arch_dependency_direction(root: &Path, results: &mut Vec<CheckResult>) {
    let path = root.join("ARCHITECTURE.md");
    if let Ok(content) = fs::read_to_string(&path) {
        let lower = content.to_lowercase();
        let has_direction = lower.contains("dependency")
            || lower.contains("dependencies flow")
            || lower.contains("downward only")
            || lower.contains("one direction");
        if !has_direction {
            results.push(CheckResult {
                path: "ARCHITECTURE.md".to_string(),
                message: "ARCHITECTURE.md does not mention dependency direction. \
                         Add a statement about which way dependencies flow."
                    .to_string(),
                severity: Severity::Warning,
            });
        }
    }
}

fn check_quality_score_exists(root: &Path, results: &mut Vec<CheckResult>) {
    let path = root.join("docs/QUALITY_SCORE.md");
    if !path.exists() {
        results.push(CheckResult {
            path: "docs/QUALITY_SCORE.md".to_string(),
            message: "No quality score found. Run `harn score update` to create one.".to_string(),
            severity: Severity::Warning,
        });
    }
}

fn print_report(root: &Path, report: &CheckReport) {
    let project_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

    println!();
    println!("Harness integrity check: {project_name}");
    println!();

    if report.results.is_empty() {
        for file in &[
            "AGENTS.md",
            "ARCHITECTURE.md",
            "docs/evaluation/criteria.md",
        ] {
            if root.join(file).exists() {
                println!("  {} {file} exists and has content", style("✓").green());
            }
        }
        return;
    }

    for result in &report.results {
        match result.severity {
            Severity::Error => {
                println!("  {} {}", style("✗").red(), result.message);
            }
            Severity::Warning => {
                println!("  {} {}", style("⚠").yellow(), result.message);
            }
        }
    }
}

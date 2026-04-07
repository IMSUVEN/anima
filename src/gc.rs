use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use console::style;

use crate::config::Config;
use crate::util::{extract_md_links, sha256_hex};

#[derive(Debug, serde::Serialize)]
pub struct GcFinding {
    pub path: String,
    pub severity: String,
    pub message: String,
}

pub fn run(
    project_root: &Path,
    days_override: Option<u32>,
    report_only: bool,
    json: bool,
) -> Result<()> {
    let config = Config::load(project_root)?;
    let threshold_days = days_override.unwrap_or(config.gc.stale_threshold_days);
    let mut findings = Vec::new();

    check_age(project_root, threshold_days, &mut findings);
    check_code_doc_divergence(project_root, &config, &mut findings);
    check_template_customization(project_root, &config, &mut findings);
    check_references(project_root, &mut findings);

    if json {
        let output = serde_json::to_string_pretty(&findings)
            .context("Failed to serialize gc findings to JSON")?;
        println!("{output}");
        return Ok(());
    }

    println!();
    println!("Scanning documentation freshness...");
    println!();

    if findings.is_empty() {
        println!("  {} All documentation is current.", style("✓").green());
    } else {
        for finding in &findings {
            match finding.severity.as_str() {
                "error" => println!("  {} {}", style("✗").red(), finding.message),
                "warning" => println!("  {} {}", style("⚠").yellow(), finding.message),
                _ => println!("  {} {}", style("ℹ").blue(), finding.message),
            }
        }
        println!();
        let count = findings.len();
        println!(
            "Found {} potentially stale document{}.",
            count,
            if count == 1 { "" } else { "s" }
        );
        if !report_only {
            println!("Consider reviewing with your AI coding tool, or updating manually.");
        }
    }

    Ok(())
}

fn check_age(project_root: &Path, threshold_days: u32, findings: &mut Vec<GcFinding>) {
    let repo = match git2::Repository::discover(project_root) {
        Ok(r) => r,
        Err(_) => return,
    };

    let doc_files = collect_doc_files(project_root);
    let now = chrono::Utc::now().timestamp();

    for rel_path in &doc_files {
        if let Some(last_modified) = git_last_modified(&repo, rel_path) {
            let days_old = (now - last_modified) / 86400;
            if days_old > threshold_days as i64 {
                findings.push(GcFinding {
                    path: rel_path.clone(),
                    severity: "info".to_string(),
                    message: format!("{rel_path} — not modified in {days_old} days"),
                });
            }
        }
    }
}

fn check_code_doc_divergence(project_root: &Path, config: &Config, findings: &mut Vec<GcFinding>) {
    let repo = match git2::Repository::discover(project_root) {
        Ok(r) => r,
        Err(_) => return,
    };

    for mapping in &config.gc.mappings {
        let doc_time = git_last_modified(&repo, &mapping.doc);
        let code_time = mapping
            .code
            .iter()
            .filter_map(|p| git_last_modified(&repo, p))
            .max();

        if let (Some(dt), Some(ct)) = (doc_time, code_time) {
            if ct > dt {
                let code_commits = count_commits_since(&repo, &mapping.code, dt);
                findings.push(GcFinding {
                    path: mapping.doc.clone(),
                    severity: "warning".to_string(),
                    message: format!(
                        "{} — not modified since related code changed ({} commit{} since last doc update)",
                        mapping.doc,
                        code_commits,
                        if code_commits == 1 { "" } else { "s" }
                    ),
                });
            }
        }
    }
}

fn check_template_customization(
    project_root: &Path,
    config: &Config,
    findings: &mut Vec<GcFinding>,
) {
    for (file, original_hash) in &config.init.file_hashes {
        let full = project_root.join(file);
        if let Ok(content) = fs::read_to_string(&full) {
            let current = sha256_hex(&content);
            if current == *original_hash {
                findings.push(GcFinding {
                    path: file.clone(),
                    severity: "warning".to_string(),
                    message: format!("{file} — still matches init template"),
                });
            }
        }
    }
}

fn check_references(project_root: &Path, findings: &mut Vec<GcFinding>) {
    let agents_path = project_root.join("AGENTS.md");
    if let Ok(content) = fs::read_to_string(&agents_path) {
        for line in content.lines() {
            for link in extract_md_links(line) {
                if link.starts_with("http://") || link.starts_with("https://") {
                    continue;
                }
                let target = project_root.join(&link);
                if !target.exists() {
                    findings.push(GcFinding {
                        path: "AGENTS.md".to_string(),
                        severity: "error".to_string(),
                        message: format!("AGENTS.md references {link} which does not exist"),
                    });
                }
            }
        }
    }
}

fn collect_doc_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in ["AGENTS.md", "ARCHITECTURE.md", "CLAUDE.md"] {
        if root.join(entry).exists() {
            files.push(entry.to_string());
        }
    }
    let docs_dir = root.join("docs");
    if docs_dir.exists() {
        for entry in walkdir::WalkDir::new(&docs_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "md")
            })
        {
            if let Ok(rel) = entry.path().strip_prefix(root) {
                files.push(rel.to_string_lossy().to_string());
            }
        }
    }
    files
}

fn git_last_modified(repo: &git2::Repository, path: &str) -> Option<i64> {
    let mut revwalk = repo.revwalk().ok()?;
    revwalk.push_head().ok()?;
    revwalk.set_sorting(git2::Sort::TIME).ok()?;

    for oid in revwalk.flatten() {
        let commit = repo.find_commit(oid).ok()?;
        let tree = commit.tree().ok()?;
        if tree.get_path(Path::new(path)).is_ok() {
            if let Some(parent) = commit.parents().next() {
                let parent_tree = parent.tree().ok()?;
                if parent_tree.get_path(Path::new(path)).is_ok() {
                    let a_entry = parent_tree.get_path(Path::new(path)).ok()?;
                    let b_entry = tree.get_path(Path::new(path)).ok()?;
                    if a_entry.id() == b_entry.id() {
                        continue;
                    }
                }
            }
            return Some(commit.time().seconds());
        }
    }
    None
}

fn count_commits_since(repo: &git2::Repository, paths: &[String], since: i64) -> usize {
    let mut count = 0;
    if let Ok(mut revwalk) = repo.revwalk() {
        let _ = revwalk.push_head();
        let _ = revwalk.set_sorting(git2::Sort::TIME);
        for oid in revwalk.flatten() {
            if let Ok(commit) = repo.find_commit(oid) {
                if commit.time().seconds() <= since {
                    break;
                }
                let tree = match commit.tree() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                for path in paths {
                    if tree.get_path(Path::new(path)).is_ok() {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}

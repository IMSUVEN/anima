use std::fs;
use std::path::Path;

use anyhow::Result;
use console::style;

use crate::config::Config;
use crate::sprint::SprintState;

pub fn run(project_root: &Path) -> Result<()> {
    let config = Config::load(project_root)?;

    let tools: Vec<&str> = config.tools.agents.iter().map(|t| t.as_str()).collect();

    println!();
    println!(
        "Project: {} ({})",
        style(&config.project.name).bold(),
        config.init.stack
    );
    println!("Harness: harn v{}", config.project.harn_version);
    println!("Tools: {}", tools.join(", "));
    println!();

    // Sprint info
    let sprint_path = project_root.join(".agents/harn/current-sprint.toml");
    if sprint_path.exists() {
        match fs::read_to_string(&sprint_path) {
            Ok(content) => match toml::from_str::<SprintState>(&content) {
                Ok(state) => {
                    let progress = sprint_progress(project_root, &state);
                    println!("Sprint: {} {}", style(&state.name).bold(), progress);
                    if let Some(ref plan) = state.plan {
                        println!("  └─ plan: {plan}");
                    }
                }
                Err(_) => {
                    println!(
                        "Sprint: {} (invalid state file — run `harn sprint done` or fix {})",
                        style("unreadable").red(),
                        sprint_path.display()
                    );
                }
            },
            Err(_) => {
                println!(
                    "Sprint: {} (could not read {})",
                    style("unreadable").red(),
                    sprint_path.display()
                );
            }
        }
    } else {
        println!("Sprint: {}", style("none active").dim());
    }

    // Active plans
    let active_dir = project_root.join("docs/exec-plans/active");
    let plans = list_plan_names(&active_dir);
    println!("Active plans: {}", plans.len());
    for plan in &plans {
        println!("  • {}", plan.trim_end_matches(".md"));
    }

    Ok(())
}

fn sprint_progress(project_root: &Path, state: &SprintState) -> String {
    let path = project_root.join(&state.contract_path);
    if let Ok(content) = fs::read_to_string(path) {
        let checked = content
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("- [x]") || t.starts_with("- [X]")
            })
            .count();
        let unchecked = content
            .lines()
            .filter(|l| l.trim_start().starts_with("- [ ]"))
            .count();
        let total = checked + unchecked;
        if total > 0 {
            format!("({checked}/{total} acceptance criteria)")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

fn list_plan_names(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".md")
                && !name.starts_with("sprint-")
                && !name.starts_with("handoff-")
            {
                files.push(name);
            }
        }
    }
    files.sort();
    files
}

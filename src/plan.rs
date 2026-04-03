use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::types::{HarnDate, Slug};

const ACTIVE_DIR: &str = "docs/exec-plans/active";
const COMPLETED_DIR: &str = "docs/exec-plans/completed";

pub fn new_plan(project_root: &Path, description: &str, slug_override: Option<&str>) -> Result<()> {
    let active_dir = project_root.join(ACTIVE_DIR);
    ensure_dir(&active_dir)?;

    let slug = resolve_slug(description, slug_override, &active_dir, "plan")?;
    let date = HarnDate::today();
    let filename = format!("{}-{}.md", date, slug);
    let filepath = active_dir.join(&filename);

    if filepath.exists() {
        bail!(
            "Plan file already exists: {}\nChoose a different slug with --slug or wait until tomorrow.",
            filepath.display()
        );
    }

    let template = include_str!("../templates/docs/templates/exec-plan.md");
    let content = template.replace(
        "# ExecPlan: [Short, Action-Oriented Title]",
        &format!("# ExecPlan: {description}"),
    );

    fs::write(&filepath, content)
        .with_context(|| format!("Could not write plan file: {}", filepath.display()))?;

    println!();
    println!("Created: {ACTIVE_DIR}/{filename}");
    println!();
    println!("Edit this file to fill in:");
    println!("  - Purpose and context");
    println!("  - Scope (in/out)");
    println!("  - Milestones with observable acceptance");
    println!("  - Validation and acceptance criteria");

    Ok(())
}

pub fn list_plans(project_root: &Path) -> Result<()> {
    let active_dir = project_root.join(ACTIVE_DIR);
    let completed_dir = project_root.join(COMPLETED_DIR);

    let active_plans = list_plan_files(&active_dir);
    let completed_plans = list_plan_files(&completed_dir);

    println!();
    if active_plans.is_empty() && completed_plans.is_empty() {
        println!("No plans found.");
        println!("Create one with: harn plan new \"description\"");
        return Ok(());
    }

    if !active_plans.is_empty() {
        println!("Active plans:");
        for (i, plan) in active_plans.iter().enumerate() {
            let milestones = count_milestones(project_root, ACTIVE_DIR, plan);
            println!(
                "  {}. {} ({})",
                i + 1,
                plan.trim_end_matches(".md"),
                milestones
            );
        }
    }

    if !completed_plans.is_empty() {
        println!();
        println!("Completed:");
        for (i, plan) in completed_plans.iter().enumerate() {
            println!(
                "  {}. {}",
                active_plans.len() + i + 1,
                plan.trim_end_matches(".md")
            );
        }
    }

    Ok(())
}

pub fn complete_plan(project_root: &Path, name: &str) -> Result<()> {
    let active_dir = project_root.join(ACTIVE_DIR);
    let completed_dir = project_root.join(COMPLETED_DIR);
    ensure_dir(&completed_dir)?;

    let plan_file = find_plan_file(&active_dir, name)?;
    let source = active_dir.join(&plan_file);
    let dest = completed_dir.join(&plan_file);

    // Check for active linked sprint
    let sprint_state_path = project_root.join(".agents/harn/current-sprint.toml");
    if sprint_state_path.exists() {
        let sprint_content = fs::read_to_string(&sprint_state_path)?;
        let plan_slug = extract_slug_from_filename(&plan_file);
        if sprint_content.contains(&format!("plan = \"{plan_slug}\""))
            || sprint_content.contains(&format!("plan = \"{}\"", name))
        {
            bail!(
                "Plan \"{name}\" has an active linked sprint.\n\
                 Run `harn sprint done` to complete the sprint first."
            );
        }
    }

    fs::rename(&source, &dest).with_context(|| {
        format!(
            "Could not move plan from {} to {}",
            source.display(),
            dest.display()
        )
    })?;

    println!();
    println!(
        "Plan \"{}\" completed. Moved to {COMPLETED_DIR}/{}",
        name, plan_file
    );

    Ok(())
}

fn resolve_slug(
    description: &str,
    slug_override: Option<&str>,
    dir: &Path,
    prefix: &str,
) -> Result<Slug> {
    if let Some(explicit) = slug_override {
        return Slug::from_explicit(explicit);
    }
    match Slug::from_description(description) {
        Some(slug) => Ok(slug),
        None => Slug::sequential(prefix, dir),
    }
}

fn ensure_dir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .with_context(|| format!("Could not create directory: {}", dir.display()))?;
    }
    Ok(())
}

fn list_plan_files(dir: &Path) -> Vec<String> {
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

fn find_plan_file(dir: &Path, name: &str) -> Result<String> {
    let plans = list_plan_files(dir);

    // Exact filename match
    if plans.contains(&format!("{name}.md")) {
        return Ok(format!("{name}.md"));
    }

    // Slug match (name appears anywhere in filename)
    let matches: Vec<&String> = plans.iter().filter(|p| p.contains(name)).collect();

    match matches.len() {
        0 => bail!(
            "No plan found matching \"{name}\" in {}\nRun `harn plan list` to see available plans.",
            dir.display()
        ),
        1 => Ok(matches[0].clone()),
        _ => {
            let names: Vec<&str> = matches.iter().map(|s| s.as_str()).collect();
            bail!(
                "Multiple plans match \"{name}\": {}\nBe more specific.",
                names.join(", ")
            );
        }
    }
}

fn extract_slug_from_filename(filename: &str) -> &str {
    // Format: YYYY-MM-DD-slug.md → extract slug
    let without_ext = filename.trim_end_matches(".md");
    if without_ext.len() > 11 && without_ext.as_bytes()[10] == b'-' {
        &without_ext[11..]
    } else {
        without_ext
    }
}

fn count_milestones(project_root: &Path, dir: &str, filename: &str) -> String {
    let path = project_root.join(dir).join(filename);
    if let Ok(content) = fs::read_to_string(path) {
        let total = content
            .lines()
            .filter(|l| l.starts_with("### Milestone"))
            .count();
        let checked = content
            .lines()
            .filter(|l| l.trim_start().starts_with("- [x]") || l.trim_start().starts_with("- [X]"))
            .count();
        let unchecked = content
            .lines()
            .filter(|l| l.trim_start().starts_with("- [ ]"))
            .count();

        if total > 0 {
            format!("{checked}/{} milestones", total)
        } else if checked + unchecked > 0 {
            format!("{checked}/{} tasks", checked + unchecked)
        } else {
            "no milestones yet".to_string()
        }
    } else {
        "unreadable".to_string()
    }
}

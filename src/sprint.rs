use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use console::style;

use crate::types::{HarnDate, Slug, SprintState};

const ACTIVE_DIR: &str = "docs/exec-plans/active";
const COMPLETED_DIR: &str = "docs/exec-plans/completed";
const SPRINT_STATE: &str = ".agents/harn/current-sprint.toml";

pub fn new_sprint(
    project_root: &Path,
    description: &str,
    slug_override: Option<&str>,
    plan: Option<&str>,
) -> Result<()> {
    let state_path = project_root.join(SPRINT_STATE);
    if state_path.exists() {
        let existing = load_sprint_state(project_root)?;
        bail!(
            "Sprint already active: \"{}\"\n\
             Run `harn sprint done` to complete it first, or delete {}.",
            existing.name,
            SPRINT_STATE
        );
    }

    let active_dir = project_root.join(ACTIVE_DIR);
    if !active_dir.exists() {
        fs::create_dir_all(&active_dir).with_context(|| {
            format!(
                "Could not create directory: {}. Check filesystem permissions.",
                active_dir.display()
            )
        })?;
    }

    let slug = resolve_slug(description, slug_override, &active_dir)?;
    let date = HarnDate::today();
    let filename = format!("sprint-{}-{}.md", date, slug);
    let contract_path = format!("{ACTIVE_DIR}/{filename}");
    let full_path = project_root.join(&contract_path);

    if let Some(plan_name) = plan {
        let plan_exists = fs::read_dir(&active_dir)
            .with_context(|| {
                format!(
                    "Could not read {}.\nCheck that the directory exists and has correct permissions.",
                    active_dir.display()
                )
            })?
            .filter_map(|e| e.ok())
            .any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with("sprint-") || name.starts_with("handoff-") {
                    return false;
                }
                let slug = extract_slug_from_plan_filename(&name);
                slug == plan_name || name.contains(&format!("-{plan_name}.md"))
            });
        if !plan_exists {
            bail!(
                "No active plan found matching \"{plan_name}\".\n\
                 Run `harn plan list` to see available plans."
            );
        }
    }

    let template = include_str!("../templates/docs/templates/sprint-contract.md");
    let content = template.replace(
        "# Sprint Contract: [Title]",
        &format!("# Sprint Contract: {description}"),
    );

    fs::write(&full_path, content).with_context(|| {
        format!(
            "Could not write sprint contract: {}. Check filesystem permissions.",
            full_path.display()
        )
    })?;

    let state = SprintState {
        name: description.to_string(),
        slug,
        created: date,
        plan: plan.map(|s| s.to_string()),
        contract_path: crate::types::HarnPath::new(&contract_path)?,
    };

    save_sprint_state(project_root, &state)?;

    println!();
    println!("Created sprint contract:");
    println!("  Contract: {contract_path}");
    println!("  State: {SPRINT_STATE}");
    if let Some(p) = plan {
        println!("  Linked to plan: {p}");
    }
    println!();
    println!("Fill in deliverables and acceptance criteria before starting work.");

    Ok(())
}

pub fn sprint_status(project_root: &Path) -> Result<()> {
    let state_path = project_root.join(SPRINT_STATE);
    if !state_path.exists() {
        println!();
        println!("No active sprint.");
        println!("Create one with: harn sprint new \"description\"");
        return Ok(());
    }

    let state = load_sprint_state(project_root)?;
    let contract_path = state.contract_path.resolve(project_root);

    println!();
    println!(
        "Sprint: {} {}",
        style(&state.name).bold(),
        style(format!("(created {})", state.created)).dim()
    );
    if let Some(ref plan) = state.plan {
        println!("  └─ plan: {plan}");
    }

    if let Ok(content) = fs::read_to_string(contract_path) {
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
            println!("  Acceptance criteria: {checked}/{total}");
            println!();
            for line in content.lines() {
                let t = line.trim_start();
                if t.starts_with("- [x]") || t.starts_with("- [X]") {
                    println!("    {} {}", style("✓").green(), &t[6..]);
                } else if t.starts_with("- [ ]") {
                    println!("    {} {}", style("○").dim(), &t[6..]);
                }
            }
        }
    }

    Ok(())
}

pub fn sprint_done(project_root: &Path) -> Result<()> {
    let state = load_sprint_state(project_root)?;
    let completed_dir = project_root.join(COMPLETED_DIR);
    if !completed_dir.exists() {
        fs::create_dir_all(&completed_dir).with_context(|| {
            format!(
                "Could not create directory: {}. Check filesystem permissions.",
                completed_dir.display()
            )
        })?;
    }

    // Move contract from active to completed
    let source = state.contract_path.resolve(project_root);
    let filename = state
        .contract_path
        .as_path()
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .with_context(|| {
            format!(
                "Invalid contract path in sprint state: \"{}\"\n\
                 Fix the path in {} or delete the file and run `harn sprint new`.",
                state.contract_path, SPRINT_STATE
            )
        })?;
    let dest = completed_dir.join(&filename);

    if source.exists() {
        fs::rename(&source, &dest).with_context(|| {
            format!(
                "Could not move sprint contract from {} to {}. Check filesystem permissions.",
                source.display(),
                dest.display()
            )
        })?;
    } else {
        bail!(
            "Sprint contract not found: {}\n\
             The contract file may have been moved or deleted.\n\
             To force cleanup, delete {} manually.",
            source.display(),
            SPRINT_STATE
        );
    }

    // Remove sprint state only after successful archive
    let state_path = project_root.join(SPRINT_STATE);
    fs::remove_file(&state_path).with_context(|| {
        format!(
            "Could not remove sprint state: {}. Check filesystem permissions.",
            state_path.display()
        )
    })?;

    println!();
    println!("Sprint \"{}\" completed.", state.name);

    // Generate handoff if stdout is a terminal
    if std::io::IsTerminal::is_terminal(&std::io::stdout()) {
        use dialoguer::Confirm;
        let generate = Confirm::new()
            .with_prompt("Generate handoff artifact for context reset?")
            .default(false)
            .interact()?;

        if generate {
            generate_handoff(project_root, &state)?;
        }
    }

    Ok(())
}

fn generate_handoff(project_root: &Path, state: &SprintState) -> Result<()> {
    let completed_dir = project_root.join(COMPLETED_DIR);
    let date = HarnDate::today();
    let filename = format!("handoff-{}-{}.md", date, state.slug);
    let filepath = completed_dir.join(&filename);

    let template = include_str!("../templates/docs/templates/handoff.md");
    let content = template.replace(
        "# Handoff: [From Context] → [To Context]",
        &format!("# Handoff: {} → Next Context", state.name),
    );

    fs::write(&filepath, &content).with_context(|| {
        format!(
            "Could not write handoff: {}. Check filesystem permissions.",
            filepath.display()
        )
    })?;

    println!();
    println!("Created: {COMPLETED_DIR}/{filename}");
    println!("Edit the handoff to record:");
    println!("  - Completed work");
    println!("  - Current state");
    println!("  - Known issues");
    println!("  - Next steps");

    Ok(())
}

fn extract_slug_from_plan_filename(filename: &str) -> &str {
    let without_ext = filename.trim_end_matches(".md");
    // Skip YYYY-MM-DD- prefix (11 chars) if present
    if without_ext.len() > 11
        && without_ext.as_bytes().get(4) == Some(&b'-')
        && without_ext.as_bytes().get(7) == Some(&b'-')
        && without_ext.as_bytes().get(10) == Some(&b'-')
    {
        &without_ext[11..]
    } else {
        without_ext
    }
}

fn resolve_slug(description: &str, slug_override: Option<&str>, dir: &Path) -> Result<Slug> {
    if let Some(explicit) = slug_override {
        return Slug::from_explicit(explicit);
    }
    match Slug::from_description(description) {
        Some(slug) => Ok(slug),
        None => Slug::sequential("sprint", dir),
    }
}

fn load_sprint_state(project_root: &Path) -> Result<SprintState> {
    let path = project_root.join(SPRINT_STATE);
    let content = fs::read_to_string(&path).with_context(|| {
        format!(
            "Could not read sprint state at {}.\n\
             There may be no active sprint. Run `harn sprint new` to create one.",
            path.display()
        )
    })?;
    let state: SprintState = toml::from_str(&content).with_context(|| {
        format!(
            "Invalid sprint state at {}.\n\
             Fix the TOML syntax or delete the file and run `harn sprint new`.",
            path.display()
        )
    })?;
    Ok(state)
}

fn save_sprint_state(project_root: &Path, state: &SprintState) -> Result<()> {
    let path = project_root.join(SPRINT_STATE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Could not create directory: {}. Check filesystem permissions.",
                parent.display()
            )
        })?;
    }
    let content =
        toml::to_string_pretty(state).context("Failed to serialize sprint state to TOML")?;
    fs::write(&path, content).with_context(|| {
        format!(
            "Could not write sprint state: {}. Check filesystem permissions.",
            path.display()
        )
    })?;
    Ok(())
}

use clap::{Parser, Subcommand};
use std::path::Path;
use std::{fs, io};

const SEED_AGENTS: &str = include_str!("../seed/AGENTS.md");
const SEED_ARCHITECTURE: &str = include_str!("../seed/docs/ARCHITECTURE.md");
const SEED_DECISIONS_README: &str = include_str!("../seed/docs/decisions/README.md");

#[derive(Parser)]
#[command(name = "anima", about = "Plant seeds, not templates.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Plant a seed into the current project
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Observe the project's cultivation state
    Check,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { name } => {
            if let Err(e) = run_init(name) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Command::Check => {
            if let Err(e) = run_check() {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn run_init(name: Option<String>) -> io::Result<()> {
    let project_name = match name {
        Some(n) => n,
        None => infer_project_name()?,
    };

    if Path::new("AGENTS.md").exists() {
        eprintln!("warning: AGENTS.md already exists, skipping");
        return Ok(());
    }

    let agents_content = SEED_AGENTS.replace("{project-name}", &project_name);

    fs::write("AGENTS.md", agents_content)?;
    fs::create_dir_all("docs/decisions")?;
    fs::write("docs/ARCHITECTURE.md", SEED_ARCHITECTURE)?;
    fs::write("docs/decisions/README.md", SEED_DECISIONS_README)?;

    println!("Seed planted for '{project_name}'.");
    println!();
    println!("  AGENTS.md              — start here");
    println!("  docs/ARCHITECTURE.md   — fills as architecture emerges");
    println!("  docs/decisions/        — record decisions as you go");

    Ok(())
}

fn infer_project_name() -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let name = cwd
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");
    Ok(name.to_string())
}

// --- check command ---

fn run_check() -> io::Result<()> {
    println!("anima check");
    println!();

    if !Path::new("AGENTS.md").exists() {
        println!("  No seed found. Run `anima init` to plant one.");
        return Ok(());
    }

    let agents = fs::read_to_string("AGENTS.md")?;

    let phase = extract_phase(&agents);
    let phase_initial = phase.as_deref() == Some("Germination");
    let phase_display = match &phase {
        Some(p) if phase_initial => format!("{p} (initial)"),
        Some(p) => p.clone(),
        None => "unknown".to_string(),
    };

    let arch = observe_architecture()?;
    let decisions = count_decisions()?;
    let conventions = count_conventions(&agents);

    println!("  state:         {phase_display}");
    println!("  architecture:  {arch}");
    println!(
        "  decisions:     {}",
        if decisions == 0 {
            "none recorded".to_string()
        } else {
            format!("{decisions} recorded")
        }
    );
    println!(
        "  conventions:   {}",
        if conventions == 0 {
            "none yet".to_string()
        } else {
            format!("{conventions} established")
        }
    );
    println!();

    let mut dormant: Vec<&str> = Vec::new();
    if phase_initial {
        dormant.push("state");
    }
    if arch.contains("empty") || arch.contains("not found") {
        dormant.push("architecture");
    }
    if decisions == 0 {
        dormant.push("decisions");
    }
    if conventions == 0 {
        dormant.push("conventions");
    }

    if dormant.is_empty() {
        println!("  The harness is growing well.");
    } else if dormant.len() == 4 {
        println!("  The seed is planted but dormant. Growth begins with practice.");
    } else {
        println!("  Dormant areas: {}", dormant.join(", "));
    }

    Ok(())
}

fn extract_phase(content: &str) -> Option<String> {
    for line in content.lines() {
        let Some(start) = line.find("**Phase:") else {
            continue;
        };
        let after = line[start + 8..].trim_start();
        if let Some(end) = after.find(".**") {
            return Some(after[..end].to_string());
        }
        if let Some(end) = after.find("**") {
            return Some(after[..end].trim_end_matches('.').to_string());
        }
    }
    None
}

fn observe_architecture() -> io::Result<String> {
    let path = Path::new("docs/ARCHITECTURE.md");
    if !path.exists() {
        return Ok("not found".to_string());
    }
    let content = fs::read_to_string(path)?;
    if content.contains("This document is empty") {
        Ok("empty (initial)".to_string())
    } else {
        Ok("documented".to_string())
    }
}

fn count_decisions() -> io::Result<usize> {
    let dir = Path::new("docs/decisions");
    if !dir.exists() {
        return Ok(0);
    }
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let name = entry?.file_name();
        let name = name.to_string_lossy();
        if name.ends_with(".md") && name.as_ref() != "README.md" {
            count += 1;
        }
    }
    Ok(count)
}

fn count_conventions(content: &str) -> usize {
    let mut in_section = false;
    let mut count = 0;
    for line in content.lines() {
        if line.starts_with("## Conventions") {
            in_section = true;
            continue;
        }
        if in_section && line.starts_with("## ") {
            break;
        }
        if in_section {
            let t = line.trim();
            if t.is_empty() || t.contains("None yet") {
                continue;
            }
            if t.starts_with("- ") || t.starts_with("* ") {
                count += 1;
            }
        }
    }
    count
}

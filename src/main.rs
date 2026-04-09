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

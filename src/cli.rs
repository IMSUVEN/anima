use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

/// A CLI tool that bootstraps and maintains harness structures for AI-agent-driven development.
#[derive(Debug, Parser)]
#[command(name = "harn", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Show detailed output
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Operate on a different project directory
    #[arg(long = "dir", short = 'C', global = true)]
    pub project_dir: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Bootstrap harness structure for a new or existing project
    Init {
        /// Project name
        #[arg(long, short = 'n')]
        name: Option<String>,

        /// AI tools (comma-separated): claude-code, codex
        #[arg(long, short = 't', value_delimiter = ',')]
        tools: Option<Vec<String>>,

        /// Stack hint: rust, node, python, go, generic
        #[arg(long, short = 's')]
        stack: Option<String>,

        /// Full interactive mode with all options
        #[arg(long, short = 'i')]
        interactive: bool,

        /// Only generate essential core
        #[arg(long)]
        minimal: bool,

        /// Use custom external templates
        #[arg(long)]
        template_dir: Option<PathBuf>,

        /// Overwrite existing files without confirmation
        #[arg(long, short = 'f')]
        force: bool,

        /// Show what would be generated, don't write
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate harness structure integrity
    Check {
        /// Auto-fix simple issues (create missing dirs)
        #[arg(long)]
        fix: bool,

        /// Exit code 1 on warnings, 2 on errors (for CI pipelines)
        #[arg(long)]
        ci: bool,
    },

    /// Manage execution plans
    Plan {
        #[command(subcommand)]
        action: PlanAction,
    },

    /// Manage sprint contracts
    Sprint {
        #[command(subcommand)]
        action: SprintAction,
    },

    /// Show current project state at a glance
    Status,

    /// Detect stale documentation using git history analysis
    Gc {
        /// Staleness threshold in days
        #[arg(long)]
        days: Option<u32>,

        /// Output report only, no suggestions
        #[arg(long)]
        report: bool,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Exit code 1 on warnings, 2 on errors (for CI pipelines)
        #[arg(long)]
        ci: bool,
    },

    /// View and update quality grades
    Score {
        #[command(subcommand)]
        action: ScoreAction,
    },

    /// Update harness structure when harn version changes
    Upgrade {
        /// Show what would change, don't write
        #[arg(long)]
        dry_run: bool,

        /// Use custom external templates (mirrors init's escape hatch)
        #[arg(long)]
        template_dir: Option<PathBuf>,
    },

    /// Assess harness maturity against HARNESS-SPEC levels
    Assess {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum PlanAction {
    /// Create a new execution plan
    New {
        /// Plan description
        description: String,

        /// Explicit slug for the filename
        #[arg(long)]
        slug: Option<String>,
    },

    /// List active and recently completed plans
    List,

    /// Complete a plan (move from active to completed)
    Complete {
        /// Plan name or slug to complete
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum SprintAction {
    /// Create a new sprint contract
    New {
        /// Sprint description
        description: String,

        /// Explicit slug for the filename
        #[arg(long)]
        slug: Option<String>,

        /// Link to a parent plan
        #[arg(long)]
        plan: Option<String>,
    },

    /// Show current sprint state
    Status,

    /// Complete the current sprint
    Done,
}

#[derive(Debug, Subcommand)]
pub enum ScoreAction {
    /// Display quality scores
    Show,

    /// Interactive quality score update
    Update,
}

pub fn dispatch(cli: Cli) -> Result<u8> {
    let project_root = match cli.project_dir {
        Some(dir) => dir,
        None => std::env::current_dir().context(
            "Could not determine current directory.\n\
             The working directory may have been deleted. Use --dir to specify a project path.",
        )?,
    };

    match cli.command {
        Command::Init {
            name,
            tools,
            stack,
            interactive,
            minimal,
            template_dir,
            force,
            dry_run,
        } => {
            crate::init::run_from_args(
                &project_root,
                name,
                tools,
                stack,
                interactive,
                minimal,
                template_dir,
                force,
                dry_run,
            )?;
            Ok(0)
        }
        Command::Check { fix, ci } => crate::check::run(&project_root, fix, ci),
        Command::Plan { action } => {
            match action {
                PlanAction::New { description, slug } => {
                    crate::plan::new_plan(&project_root, &description, slug.as_deref())?;
                }
                PlanAction::List => crate::plan::list_plans(&project_root)?,
                PlanAction::Complete { name } => {
                    crate::plan::complete_plan(&project_root, &name)?;
                }
            }
            Ok(0)
        }
        Command::Sprint { action } => {
            match action {
                SprintAction::New {
                    description,
                    slug,
                    plan,
                } => crate::sprint::new_sprint(
                    &project_root,
                    &description,
                    slug.as_deref(),
                    plan.as_deref(),
                )?,
                SprintAction::Status => crate::sprint::sprint_status(&project_root)?,
                SprintAction::Done => crate::sprint::sprint_done(&project_root)?,
            }
            Ok(0)
        }
        Command::Status => {
            crate::status::run(&project_root)?;
            Ok(0)
        }
        Command::Gc {
            days,
            report,
            json,
            ci,
        } => crate::gc::run(&project_root, days, report, json, ci),
        Command::Score { action } => {
            match action {
                ScoreAction::Show => crate::score::show(&project_root)?,
                ScoreAction::Update => crate::score::update(&project_root)?,
            }
            Ok(0)
        }
        Command::Upgrade {
            dry_run,
            template_dir,
        } => {
            crate::upgrade::run(&project_root, dry_run, template_dir)?;
            Ok(0)
        }
        Command::Assess { json } => {
            crate::assess::run(&project_root, json)?;
            Ok(0)
        }
    }
}

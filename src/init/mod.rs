pub mod render;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use console::style;

use crate::config::{CheckSection, Config, GcSection, InitSection, ProjectSection, ToolsSection};
use crate::detect::DetectionResult;
use crate::types::{AiTool, HarnDate, ProjectName, Stack};
use render::{RenderContext, RenderedFile};

/// Options resolved from CLI flags, detection, and prompts.
pub struct InitOptions {
    pub project_name: ProjectName,
    pub tools: Vec<AiTool>,
    pub stack: Stack,
    pub force: bool,
    pub dry_run: bool,
    pub minimal: bool,
    pub template_dir: Option<std::path::PathBuf>,
    pub interactive: bool,
}

/// Run the full init pipeline: detect → resolve → render → write.
pub fn run(project_root: &Path, opts: InitOptions, detection: &DetectionResult) -> Result<()> {
    print_detection(detection);

    let date = HarnDate::today();
    let ctx = RenderContext {
        project_name: opts.project_name.as_str().to_string(),
        project_description: "TODO: Describe your project in 1-2 sentences.".to_string(),
        date: date.as_str().to_string(),
        harn_version: env!("CARGO_PKG_VERSION").to_string(),
        stack: opts.stack,
    };

    let files = if let Some(ref tpl_dir) = opts.template_dir {
        render::render_all_from_dir(tpl_dir, &ctx)?
    } else {
        render::render_all(&ctx)?
    };

    let files = render::filter_by_tools(files, &opts.tools);
    let files = if opts.minimal {
        render::filter_minimal(files)
    } else {
        files
    };

    if opts.dry_run {
        print_dry_run(&files);
        return Ok(());
    }

    let dirs = render::init_directories();
    for dir in &dirs {
        let full = project_root.join(dir);
        if !full.exists() {
            fs::create_dir_all(&full).with_context(|| {
                format!(
                    "Could not create directory: {}. Check filesystem permissions.",
                    full.display()
                )
            })?;
        }
    }

    let mut file_count = 0;
    let mut skip_count = 0;
    let mut file_hashes = BTreeMap::new();

    for file in &files {
        let full = project_root.join(&file.rel_path);

        if full.exists() && !opts.force {
            println!(
                "  {} {} (already exists)",
                style("skip").yellow(),
                file.rel_path
            );
            skip_count += 1;
            continue;
        }

        if let Some(parent) = full.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "Could not create directory: {}. Check filesystem permissions.",
                        parent.display()
                    )
                })?;
            }
        }

        fs::write(&full, &file.content).with_context(|| {
            format!(
                "Could not write file: {}\nCheck file permissions and try again.",
                full.display()
            )
        })?;

        let hash = sha256_hex(&file.content);
        file_hashes.insert(file.rel_path.clone(), hash);

        println!("  {} {}", style("✓").green(), file.rel_path);
        file_count += 1;
    }

    let config = Config {
        project: ProjectSection {
            name: opts.project_name.clone(),
            created: date.clone(),
            harn_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        tools: ToolsSection {
            agents: opts.tools.clone(),
        },
        init: InitSection {
            stack: opts.stack,
            file_hashes,
        },
        check: CheckSection {
            required_files: vec![
                "AGENTS.md".to_string(),
                "ARCHITECTURE.md".to_string(),
                "docs/evaluation/criteria.md".to_string(),
            ],
        },
        gc: GcSection {
            stale_threshold_days: 14,
            ignore_paths: vec![],
            mappings: vec![],
        },
    };

    let config_path = crate::config::config_path(project_root);
    if config_path.exists() && !opts.force {
        println!(
            "  {} .agents/harn/config.toml (already exists)",
            style("skip").yellow()
        );
    } else {
        config.save(project_root)?;
        println!("  {} .agents/harn/config.toml", style("✓").green());
        file_count += 1;
    }

    println!();
    if skip_count > 0 {
        println!(
            "Done! Created {} files, skipped {} (already exist). Use --force to overwrite.",
            file_count, skip_count
        );
    } else {
        println!("Done! Created {} files.", file_count);
    }

    print_next_steps();

    Ok(())
}

/// Resolve AI tools from CLI flags, detection, or interactive prompt.
pub fn resolve_tools(
    cli_tools: &Option<Vec<String>>,
    detection: &DetectionResult,
    interactive: bool,
) -> Result<Vec<AiTool>> {
    if let Some(tools) = cli_tools {
        return tools
            .iter()
            .map(|s| s.parse::<AiTool>())
            .collect::<Result<Vec<_>>>();
    }

    if interactive {
        return prompt_tools();
    }

    if !detection.ai_tools.is_empty() {
        return Ok(detection.ai_tools.clone());
    }

    if atty_stdout() {
        prompt_tools()
    } else {
        Ok(vec![AiTool::Codex, AiTool::ClaudeCode])
    }
}

/// Resolve stack from CLI flag, detection, or default.
pub fn resolve_stack(cli_stack: &Option<String>, detection: &DetectionResult) -> Result<Stack> {
    if let Some(s) = cli_stack {
        return s.parse::<Stack>();
    }
    Ok(detection.stack.unwrap_or(Stack::Generic))
}

/// Resolve project name from CLI flag or directory name.
pub fn resolve_project_name(cli_name: &Option<String>, project_root: &Path) -> Result<ProjectName> {
    if let Some(name) = cli_name {
        return ProjectName::new(name);
    }
    let dir_name = project_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "my-project".to_string());
    ProjectName::new(dir_name)
}

fn prompt_tools() -> Result<Vec<AiTool>> {
    use dialoguer::MultiSelect;
    let items = vec!["codex", "claude-code"];
    let defaults = vec![true, true];
    let selections = MultiSelect::new()
        .with_prompt("AI coding tools")
        .items(&items)
        .defaults(&defaults)
        .interact()?;

    let mut tools = Vec::new();
    for idx in selections {
        tools.push(items[idx].parse::<AiTool>()?);
    }
    if tools.is_empty() {
        tools = vec![AiTool::Codex, AiTool::ClaudeCode];
    }
    Ok(tools)
}

fn atty_stdout() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}

fn print_detection(detection: &DetectionResult) {
    println!();
    println!("Detecting project environment...");
    if detection.has_git {
        println!("  {} Git repository", style("✓").green());
    } else {
        println!("  {} No git repository", style("✗").red());
    }
    match detection.stack {
        Some(stack) => println!("  {} {} project", style("✓").green(), stack),
        None => println!(
            "  {} No package manager detected (generic project)",
            style("✗").red()
        ),
    }
    if detection.ai_tools.is_empty() {
        println!("  {} No AI tool configs detected", style("✗").red());
    } else {
        for tool in &detection.ai_tools {
            println!("  {} {} already configured", style("✓").green(), tool);
        }
    }
    println!();
    println!("Creating harness structure...");
}

fn print_dry_run(files: &[RenderedFile]) {
    println!();
    println!("Dry run — would create:");
    for file in files {
        println!("  {}", file.rel_path);
    }
    for dir in render::init_directories() {
        println!("  {}/", dir);
    }
    println!("  .agents/harn/config.toml");
    println!();
    println!("No files written.");
}

fn print_next_steps() {
    println!();
    println!("Next steps:");
    println!("  1. Edit AGENTS.md — fill in project overview and key constraints");
    println!("  2. Edit ARCHITECTURE.md — define domain structure and layer rules");
    println!("  3. Review docs/evaluation/criteria.md — adjust quality criteria");
    println!("  4. Run `harn check` to validate structural integrity");
}

/// Entry point for `harn init` called from CLI dispatch.
/// Performs detection, resolution, and init in one call so cli.rs stays pure dispatch.
#[allow(clippy::too_many_arguments)]
pub fn run_from_args(
    project_root: &Path,
    name: Option<String>,
    tools: Option<Vec<String>>,
    stack: Option<String>,
    interactive: bool,
    minimal: bool,
    template_dir: Option<std::path::PathBuf>,
    force: bool,
    dry_run: bool,
) -> Result<()> {
    let detection = crate::detect::detect(project_root);
    let resolved_tools = resolve_tools(&tools, &detection, interactive)?;
    let resolved_stack = resolve_stack(&stack, &detection)?;
    let project_name = resolve_project_name(&name, project_root)?;

    let opts = InitOptions {
        project_name,
        tools: resolved_tools,
        stack: resolved_stack,
        force,
        dry_run,
        minimal,
        template_dir,
        interactive,
    };
    run(project_root, opts, &detection)
}

pub use crate::util::sha256_hex;

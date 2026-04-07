use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use console::style;

use crate::config::Config;
use crate::init::render::{self, RenderContext};
use crate::util::sha256_hex;

pub fn run(
    project_root: &Path,
    dry_run: bool,
    template_dir: Option<std::path::PathBuf>,
) -> Result<()> {
    let mut config = Config::load(project_root)?;

    let ctx = RenderContext {
        project_name: config.project.name.as_str().to_string(),
        project_description: "TODO: Describe your project in 1-2 sentences.".to_string(),
        date: config.project.created.as_str().to_string(),
        harn_version: env!("CARGO_PKG_VERSION").to_string(),
        stack: config.init.stack,
    };

    let new_files = if let Some(ref tpl_dir) = template_dir {
        render::render_all_from_dir(tpl_dir, &ctx)?
    } else {
        render::render_all(&ctx)?
    };
    let new_files = render::filter_by_tools(new_files, &config.tools.agents);

    println!();
    println!(
        "Upgrading harness to harn v{}...",
        env!("CARGO_PKG_VERSION")
    );
    println!();

    let mut overwritten = 0;
    let mut sidecars = 0;
    let mut created = 0;

    for file in &new_files {
        let full_path = project_root.join(&file.rel_path);
        let new_hash = sha256_hex(&file.content);

        if !full_path.exists() {
            if dry_run {
                println!(
                    "  {} {} (new file)",
                    style("would create").cyan(),
                    file.rel_path
                );
            } else {
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!(
                            "Could not create directory: {}. Check filesystem permissions.",
                            parent.display()
                        )
                    })?;
                }
                fs::write(&full_path, &file.content).with_context(|| {
                    format!(
                        "Could not write file: {}. Check filesystem permissions.",
                        full_path.display()
                    )
                })?;
                config
                    .init
                    .file_hashes
                    .insert(file.rel_path.clone(), new_hash);
                println!(
                    "  {} {} (new file)",
                    style("created").green(),
                    file.rel_path
                );
            }
            created += 1;
            continue;
        }

        let current_content = fs::read_to_string(&full_path).with_context(|| {
            format!(
                "Could not read file: {}. Check filesystem permissions.",
                full_path.display()
            )
        })?;
        let current_hash = sha256_hex(&current_content);

        if let Some(original_hash) = config.init.file_hashes.get(&file.rel_path) {
            if current_hash == *original_hash {
                // File unchanged from init — safe to overwrite
                if current_hash == new_hash {
                    println!(
                        "  {} {} (already up to date)",
                        style("✓").green(),
                        file.rel_path
                    );
                    continue;
                }
                if dry_run {
                    println!(
                        "  {} {} (unchanged, would overwrite)",
                        style("would update").cyan(),
                        file.rel_path
                    );
                } else {
                    fs::write(&full_path, &file.content).with_context(|| {
                        format!(
                            "Could not write file: {}. Check filesystem permissions.",
                            full_path.display()
                        )
                    })?;
                    config
                        .init
                        .file_hashes
                        .insert(file.rel_path.clone(), new_hash);
                    println!(
                        "  {} {} (updated from template)",
                        style("updated").green(),
                        file.rel_path
                    );
                }
                overwritten += 1;
            } else {
                // File was modified by user — generate sidecar
                if dry_run {
                    println!(
                        "  {} {} (modified, would create .harn-upgrade sidecar)",
                        style("would sidecar").yellow(),
                        file.rel_path
                    );
                } else {
                    let sidecar = format!("{}.harn-upgrade", full_path.display());
                    fs::write(&sidecar, &file.content).with_context(|| {
                        format!("Could not write sidecar: {sidecar}. Check filesystem permissions.")
                    })?;
                    println!(
                        "  {} {} (modified — sidecar created)",
                        style("sidecar").yellow(),
                        file.rel_path
                    );
                }
                sidecars += 1;
            }
        } else {
            // No hash record — treat as user-created, generate sidecar
            if dry_run {
                println!(
                    "  {} {} (no hash record, would create sidecar)",
                    style("would sidecar").yellow(),
                    file.rel_path
                );
            } else {
                let sidecar = format!("{}.harn-upgrade", full_path.display());
                fs::write(&sidecar, &file.content).with_context(|| {
                    format!("Could not write sidecar: {sidecar}. Check filesystem permissions.")
                })?;
                println!(
                    "  {} {} (no hash record — sidecar created)",
                    style("sidecar").yellow(),
                    file.rel_path
                );
            }
            sidecars += 1;
        }
    }

    // Update config version
    if !dry_run {
        config.project.harn_version = env!("CARGO_PKG_VERSION").to_string();
        config.save(project_root)?;
    }

    println!();
    if dry_run {
        println!(
            "Dry run complete. Would update {overwritten}, create {created}, sidecar {sidecars} file(s)."
        );
    } else {
        println!(
            "Upgrade complete. Updated {overwritten}, created {created}, sidecar {sidecars} file(s)."
        );
        if sidecars > 0 {
            println!();
            println!("Review .harn-upgrade files and merge changes manually or with your AI tool.");
        }
    }

    Ok(())
}

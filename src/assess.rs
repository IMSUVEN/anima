use anyhow::Result;
use console::style;
use std::fs;
use std::path::Path;

/// Harness maturity assessment based on HARNESS-SPEC.md levels.
///
/// Checks project structure, tooling, and workflow artifacts against
/// the specification's MUST/SHOULD requirements at each maturity level.

#[derive(Debug)]
struct Assessment {
    category: &'static str,
    requirement: &'static str,
    level: u8,
    obligation: Obligation,
    status: Status,
    detail: String,
}

#[derive(Debug, Clone, Copy)]
enum Obligation {
    Must,
    Should,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Pass,
    Fail,
    Partial,
}

pub fn run(project_root: &Path, json: bool) -> Result<()> {
    let assessments = run_assessments(project_root);

    if json {
        print_json(&assessments)?;
    } else {
        print_report(project_root, &assessments);
    }

    Ok(())
}

fn run_assessments(root: &Path) -> Vec<Assessment> {
    vec![
        // === Level 1: Single Agent + Loop ===
        // §1.1 Repository Knowledge
        check_agents_md(root),
        check_architecture_md(root),
        check_knowledge_in_repo(root),
        // §1.2 Codebase Legibility
        check_type_checked_language(root),
        // §1.4 Mechanical Enforcement
        check_ci_config(root),
        check_linter_config(root),
        // §1.5 Testing
        check_test_speed(root),
        // §1.6 Dev Environment
        check_dev_setup(root),
        // §1.7 Safety
        check_no_secrets(root),
        // === Level 2: Multi-Agent + Planning ===
        // §2.2 Execution Plans
        check_exec_plans(root),
        // §2.3 Sprint Contracts
        check_sprint_contracts(root),
        // §2.5 Quality Criteria
        check_quality_criteria(root),
        // §4.1 Entropy Management
        check_entropy_management(root),
        // §4.2 Knowledge Hygiene
        check_doc_code_mappings(root),
    ]
}

fn check_agents_md(root: &Path) -> Assessment {
    let path = root.join("AGENTS.md");
    if !path.exists() {
        return Assessment {
            category: "Repository Knowledge",
            requirement: "AGENTS.md at repository root as agent entry point",
            level: 1,
            obligation: Obligation::Must,
            status: Status::Fail,
            detail: "AGENTS.md not found. Run `harn init` to create it.".into(),
        };
    }

    let content = fs::read_to_string(&path).unwrap_or_default();
    let lines = content.lines().count();

    if lines > 150 {
        Assessment {
            category: "Repository Knowledge",
            requirement: "AGENTS.md at repository root as agent entry point",
            level: 1,
            obligation: Obligation::Must,
            status: Status::Partial,
            detail: format!(
                "AGENTS.md exists but is {lines} lines (recommended ≤150). \
                 It should be a map, not a manual."
            ),
        }
    } else {
        Assessment {
            category: "Repository Knowledge",
            requirement: "AGENTS.md at repository root as agent entry point",
            level: 1,
            obligation: Obligation::Must,
            status: Status::Pass,
            detail: format!("AGENTS.md exists ({lines} lines)."),
        }
    }
}

fn check_architecture_md(root: &Path) -> Assessment {
    let path = root.join("ARCHITECTURE.md");
    let status = if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let lower = content.to_lowercase();
        if lower.contains("dependency") || lower.contains("module") {
            Status::Pass
        } else {
            Status::Partial
        }
    } else {
        Status::Fail
    };

    Assessment {
        category: "Repository Knowledge",
        requirement: "ARCHITECTURE.md with domain layering and module boundaries",
        level: 1,
        obligation: Obligation::Should,
        status,
        detail: match status {
            Status::Pass => "ARCHITECTURE.md exists with dependency/module information.".into(),
            Status::Partial => {
                "ARCHITECTURE.md exists but lacks dependency direction or module structure.".into()
            }
            Status::Fail => "ARCHITECTURE.md not found.".into(),
        },
    }
}

fn check_knowledge_in_repo(root: &Path) -> Assessment {
    let docs_dir = root.join("docs");
    let has_docs = docs_dir.exists() && docs_dir.is_dir();
    let has_design = root.join("docs/design-docs").exists();
    let has_eval = root.join("docs/evaluation").exists();

    let status = if has_docs && (has_design || has_eval) {
        Status::Pass
    } else if has_docs {
        Status::Partial
    } else {
        Status::Fail
    };

    Assessment {
        category: "Repository Knowledge",
        requirement: "Structured knowledge in versioned docs/",
        level: 1,
        obligation: Obligation::Must,
        status,
        detail: match status {
            Status::Pass => {
                "docs/ directory with design-docs and/or evaluation criteria found.".into()
            }
            Status::Partial => {
                "docs/ exists but missing design-docs or evaluation subdirectories.".into()
            }
            Status::Fail => {
                "No docs/ directory. Run `harn init` to create the knowledge structure.".into()
            }
        },
    }
}

fn check_type_checked_language(root: &Path) -> Assessment {
    let signals = [
        ("Cargo.toml", "Rust (type-safe)"),
        ("tsconfig.json", "TypeScript (type-safe)"),
        ("pyproject.toml", "Python (check for mypy/pyright)"),
        ("go.mod", "Go (type-safe)"),
    ];

    for (file, lang) in &signals {
        if root.join(file).exists() {
            let is_strong = *file != "pyproject.toml";
            return Assessment {
                category: "Codebase Legibility",
                requirement: "Language with build-time type checking",
                level: 1,
                obligation: Obligation::Must,
                status: if is_strong {
                    Status::Pass
                } else {
                    Status::Partial
                },
                detail: format!("Detected: {lang}."),
            };
        }
    }

    if root.join("package.json").exists() {
        return Assessment {
            category: "Codebase Legibility",
            requirement: "Language with build-time type checking",
            level: 1,
            obligation: Obligation::Must,
            status: Status::Partial,
            detail: "JavaScript detected. Consider adding TypeScript for type safety.".into(),
        };
    }

    Assessment {
        category: "Codebase Legibility",
        requirement: "Language with build-time type checking",
        level: 1,
        obligation: Obligation::Must,
        status: Status::Fail,
        detail: "No recognized type-checked language detected.".into(),
    }
}

fn check_ci_config(root: &Path) -> Assessment {
    let ci_indicators = [
        ".github/workflows",
        ".gitlab-ci.yml",
        ".circleci",
        "Jenkinsfile",
        ".travis.yml",
    ];

    let found = ci_indicators.iter().any(|p| root.join(p).exists());

    Assessment {
        category: "Mechanical Enforcement",
        requirement: "CI pipeline enforcing lint/test on every change",
        level: 1,
        obligation: Obligation::Must,
        status: if found { Status::Pass } else { Status::Fail },
        detail: if found {
            "CI configuration detected.".into()
        } else {
            "No CI configuration found (.github/workflows, .gitlab-ci.yml, etc.).".into()
        },
    }
}

fn check_linter_config(root: &Path) -> Assessment {
    let linter_indicators = [
        ".clippy.toml",
        "clippy.toml",
        ".eslintrc",
        ".eslintrc.js",
        ".eslintrc.json",
        "eslint.config.js",
        "eslint.config.mjs",
        ".flake8",
        "ruff.toml",
        "pyproject.toml",
        ".golangci.yml",
    ];

    let is_rust = root.join("Cargo.toml").exists();
    let found = linter_indicators.iter().any(|p| root.join(p).exists());

    Assessment {
        category: "Mechanical Enforcement",
        requirement: "Linter configured with enforced rules",
        level: 1,
        obligation: Obligation::Should,
        status: if found || is_rust {
            Status::Pass
        } else {
            Status::Fail
        },
        detail: if is_rust {
            "Rust project — cargo clippy available by default.".into()
        } else if found {
            "Linter configuration detected.".into()
        } else {
            "No linter configuration found.".into()
        },
    }
}

fn check_test_speed(root: &Path) -> Assessment {
    let config_path = root.join(".agents/harn/config.toml");
    if !config_path.exists() {
        return Assessment {
            category: "Testing",
            requirement: "Full test suite executable in ≤1 minute",
            level: 1,
            obligation: Obligation::Must,
            status: Status::Partial,
            detail: "Cannot verify — no harn config. Run tests manually to confirm speed.".into(),
        };
    }

    let is_rust = root.join("Cargo.toml").exists();
    let has_tests = if is_rust {
        root.join("tests").exists() || root.join("src").exists()
    } else {
        root.join("tests").exists() || root.join("test").exists() || root.join("__tests__").exists()
    };

    Assessment {
        category: "Testing",
        requirement: "Full test suite executable in ≤1 minute",
        level: 1,
        obligation: Obligation::Must,
        status: if has_tests {
            Status::Partial
        } else {
            Status::Fail
        },
        detail: if has_tests {
            "Test directory found. Run your test suite to verify it completes in ≤60s.".into()
        } else {
            "No test directory found.".into()
        },
    }
}

fn check_dev_setup(root: &Path) -> Assessment {
    let setup_indicators = [
        "Makefile",
        "justfile",
        "Taskfile.yml",
        "docker-compose.yml",
        "docker-compose.yaml",
        ".devcontainer",
        "flake.nix",
        "shell.nix",
    ];

    let is_rust = root.join("Cargo.toml").exists();
    let found = setup_indicators.iter().any(|p| root.join(p).exists());

    Assessment {
        category: "Dev Environment",
        requirement: "Dev environment spinnable in one command",
        level: 1,
        obligation: Obligation::Must,
        status: if found || is_rust {
            Status::Pass
        } else {
            Status::Partial
        },
        detail: if is_rust {
            "Rust project — `cargo build` provides one-command setup.".into()
        } else if found {
            "Build/setup configuration detected.".into()
        } else {
            "No build automation found (Makefile, justfile, docker-compose, etc.).".into()
        },
    }
}

fn check_no_secrets(root: &Path) -> Assessment {
    let danger_files = [".env", "credentials.json", "secrets.json", ".env.local"];
    let has_gitignore = root.join(".gitignore").exists();

    let found_secrets: Vec<&str> = danger_files
        .iter()
        .filter(|f| root.join(f).exists())
        .copied()
        .collect();

    let status = if found_secrets.is_empty() && has_gitignore {
        Status::Pass
    } else if found_secrets.is_empty() {
        Status::Partial
    } else {
        Status::Fail
    };

    Assessment {
        category: "Safety",
        requirement: "No secrets committed to repository",
        level: 1,
        obligation: Obligation::Should,
        status,
        detail: if !found_secrets.is_empty() {
            format!(
                "Potential secret files found: {}. Ensure these are in .gitignore.",
                found_secrets.join(", ")
            )
        } else if !has_gitignore {
            "No .gitignore found — ensure secrets are not committed.".into()
        } else {
            ".gitignore present, no secret files detected.".into()
        },
    }
}

fn check_exec_plans(root: &Path) -> Assessment {
    let active = root.join("docs/exec-plans/active");
    let completed = root.join("docs/exec-plans/completed");
    let template = root.join("docs/templates/exec-plan.md");

    let has_template = template.exists();
    let has_plans = active.exists()
        && fs::read_dir(&active)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.file_name().to_string_lossy().ends_with(".md"))
            })
            .unwrap_or(false);
    let has_completed = completed.exists()
        && fs::read_dir(&completed)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.file_name().to_string_lossy().ends_with(".md"))
            })
            .unwrap_or(false);

    let status = if has_plans || has_completed {
        Status::Pass
    } else if has_template {
        Status::Partial
    } else {
        Status::Fail
    };

    Assessment {
        category: "Execution Plans",
        requirement: "Self-contained ExecPlans for complex work",
        level: 2,
        obligation: Obligation::Must,
        status,
        detail: match status {
            Status::Pass => "Active or completed execution plans found.".into(),
            Status::Partial => {
                "Plan template exists but no plans created yet. Use `harn plan new`.".into()
            }
            Status::Fail => "No execution plan infrastructure. Run `harn init`.".into(),
        },
    }
}

fn check_sprint_contracts(root: &Path) -> Assessment {
    let template = root.join("docs/templates/sprint-contract.md");
    let state = root.join(".agents/harn/current-sprint.toml");

    let status = if state.exists() || template.exists() {
        if state.exists() {
            Status::Pass
        } else {
            Status::Partial
        }
    } else {
        Status::Fail
    };

    Assessment {
        category: "Sprint Contracts",
        requirement: "Negotiated sprint contracts with testable acceptance criteria",
        level: 2,
        obligation: Obligation::Should,
        status,
        detail: match status {
            Status::Pass => "Active sprint contract found.".into(),
            Status::Partial => {
                "Sprint template exists but no sprint active. Use `harn sprint new`.".into()
            }
            Status::Fail => "No sprint infrastructure. Run `harn init`.".into(),
        },
    }
}

fn check_quality_criteria(root: &Path) -> Assessment {
    let criteria = root.join("docs/evaluation/criteria.md");
    let score = root.join("docs/QUALITY_SCORE.md");

    let status = if criteria.exists() && score.exists() {
        Status::Pass
    } else if criteria.exists() {
        Status::Partial
    } else {
        Status::Fail
    };

    Assessment {
        category: "Quality Criteria",
        requirement: "Explicit grading criteria shared with generators and evaluators",
        level: 2,
        obligation: Obligation::Must,
        status,
        detail: match status {
            Status::Pass => "Evaluation criteria and quality scores both exist.".into(),
            Status::Partial => {
                "Criteria exist but no quality scores yet. Run `harn score update`.".into()
            }
            Status::Fail => "No evaluation criteria found.".into(),
        },
    }
}

fn check_entropy_management(root: &Path) -> Assessment {
    let config_path = root.join(".agents/harn/config.toml");
    let has_gc_config = config_path.exists()
        && fs::read_to_string(&config_path)
            .map(|c| c.contains("[gc]"))
            .unwrap_or(false);
    let has_debt_tracker = root.join("docs/exec-plans/tech-debt-tracker.md").exists();

    let status = if has_gc_config {
        Status::Pass
    } else if has_debt_tracker {
        Status::Partial
    } else {
        Status::Fail
    };

    Assessment {
        category: "Entropy Management",
        requirement: "Active entropy detection and correction",
        level: 1,
        obligation: Obligation::Must,
        status,
        detail: match status {
            Status::Pass => "GC configuration found — run `harn gc` to detect stale docs.".into(),
            Status::Partial => "Tech debt tracker found but no GC configuration.".into(),
            Status::Fail => "No entropy management configured. Run `harn init`.".into(),
        },
    }
}

fn check_doc_code_mappings(root: &Path) -> Assessment {
    let config_path = root.join(".agents/harn/config.toml");
    let has_mappings = config_path.exists()
        && fs::read_to_string(&config_path)
            .map(|c| c.contains("[[gc.mappings]]"))
            .unwrap_or(false);

    Assessment {
        category: "Knowledge Hygiene",
        requirement: "Documentation-to-code mappings for drift detection",
        level: 2,
        obligation: Obligation::Should,
        status: if has_mappings {
            Status::Pass
        } else {
            Status::Fail
        },
        detail: if has_mappings {
            "Doc-code mappings configured in config.toml.".into()
        } else {
            "No [[gc.mappings]] in config. Add mappings to detect doc drift.".into()
        },
    }
}

fn print_report(root: &Path, assessments: &[Assessment]) {
    let project_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

    println!();
    println!(
        "{}",
        style(format!("Harness Maturity Assessment: {project_name}")).bold()
    );
    println!();

    // Calculate scores per level
    let l1: Vec<&Assessment> = assessments.iter().filter(|a| a.level == 1).collect();
    let l2: Vec<&Assessment> = assessments.iter().filter(|a| a.level == 2).collect();

    let l1_score = level_score(&l1);
    let l2_score = level_score(&l2);

    let overall_level = if l1_score >= 70 && l2_score >= 50 {
        2
    } else if l1_score >= 50 {
        1
    } else {
        0
    };

    println!(
        "Overall: {} (Level 1: {}%, Level 2: {}%)",
        style(format!("Level {overall_level}")).bold().cyan(),
        l1_score,
        l2_score
    );
    println!();

    // Print Level 1
    println!(
        "{} ({}%)",
        style("Level 1 — Single Agent + Loop").bold().underlined(),
        l1_score
    );
    println!();
    print_level_assessments(&l1);

    // Print Level 2
    println!();
    println!(
        "{} ({}%)",
        style("Level 2 — Multi-Agent + Planning")
            .bold()
            .underlined(),
        l2_score
    );
    println!();
    print_level_assessments(&l2);

    // Summary
    println!();
    let fails: Vec<&Assessment> = assessments
        .iter()
        .filter(|a| a.status == Status::Fail && matches!(a.obligation, Obligation::Must))
        .collect();

    if fails.is_empty() {
        println!(
            "{}",
            style("All MUST requirements met. Focus on SHOULD items to strengthen the harness.")
                .green()
        );
    } else {
        println!(
            "{}",
            style(format!(
                "{} MUST requirement{} not met. Address these first:",
                fails.len(),
                if fails.len() == 1 { "" } else { "s" }
            ))
            .yellow()
        );
        for a in &fails {
            println!("  • {}: {}", a.category, a.detail);
        }
    }
}

fn print_level_assessments(assessments: &[&Assessment]) {
    let mut current_category = "";
    for a in assessments {
        if a.category != current_category {
            current_category = a.category;
        }
        let icon = match a.status {
            Status::Pass => style("✓").green(),
            Status::Partial => style("◐").yellow(),
            Status::Fail => style("✗").red(),
        };
        let obligation = match a.obligation {
            Obligation::Must => style("MUST").red().bold(),
            Obligation::Should => style("SHOULD").yellow(),
        };
        println!("  {icon} [{obligation}] {}: {}", a.category, a.requirement);
        if a.status != Status::Pass {
            println!("         {}", style(&a.detail).dim());
        }
    }
}

fn level_score(assessments: &[&Assessment]) -> u32 {
    if assessments.is_empty() {
        return 0;
    }
    let total_weight: f64 = assessments
        .iter()
        .map(|a| match a.obligation {
            Obligation::Must => 2.0,
            Obligation::Should => 1.0,
        })
        .sum();

    let earned: f64 = assessments
        .iter()
        .map(|a| {
            let weight = match a.obligation {
                Obligation::Must => 2.0,
                Obligation::Should => 1.0,
            };
            match a.status {
                Status::Pass => weight,
                Status::Partial => weight * 0.5,
                Status::Fail => 0.0,
            }
        })
        .sum();

    ((earned / total_weight) * 100.0) as u32
}

fn print_json(assessments: &[Assessment]) -> Result<()> {
    let json_items: Vec<serde_json::Value> = assessments
        .iter()
        .map(|a| {
            serde_json::json!({
                "category": a.category,
                "requirement": a.requirement,
                "level": a.level,
                "obligation": format!("{:?}", a.obligation).to_lowercase(),
                "status": match a.status {
                    Status::Pass => "pass",
                    Status::Partial => "partial",
                    Status::Fail => "fail",
                },
                "detail": a.detail,
            })
        })
        .collect();

    let output = serde_json::to_string_pretty(&json_items).unwrap_or_else(|_| "[]".to_string());
    println!("{output}");
    Ok(())
}

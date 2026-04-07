use std::fs;
use std::path::Path;

use super::{Assessment, Obligation, Status};

pub(super) fn run_assessments(root: &Path) -> Vec<Assessment> {
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

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            return Assessment {
                category: "Repository Knowledge",
                requirement: "AGENTS.md at repository root as agent entry point",
                level: 1,
                obligation: Obligation::Must,
                status: Status::Partial,
                detail: format!("AGENTS.md exists but could not be read: {e}"),
            };
        }
    };
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
    if !path.exists() {
        return Assessment {
            category: "Repository Knowledge",
            requirement: "ARCHITECTURE.md with domain layering and module boundaries",
            level: 1,
            obligation: Obligation::Should,
            status: Status::Fail,
            detail: "ARCHITECTURE.md not found.".into(),
        };
    }

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            return Assessment {
                category: "Repository Knowledge",
                requirement: "ARCHITECTURE.md with domain layering and module boundaries",
                level: 1,
                obligation: Obligation::Should,
                status: Status::Partial,
                detail: format!("ARCHITECTURE.md exists but could not be read: {e}"),
            };
        }
    };

    let lower = content.to_lowercase();
    let has_structure = lower.contains("dependency") || lower.contains("module");

    Assessment {
        category: "Repository Knowledge",
        requirement: "ARCHITECTURE.md with domain layering and module boundaries",
        level: 1,
        obligation: Obligation::Should,
        status: if has_structure {
            Status::Pass
        } else {
            Status::Partial
        },
        detail: if has_structure {
            "ARCHITECTURE.md exists with dependency/module information.".into()
        } else {
            "ARCHITECTURE.md exists but lacks dependency direction or module structure.".into()
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
    let has_gc_config = if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(c) => c.contains("[gc]"),
            Err(_) => false,
        }
    } else {
        false
    };
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
    let has_mappings = if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(c) => c.contains("[[gc.mappings]]"),
            Err(_) => false,
        }
    } else {
        false
    };

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

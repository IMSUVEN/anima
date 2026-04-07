use std::path::Path;

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use minijinja::Environment;

use crate::types::Stack;

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Context variables for template rendering.
pub struct RenderContext {
    pub project_name: String,
    pub project_description: String,
    pub date: String,
    pub harn_version: String,
    pub stack: Stack,
}

impl RenderContext {
    fn to_minijinja(&self) -> minijinja::Value {
        minijinja::context! {
            project_name => self.project_name,
            project_description => self.project_description,
            date => self.date,
            harn_version => self.harn_version,
            stack => self.stack.as_str(),
        }
    }
}

/// A template file with its relative output path and rendered content.
pub struct RenderedFile {
    pub rel_path: String,
    pub content: String,
}

/// List of all template files that should be rendered during init.
/// Returns (template_path_in_embedded_dir, output_rel_path) pairs.
fn template_manifest() -> Vec<(&'static str, &'static str)> {
    vec![
        ("AGENTS.md.j2", "AGENTS.md"),
        ("CLAUDE.md.j2", "CLAUDE.md"),
        ("ARCHITECTURE.md.j2", "ARCHITECTURE.md"),
        ("docs/design-docs/index.md.j2", "docs/design-docs/index.md"),
        (
            "docs/design-docs/core-beliefs.md.j2",
            "docs/design-docs/core-beliefs.md",
        ),
        (
            "docs/evaluation/criteria.md.j2",
            "docs/evaluation/criteria.md",
        ),
        // Static templates (no Jinja variables) — copied as-is
        ("docs/templates/exec-plan.md", "docs/templates/exec-plan.md"),
        (
            "docs/templates/sprint-contract.md",
            "docs/templates/sprint-contract.md",
        ),
        ("docs/templates/handoff.md", "docs/templates/handoff.md"),
    ]
}

/// Directories to create (empty, no files) during init.
pub fn init_directories() -> Vec<&'static str> {
    vec![
        "docs/exec-plans/active",
        "docs/exec-plans/completed",
        "docs/templates",
        "docs/product-specs",
        "docs/references",
    ]
}

/// Render all templates with the given context.
/// Templates ending in `.j2` are processed through minijinja.
/// Other files are included verbatim.
pub fn render_all(ctx: &RenderContext) -> Result<Vec<RenderedFile>> {
    let mut env = Environment::new();
    let jinja_ctx = ctx.to_minijinja();
    let manifest = template_manifest();
    let mut rendered = Vec::with_capacity(manifest.len());

    for (template_path, output_path) in &manifest {
        let file = TEMPLATES_DIR
            .get_file(template_path)
            .with_context(|| format!("Embedded template not found: {template_path}. This is a bug — please report it at https://github.com/imsuven/harn/issues."))?;
        let source = file
            .contents_utf8()
            .with_context(|| format!("Template is not valid UTF-8: {template_path}. This is a bug — please report it at https://github.com/imsuven/harn/issues."))?;

        let content = if template_path.ends_with(".j2") {
            env.add_template(template_path, source).with_context(|| {
                format!(
                    "Failed to parse template: {template_path}. This is a bug — please report it."
                )
            })?;
            let tmpl = env.get_template(template_path)?;
            tmpl.render(&jinja_ctx).with_context(|| {
                format!(
                    "Failed to render template: {template_path}. This is a bug — please report it."
                )
            })?
        } else {
            source.to_string()
        };

        rendered.push(RenderedFile {
            rel_path: output_path.to_string(),
            content,
        });
    }

    Ok(rendered)
}

/// Render templates from an external directory instead of embedded templates.
pub fn render_all_from_dir(template_dir: &Path, ctx: &RenderContext) -> Result<Vec<RenderedFile>> {
    let jinja_ctx = ctx.to_minijinja();
    let manifest = template_manifest();
    let mut rendered = Vec::with_capacity(manifest.len());

    for (template_path, output_path) in &manifest {
        let full_path = template_dir.join(template_path);
        let source = std::fs::read_to_string(&full_path).with_context(|| {
            format!(
                "Could not read external template: {}\nEnsure --template-dir points to a valid template directory.",
                full_path.display()
            )
        })?;

        let content = if template_path.ends_with(".j2") {
            let mut env = Environment::new();
            env.add_template(template_path, &source)
                .with_context(|| format!("Failed to parse template: {}. Check the Jinja2 syntax in your custom template.", full_path.display()))?;
            let tmpl = env.get_template(template_path)?;
            tmpl.render(&jinja_ctx).with_context(|| {
                format!(
                    "Failed to render template: {}. Check template variable references.",
                    full_path.display()
                )
            })?
        } else {
            source
        };

        rendered.push(RenderedFile {
            rel_path: output_path.to_string(),
            content,
        });
    }

    Ok(rendered)
}

/// Filter rendered files to only include those needed for a minimal init.
pub fn filter_minimal(files: Vec<RenderedFile>) -> Vec<RenderedFile> {
    let minimal_paths = [
        "AGENTS.md",
        "ARCHITECTURE.md",
        "docs/evaluation/criteria.md",
    ];
    files
        .into_iter()
        .filter(|f| minimal_paths.contains(&f.rel_path.as_str()))
        .collect()
}

/// Get files that should be generated for the given set of AI tools.
pub fn filter_by_tools(
    files: Vec<RenderedFile>,
    tools: &[crate::types::AiTool],
) -> Vec<RenderedFile> {
    use crate::types::AiTool;
    files
        .into_iter()
        .filter(|f| {
            match f.rel_path.as_str() {
                "CLAUDE.md" => tools.contains(&AiTool::ClaudeCode),
                _ => true, // AGENTS.md and all other files are always included
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_context(stack: Stack) -> RenderContext {
        RenderContext {
            project_name: "test-project".to_string(),
            project_description: "A test project.".to_string(),
            date: "2026-04-03".to_string(),
            harn_version: "0.1.0".to_string(),
            stack,
        }
    }

    #[test]
    fn all_templates_embedded() {
        let manifest = template_manifest();
        for (template_path, _) in &manifest {
            assert!(
                TEMPLATES_DIR.get_file(template_path).is_some(),
                "Missing embedded template: {template_path}"
            );
        }
    }

    #[test]
    fn render_all_rust_stack() {
        let ctx = test_context(Stack::Rust);
        let files = render_all(&ctx).unwrap();

        assert_eq!(files.len(), 9, "Expected 9 rendered files");

        let agents = files.iter().find(|f| f.rel_path == "AGENTS.md").unwrap();
        assert!(agents.content.contains("test-project"));
        assert!(agents.content.contains("A test project."));
        assert!(agents.content.contains("cargo build"));

        let arch = files
            .iter()
            .find(|f| f.rel_path == "ARCHITECTURE.md")
            .unwrap();
        assert!(arch.content.contains("cargo clippy"));
        assert!(arch.content.contains("Common Mistakes"));

        let criteria = files
            .iter()
            .find(|f| f.rel_path == "docs/evaluation/criteria.md")
            .unwrap();
        assert!(criteria.content.contains("API Ergonomics"));
        assert!(!criteria.content.contains("Visual & UX Design"));
    }

    #[test]
    fn render_all_node_stack() {
        let ctx = test_context(Stack::Node);
        let files = render_all(&ctx).unwrap();

        let arch = files
            .iter()
            .find(|f| f.rel_path == "ARCHITECTURE.md")
            .unwrap();
        assert!(arch.content.contains("ESLint"));
        assert!(arch.content.contains("Common Mistakes"));

        let criteria = files
            .iter()
            .find(|f| f.rel_path == "docs/evaluation/criteria.md")
            .unwrap();
        assert!(criteria.content.contains("Visual & UX Design"));
        assert!(!criteria.content.contains("API Ergonomics"));
    }

    #[test]
    fn render_all_python_stack() {
        let ctx = test_context(Stack::Python);
        let files = render_all(&ctx).unwrap();

        let arch = files
            .iter()
            .find(|f| f.rel_path == "ARCHITECTURE.md")
            .unwrap();
        assert!(arch.content.contains("import linting"));
        assert!(arch.content.contains("Common Mistakes"));
    }

    #[test]
    fn render_all_go_stack() {
        let ctx = test_context(Stack::Go);
        let files = render_all(&ctx).unwrap();

        let arch = files
            .iter()
            .find(|f| f.rel_path == "ARCHITECTURE.md")
            .unwrap();
        assert!(arch.content.contains("go vet"));
        assert!(arch.content.contains("Common Mistakes"));
    }

    #[test]
    fn render_all_generic_stack() {
        let ctx = test_context(Stack::Generic);
        let files = render_all(&ctx).unwrap();

        let arch = files
            .iter()
            .find(|f| f.rel_path == "ARCHITECTURE.md")
            .unwrap();
        assert!(arch.content.contains("linting tools"));
        assert!(arch.content.contains("Common Mistakes"));
    }

    #[test]
    fn static_templates_are_verbatim() {
        let ctx = test_context(Stack::Rust);
        let files = render_all(&ctx).unwrap();

        let exec_plan = files
            .iter()
            .find(|f| f.rel_path == "docs/templates/exec-plan.md")
            .unwrap();
        assert!(exec_plan.content.contains("ExecPlan:"));
        assert!(exec_plan.content.contains("Living document"));
    }

    #[test]
    fn filter_minimal_keeps_only_essentials() {
        let ctx = test_context(Stack::Rust);
        let files = render_all(&ctx).unwrap();
        let minimal = filter_minimal(files);

        assert_eq!(minimal.len(), 3);
        let paths: Vec<&str> = minimal.iter().map(|f| f.rel_path.as_str()).collect();
        assert!(paths.contains(&"AGENTS.md"));
        assert!(paths.contains(&"ARCHITECTURE.md"));
        assert!(paths.contains(&"docs/evaluation/criteria.md"));
    }

    #[test]
    fn filter_by_tools_codex_only() {
        use crate::types::AiTool;
        let ctx = test_context(Stack::Rust);
        let files = render_all(&ctx).unwrap();
        let filtered = filter_by_tools(files, &[AiTool::Codex]);

        let paths: Vec<&str> = filtered.iter().map(|f| f.rel_path.as_str()).collect();
        assert!(paths.contains(&"AGENTS.md"));
        assert!(!paths.contains(&"CLAUDE.md"));
    }

    #[test]
    fn filter_by_tools_both() {
        use crate::types::AiTool;
        let ctx = test_context(Stack::Rust);
        let files = render_all(&ctx).unwrap();
        let filtered = filter_by_tools(files, &[AiTool::Codex, AiTool::ClaudeCode]);

        let paths: Vec<&str> = filtered.iter().map(|f| f.rel_path.as_str()).collect();
        assert!(paths.contains(&"AGENTS.md"));
        assert!(paths.contains(&"CLAUDE.md"));
    }
}

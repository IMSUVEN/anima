use std::fmt;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Deserializer, Serialize};

/// ASCII-only slug used in filenames for plans, sprints, etc.
/// Invariant: lowercase ASCII letters, digits, and hyphens only; no leading/trailing hyphens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Slug(String);

impl<'de> Deserialize<'de> for Slug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Slug::from_explicit(&s).map_err(serde::de::Error::custom)
    }
}

impl Slug {
    /// Build a slug from an explicit `--slug` value (light validation only).
    pub fn from_explicit(raw: &str) -> Result<Self> {
        let s = raw.trim().to_lowercase();
        if s.is_empty() {
            bail!("Slug cannot be empty. Provide a non-empty value with --slug.");
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            bail!("Slug must contain only lowercase ASCII letters, digits, and hyphens: {raw}. Use --slug with a valid value (e.g., --slug my-feature).");
        }
        let s = s.trim_matches('-').to_string();
        if s.is_empty() {
            bail!("Slug cannot be only hyphens. Provide a slug with at least one letter or digit.");
        }
        Ok(Self(s))
    }

    /// Derive a slug from a free-form description.
    /// Extracts ASCII characters, lowercases, replaces non-alphanumeric with hyphens,
    /// collapses consecutive hyphens, trims leading/trailing hyphens.
    /// Returns `None` if no usable ASCII characters remain.
    pub fn from_description(desc: &str) -> Option<Self> {
        let slug: String = desc
            .chars()
            .filter(|c| c.is_ascii())
            .collect::<String>()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect();

        let collapsed = collapse_hyphens(&slug);
        let trimmed = collapsed.trim_matches('-').to_string();

        if trimmed.is_empty() {
            None
        } else {
            Some(Self(trimmed))
        }
    }

    /// Generate a sequential fallback slug: `plan-001`, `plan-002`, etc.
    pub fn sequential(prefix: &str, dir: &Path) -> Result<Self> {
        for i in 1..=999 {
            let candidate = format!("{prefix}-{i:03}");
            let entries = dir.read_dir().with_context(|| {
                format!(
                    "Could not read directory {} for slug generation",
                    dir.display()
                )
            })?;
            let exists = entries.filter_map(|e| e.ok()).any(|e| {
                e.file_name()
                    .to_string_lossy()
                    .ends_with(&format!("-{candidate}.md"))
            });

            if !exists {
                return Ok(Self(candidate));
            }
        }
        bail!(
            "Could not generate sequential slug after 999 attempts in {}.\n\
             Clean up old plans/sprints or use --slug to specify a custom slug.",
            dir.display()
        );
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Project name — human-readable, used in templates and config.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            bail!("Project name cannot be empty. Use --name to specify a project name.");
        }
        Ok(Self(name.trim().to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for ProjectName {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self> {
        Self::new(s)
    }
}

impl From<ProjectName> for String {
    fn from(p: ProjectName) -> Self {
        p.0
    }
}

/// Date formatted as YYYY-MM-DD for filenames and config.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HarnDate(String);

impl HarnDate {
    pub fn today() -> Self {
        Self(chrono::Local::now().format("%Y-%m-%d").to_string())
    }

    /// Parse and validate a YYYY-MM-DD date string.
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            bail!("Date cannot be empty. Use YYYY-MM-DD format (e.g., 2026-04-07).");
        }
        if chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_err() {
            bail!("Invalid date \"{s}\". Use YYYY-MM-DD format (e.g., 2026-04-07).");
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for HarnDate {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self> {
        Self::parse(&s)
    }
}

impl From<HarnDate> for String {
    fn from(d: HarnDate) -> Self {
        d.0
    }
}

impl fmt::Display for HarnDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A resolved file path within the project, always relative to project root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct HarnPath(PathBuf);

impl<'de> Deserialize<'de> for HarnPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Self::new(path).map_err(serde::de::Error::custom)
    }
}

impl HarnPath {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        if path.is_absolute() {
            bail!(
                "HarnPath must be relative, got absolute path: {}",
                path.display()
            );
        }
        Ok(Self(path))
    }

    pub fn resolve(&self, root: &Path) -> PathBuf {
        root.join(&self.0)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl fmt::Display for HarnPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

/// Detected technology stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stack {
    Rust,
    Node,
    Python,
    Go,
    Generic,
}

impl Stack {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Node => "node",
            Self::Python => "python",
            Self::Go => "go",
            Self::Generic => "generic",
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Stack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "node" | "nodejs" | "javascript" | "typescript" => Ok(Self::Node),
            "python" => Ok(Self::Python),
            "go" | "golang" => Ok(Self::Go),
            "generic" => Ok(Self::Generic),
            other => {
                bail!("Unknown stack: {other}. Valid options: rust, node, python, go, generic")
            }
        }
    }
}

/// Supported AI coding tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiTool {
    ClaudeCode,
    Codex,
}

impl AiTool {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
        }
    }
}

impl fmt::Display for AiTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for AiTool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "claude-code" | "claude" => Ok(Self::ClaudeCode),
            "codex" => Ok(Self::Codex),
            other => bail!("Unknown AI tool: {other}. Valid options: claude-code, codex"),
        }
    }
}

/// Active sprint state, serialized to `.agents/harn/current-sprint.toml`.
/// Shared across plan, sprint, and status modules.
#[derive(Debug, Serialize, Deserialize)]
pub struct SprintState {
    pub name: String,
    pub slug: Slug,
    pub created: HarnDate,
    pub plan: Option<String>,
    pub contract_path: HarnPath,
}

fn collapse_hyphens(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_hyphen = false;
    for c in s.chars() {
        if c == '-' {
            if !prev_hyphen {
                result.push('-');
            }
            prev_hyphen = true;
        } else {
            result.push(c);
            prev_hyphen = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_from_description_basic() {
        let slug = Slug::from_description("implement login page").unwrap();
        assert_eq!(slug.as_str(), "implement-login-page");
    }

    #[test]
    fn slug_from_description_with_special_chars() {
        let slug = Slug::from_description("OAuth2 integration!").unwrap();
        assert_eq!(slug.as_str(), "oauth2-integration");
    }

    #[test]
    fn slug_from_description_non_ascii_only_returns_none() {
        assert!(Slug::from_description("用户认证").is_none());
    }

    #[test]
    fn slug_from_description_mixed_ascii_non_ascii() {
        let slug = Slug::from_description("用户auth认证flow").unwrap();
        assert_eq!(slug.as_str(), "authflow");
    }

    #[test]
    fn slug_from_explicit_valid() {
        let slug = Slug::from_explicit("auth-flow").unwrap();
        assert_eq!(slug.as_str(), "auth-flow");
    }

    #[test]
    fn slug_from_explicit_rejects_uppercase() {
        assert!(Slug::from_explicit("Auth-Flow").is_ok()); // lowercased
        assert_eq!(
            Slug::from_explicit("Auth-Flow").unwrap().as_str(),
            "auth-flow"
        );
    }

    #[test]
    fn slug_from_explicit_rejects_spaces() {
        assert!(Slug::from_explicit("auth flow").is_err());
    }

    #[test]
    fn slug_from_explicit_trims_hyphens() {
        let slug = Slug::from_explicit("-auth-flow-").unwrap();
        assert_eq!(slug.as_str(), "auth-flow");
    }

    #[test]
    fn slug_collapse_consecutive_hyphens() {
        let slug = Slug::from_description("hello   world").unwrap();
        assert_eq!(slug.as_str(), "hello-world");
    }

    #[test]
    fn project_name_rejects_empty() {
        assert!(ProjectName::new("").is_err());
        assert!(ProjectName::new("  ").is_err());
    }

    #[test]
    fn project_name_trims() {
        let name = ProjectName::new("  my-project  ").unwrap();
        assert_eq!(name.as_str(), "my-project");
    }

    #[test]
    fn stack_from_str_variants() {
        assert_eq!("rust".parse::<Stack>().unwrap(), Stack::Rust);
        assert_eq!("node".parse::<Stack>().unwrap(), Stack::Node);
        assert_eq!("nodejs".parse::<Stack>().unwrap(), Stack::Node);
        assert_eq!("python".parse::<Stack>().unwrap(), Stack::Python);
        assert_eq!("go".parse::<Stack>().unwrap(), Stack::Go);
        assert_eq!("golang".parse::<Stack>().unwrap(), Stack::Go);
        assert_eq!("generic".parse::<Stack>().unwrap(), Stack::Generic);
        assert!("unknown".parse::<Stack>().is_err());
    }

    #[test]
    fn ai_tool_from_str_variants() {
        assert_eq!("claude-code".parse::<AiTool>().unwrap(), AiTool::ClaudeCode);
        assert_eq!("claude".parse::<AiTool>().unwrap(), AiTool::ClaudeCode);
        assert_eq!("codex".parse::<AiTool>().unwrap(), AiTool::Codex);
        assert!("unknown".parse::<AiTool>().is_err());
    }

    #[test]
    fn harn_date_parse_valid() {
        let d = HarnDate::parse("2026-04-03").unwrap();
        assert_eq!(d.as_str(), "2026-04-03");
    }

    #[test]
    fn harn_date_parse_rejects_invalid() {
        assert!(HarnDate::parse("not-a-date").is_err());
        assert!(HarnDate::parse("").is_err());
        assert!(HarnDate::parse("2026-13-01").is_err());
    }

    #[test]
    fn harn_date_serde_roundtrip() {
        let d = HarnDate::parse("2026-04-03").unwrap();
        let json = serde_json::to_string(&d).unwrap();
        let parsed: HarnDate = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.as_str(), "2026-04-03");
    }

    #[test]
    fn harn_date_serde_rejects_invalid() {
        let result: Result<HarnDate, _> = serde_json::from_str("\"not-a-date\"");
        assert!(result.is_err());
    }

    #[test]
    fn harn_path_resolve() {
        let p = HarnPath::new("docs/design.md").unwrap();
        let resolved = p.resolve(Path::new("/tmp/project"));
        assert_eq!(resolved, PathBuf::from("/tmp/project/docs/design.md"));
    }
}

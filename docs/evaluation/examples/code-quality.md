# Calibration Examples: Code Quality

These examples illustrate what grade A, C, and F code looks like for the "Code Quality" criterion. Use them to calibrate your grading.

All examples implement the same concept: a validated slug type used in filenames.

---

## Grade A — Typed, validated, tested

From `src/types.rs` (actual harn code):

```rust
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
    pub fn from_explicit(raw: &str) -> Result<Self> {
        let s = raw.trim().to_lowercase();
        if s.is_empty() {
            bail!("Slug cannot be empty. Provide a non-empty value with --slug.");
        }
        if !s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            bail!("Slug must contain only lowercase ASCII letters, digits, and hyphens: \
                   {raw}. Use --slug with a valid value (e.g., --slug my-feature).");
        }
        let s = s.trim_matches('-').to_string();
        if s.is_empty() {
            bail!("Slug cannot be only hyphens. Provide a slug with at least one letter or digit.");
        }
        Ok(Self(s))
    }
}
```

**Why A**: The type wraps `String` so it cannot be confused with other strings. Validation happens at construction time — once you have a `Slug`, the invariant holds everywhere. Error messages include both what happened and what to do. `#[serde(transparent)]` preserves the slug as a plain string in serialized formats. Custom `Deserialize` rejects invalid slugs at deserialization time, maintaining the invariant even when loading from TOML.

---

## Grade C — Raw string, external validation

```rust
fn validate_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn create_plan(slug: String, description: &str) -> Result<()> {
    if !validate_slug(&slug) {
        bail!("Invalid slug");
    }
    let filename = format!("{slug}.md");
    // ...
}
```

**Why C**: Validation exists but is opt-in — callers must remember to call `validate_slug` before using the string. Nothing prevents passing an unvalidated `String` to `create_plan`. The error message says what failed but not what to do.

---

## Grade F — No validation, panics on bad input

```rust
fn create_plan(slug: String, description: &str) {
    let filename = format!("{slug}.md");
    std::fs::write(&filename, description).unwrap();
}
```

**Why F**: No validation at all. Accepts arbitrary strings including path traversals (`../../etc/passwd`), empty strings, or Unicode. Uses `unwrap()` which panics on I/O errors instead of returning actionable errors. No type safety — any `String` is treated as a valid slug.

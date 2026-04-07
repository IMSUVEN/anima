use sha2::{Digest, Sha256};

pub fn sha256_hex(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Extract markdown link targets from a line: `[text](target)` → `target`.
pub fn extract_md_links(line: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find("](") {
        let after = &rest[open + 2..];
        if let Some(close) = after.find(')') {
            links.push(after[..close].to_string());
            rest = &after[close + 1..];
        } else {
            break;
        }
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_md_links_basic() {
        let links = extract_md_links("See [ARCHITECTURE.md](ARCHITECTURE.md) for details.");
        assert_eq!(links, vec!["ARCHITECTURE.md"]);
    }

    #[test]
    fn extract_md_links_multiple() {
        let links = extract_md_links("| [A](a.md) | [B](docs/b.md) |");
        assert_eq!(links, vec!["a.md", "docs/b.md"]);
    }

    #[test]
    fn extract_md_links_none() {
        let links = extract_md_links("No links here.");
        assert!(links.is_empty());
    }

    #[test]
    fn extract_md_links_url() {
        let links = extract_md_links("[link](https://example.com)");
        assert_eq!(links, vec!["https://example.com"]);
    }

    #[test]
    fn sha256_hex_deterministic() {
        let h1 = sha256_hex("hello");
        let h2 = sha256_hex("hello");
        assert_eq!(h1, h2);
        assert_ne!(sha256_hex("hello"), sha256_hex("world"));
    }
}

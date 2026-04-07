use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use console::style;

use crate::types::HarnDate;

const SCORE_PATH: &str = "docs/QUALITY_SCORE.md";

pub fn show(project_root: &Path) -> Result<()> {
    let path = project_root.join(SCORE_PATH);
    if !path.exists() {
        println!();
        println!("No quality assessments yet.");
        println!("Run `harn score update` to perform the first assessment.");
        return Ok(());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not read {SCORE_PATH}. Check file permissions."))?;
    println!();
    println!("{}", style("Quality Scores").bold());
    println!();

    let mut in_table = false;
    for line in content.lines() {
        if line.starts_with("| Domain") {
            in_table = true;
        }
        if in_table {
            println!("{line}");
            if line.trim().is_empty() {
                in_table = false;
            }
        }
        if line.starts_with("Last updated:") {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn update(project_root: &Path) -> Result<()> {
    let domains = read_domains(project_root);
    let date = HarnDate::today();

    println!();
    println!("{}", style("Quality Score Update").bold());
    println!();

    if domains.is_empty() {
        println!("No domains found in ARCHITECTURE.md.");
        println!("Define domains in ARCHITECTURE.md first, or enter them now.");
    }

    let domains_to_score = if domains.is_empty() {
        prompt_domains()?
    } else {
        println!("Domains from ARCHITECTURE.md:");
        for d in &domains {
            println!("  • {d}");
        }
        println!();
        domains
    };

    if domains_to_score.is_empty() {
        println!("No domains to score. Exiting.");
        return Ok(());
    }

    let mut rows = Vec::new();
    for domain in &domains_to_score {
        println!("{}", style(format!("--- {domain} ---")).bold());
        let functionality = prompt_grade("Functionality")?;
        let depth = prompt_grade("Product Depth")?;
        let quality = prompt_grade("Code Quality")?;
        let design = prompt_grade("API Ergonomics")?;
        let overall = compute_overall(&functionality, &depth, &quality, &design);
        rows.push(ScoreRow {
            domain: domain.clone(),
            functionality,
            depth,
            quality,
            design,
            overall,
            date: date.as_str().to_string(),
        });
        println!();
    }

    let content = render_score_file(&rows, &date);
    let path = project_root.join(SCORE_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, &content)
        .with_context(|| format!("Could not write {SCORE_PATH}. Check filesystem permissions."))?;

    println!("Updated: {SCORE_PATH}");
    println!();
    for row in &rows {
        println!("  {} — Overall: {}", row.domain, style(&row.overall).bold());
    }

    Ok(())
}

struct ScoreRow {
    domain: String,
    functionality: String,
    depth: String,
    quality: String,
    design: String,
    overall: String,
    date: String,
}

fn read_domains(project_root: &Path) -> Vec<String> {
    let arch_path = project_root.join("ARCHITECTURE.md");
    let content = match fs::read_to_string(arch_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut domains = Vec::new();
    let mut in_domain_table = false;
    for line in content.lines() {
        if line.contains("Domain") && line.contains("Description") && line.contains('|') {
            in_domain_table = true;
            continue;
        }
        if in_domain_table {
            if line.starts_with('|') && !line.contains("---") {
                let cols: Vec<&str> = line.split('|').collect();
                if cols.len() >= 3 {
                    let domain = cols[1].trim();
                    if !domain.is_empty() {
                        domains.push(domain.to_string());
                    }
                }
            } else if !line.starts_with('|') {
                in_domain_table = false;
            }
        }
    }
    domains
}

fn prompt_domains() -> Result<Vec<String>> {
    use dialoguer::Input;
    let input: String = Input::new()
        .with_prompt("Enter domain names (comma-separated)")
        .interact_text()?;
    Ok(input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

fn prompt_grade(criterion: &str) -> Result<String> {
    use dialoguer::Select;
    let grades = ["A", "B", "C", "D", "F"];
    let idx = Select::new()
        .with_prompt(format!("  {criterion}"))
        .items(&grades)
        .default(1) // Default to B
        .interact()?;
    Ok(grades[idx].to_string())
}

fn compute_overall(func: &str, depth: &str, quality: &str, design: &str) -> String {
    let to_num = |g: &str| match g {
        "A" => 4.0,
        "B" => 3.0,
        "C" => 2.0,
        "D" => 1.0,
        "F" => 0.0,
        _ => 2.0,
    };
    // Weighted: Functionality(3) + Depth(3) + Quality(2) + Design(2) = 10
    let weighted =
        to_num(func) * 3.0 + to_num(depth) * 3.0 + to_num(quality) * 2.0 + to_num(design) * 2.0;
    let avg = weighted / 10.0;

    if avg >= 3.5 {
        "A".to_string()
    } else if avg >= 2.5 {
        "B".to_string()
    } else if avg >= 1.5 {
        "C".to_string()
    } else if avg >= 0.5 {
        "D".to_string()
    } else {
        "F".to_string()
    }
}

fn render_score_file(rows: &[ScoreRow], date: &HarnDate) -> String {
    let mut out = String::new();
    out.push_str("# Quality Scores\n\n");
    out.push_str(&format!("Last updated: {date}\n\n"));
    out.push_str("Grade each domain on the [evaluation criteria](evaluation/criteria.md). Update scores when significant changes land.\n\n");
    out.push_str("| Domain | Functionality | Product Depth | Code Quality | Design/UX | Overall | Last Assessed |\n");
    out.push_str("|--------|:---:|:---:|:---:|:---:|:---:|:---:|\n");
    for row in rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.domain,
            row.functionality,
            row.depth,
            row.quality,
            row.design,
            row.overall,
            row.date
        ));
    }
    out.push_str("\n## History\n\n");
    out.push_str("| Date | Domain | Change | Notes |\n");
    out.push_str("|------|--------|--------|-------|\n");
    out.push_str(&format!("| {} | all | Initial assessment | |\n", date));
    out
}

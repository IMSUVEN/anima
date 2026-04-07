use anyhow::{Context, Result};
use console::style;
use std::path::Path;

use super::{Assessment, Obligation, Status};

pub(super) fn print_report(root: &Path, assessments: &[Assessment]) {
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

    println!(
        "{} ({}%)",
        style("Level 1 — Single Agent + Loop").bold().underlined(),
        l1_score
    );
    println!();
    print_level_assessments(&l1);

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

pub(super) fn level_score(assessments: &[&Assessment]) -> u32 {
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

pub(super) fn print_json(root: &Path, assessments: &[Assessment]) -> Result<()> {
    let project_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

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

    let checks: Vec<serde_json::Value> = assessments
        .iter()
        .map(|a| {
            serde_json::json!({
                "category": a.category,
                "requirement": a.requirement,
                "level": a.level,
                "obligation": match a.obligation {
                    Obligation::Must => "must",
                    Obligation::Should => "should",
                },
                "status": match a.status {
                    Status::Pass => "pass",
                    Status::Partial => "partial",
                    Status::Fail => "fail",
                },
                "detail": a.detail,
            })
        })
        .collect();

    let output = serde_json::json!({
        "project": project_name,
        "level": overall_level,
        "level1_pct": l1_score,
        "level2_pct": l2_score,
        "checks": checks,
    });

    let json = serde_json::to_string_pretty(&output)
        .context("Failed to serialize assessment to JSON. This is a bug — please report it.")?;
    println!("{json}");
    Ok(())
}

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

use crate::agents::agent_def::AgentDef;
use crate::agents::skill_def::{SkillDef, SkillMeta};

pub fn load_agent(repo: &Path, name: &str) -> Result<AgentDef> {
    let path = repo.join("agents").join(format!("{name}.yaml"));
    let text = fs::read_to_string(&path)
        .with_context(|| format!("reading agent {}", path.display()))?;
    let agent: AgentDef = serde_yaml::from_str(&text)
        .with_context(|| format!("parsing agent YAML {}", path.display()))?;
    Ok(agent)
}

pub fn load_skill(repo: &Path, slug: &str) -> Result<SkillDef> {
    // prefer skills/<slug>.md
    let path = repo.join("skills").join(format!("{slug}.md"));
    let text = fs::read_to_string(&path)
        .with_context(|| format!("reading skill {}", path.display()))?;

    // Parse optional YAML frontmatter.
    // If frontmatter exists, we use its slug if present; else fallback to filename slug.
    let (meta, body) = split_frontmatter(&text)?;
    let fm_slug = meta.slug.clone().unwrap_or_else(|| slug.to_string());

    Ok(SkillDef {
        slug: fm_slug,
        meta,
        body,
    })
}

// Very small frontmatter splitter: looks for leading --- ... ---
// If not present, meta is default and body is entire file.
fn split_frontmatter(text: &str) -> Result<(SkillMeta, String)> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return Ok((SkillMeta::default(), text.to_string()));
    }

    // Find the second '---' line.
    let mut lines = trimmed.lines();
    let first = lines.next().unwrap_or("");
    if first.trim() != "---" {
        return Ok((SkillMeta::default(), text.to_string()));
    }

    let mut fm_lines: Vec<&str> = Vec::new();
    for line in lines.by_ref() {
        if line.trim() == "---" {
            // remainder is body
            let body: String = lines.collect::<Vec<&str>>().join("\n");
            let fm_text = fm_lines.join("\n");
            let meta: SkillMeta = serde_yaml::from_str(&fm_text)
                .map_err(|e| anyhow!("invalid skill frontmatter: {e}"))?;
            return Ok((meta, body));
        }
        fm_lines.push(line);
    }

    Err(anyhow!("unterminated frontmatter block"))
}

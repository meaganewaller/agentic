use anyhow::{anyhow, Result};
use serde_json::Value;
use std::path::PathBuf;

use crate::agents::loader::{load_agent, load_skill};

#[derive(Debug, Clone)]
pub struct ResolvedAgent {
    pub name: String,
    pub system_prompt: String,
}

pub fn resolve_active_agent(merged: &Value) -> Result<Option<String>> {
    Ok(merged
        .get("agent")
        .and_then(|a| a.get("active"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

pub fn resolve_agents_repo(merged: &Value) -> Result<Option<PathBuf>> {
    Ok(merged
        .get("agents_repo")
        .and_then(|v| v.as_str())
        .map(PathBuf::from))
}

pub fn resolve_agent_bundle(merged: &Value) -> Result<Option<ResolvedAgent>> {
    let agent_name = match resolve_active_agent(merged)? {
        Some(a) => a,
        None => return Ok(None),
    };

    let repo = resolve_agents_repo(merged)?
        .ok_or_else(|| anyhow!("agent.active is set but agents_repo is missing"))?;

    let agent = load_agent(&repo, &agent_name)?;

    // Build final system prompt:
    // 1) agent system_prompt
    // 2) concatenated skill bodies
    // 3) optional config-level append
    let mut parts: Vec<String> = Vec::new();

    if let Some(sp) = agent.system_prompt.clone() {
        parts.push(sp.trim().to_string());
    }

    if !agent.skills.is_empty() {
        parts.push("## Skills".to_string());
        for slug in agent.skills.iter() {
            let skill = load_skill(&repo, slug)?;
            parts.push(format!("### {}\n{}", skill.slug, skill.body.trim()));
        }
    }

    // Allow config to append extra prompt rules
    if let Some(extra) = merged
        .get("agents")
        .and_then(|a| a.get("system_prompt_append"))
        .and_then(|v| v.as_str())
    {
        parts.push("## Local Overrides".to_string());
        parts.push(extra.trim().to_string());
    }

    Ok(Some(ResolvedAgent {
        name: agent.name,
        system_prompt: parts.join("\n\n"),
    }))
}

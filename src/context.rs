use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn detect_hostname() -> Result<String> {
    let raw = hostname::get()?
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

    // Normalize: remove domain suffix if present
    let normalized = raw
        .split('.')
        .next()
        .unwrap_or("unknown")
        .to_lowercase();

    Ok(normalized)
}

pub fn detect_project_config() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;

    // Check current directory only (v0.1 simple behavior)
    let explicit = cwd.join("agentic.yaml");
    if explicit.exists() {
        return Some(explicit);
    }

    let hidden = cwd.join(".agentic.yaml");
    if hidden.exists() {
        return Some(hidden);
    }

    None
}

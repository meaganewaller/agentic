use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub fn write_json_to_path(path: &Path, value: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Creating directory {}", parent.display()))?;
    }

    let content = serde_json::to_string_pretty(value)?;
    fs::write(path, content)
        .with_context(|| format!("Writing file {}", path.display()))?;

    Ok(())
}

pub fn default_claude_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    Ok(home.join(".claude").join("settings.json"))
}

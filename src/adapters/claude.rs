use anyhow::{anyhow, Result};
use dirs::home_dir;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::adapter::VendorAdapter;

pub struct ClaudeAdapter;

impl VendorAdapter for ClaudeAdapter {
    fn name(&self) -> &'static str {
        "claude"
    }

    fn compile(&self, merged: &Value) -> Result<Value> {
        let claude = merged
            .get("vendors")
            .and_then(|v| v.get("claude"))
            .ok_or_else(|| anyhow!("No vendors.claude section found"))?;

        Ok(json!({
            "enabled": claude.get("enabled").cloned().unwrap_or(json!(true)),
            "model": claude.get("model").cloned().unwrap_or(json!("claude-3-opus")),
            "temperature": claude.get("temperature").cloned().unwrap_or(json!(0.2)),
            "allow_shell": claude.get("allow_shell").cloned().unwrap_or(json!(false)),
            "agentic_version": "0.1.0"
        }))
    }

    fn default_output_path(&self) -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(home.join(".claude").join("settings.json"))
    }
}

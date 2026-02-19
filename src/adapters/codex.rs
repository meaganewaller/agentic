use anyhow::{anyhow, Result};
use dirs::home_dir;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::adapter::VendorAdapter;

pub struct CodexAdapter;

impl VendorAdapter for CodexAdapter {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn compile(&self, merged: &Value) -> Result<Value> {
        let codex = merged
            .get("vendors")
            .and_then(|v| v.get("codex"))
            .ok_or_else(|| anyhow!("No vendors.codex section found"))?;

        Ok(json!({
            "model": codex.get("model").cloned().unwrap_or(json!("gpt-4.1")),
            "temperature": codex.get("temperature").cloned().unwrap_or(json!(0.2)),
            "allow_shell": codex.get("allow_shell").cloned().unwrap_or(json!(false)),
            "agentic_version": "0.1.0"
        }))
    }

    fn default_output_path(&self) -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(home.join(".codex").join("config.json"))
    }
}

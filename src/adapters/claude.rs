use anyhow::{anyhow, Result};
use dirs::home_dir;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::adapter::{CompileInput, VendorAdapter};

pub struct ClaudeAdapter;

impl VendorAdapter for ClaudeAdapter {
    fn name(&self) -> &'static str {
        "claude"
    }

    fn compile(&self, input: CompileInput<'_>) -> Result<Value> {
        let merged = input.merged;

        let claude = merged
            .get("vendors")
            .and_then(|v| v.get("claude"))
            .ok_or_else(|| anyhow!("No vendors.claude section found"))?;

        let mut out = json!({
            "enabled": claude.get("enabled").cloned().unwrap_or(json!(true)),
            "model": claude.get("model").cloned().unwrap_or(json!("claude-3-opus")),
            "temperature": claude.get("temperature").cloned().unwrap_or(json!(0.2)),
            "allow_shell": claude.get("allow_shell").cloned().unwrap_or(json!(false)),
            "agentic_version": "0.1.0"
        });

        if let Some(prompt) = input.resolved_agent_prompt {
            out["system_prompt"] = Value::String(prompt.to_string());
        }

        Ok(out)
    }

    fn default_output_path(&self) -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(home.join(".claude").join("settings.json"))
    }
}

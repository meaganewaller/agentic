use anyhow::{anyhow, Result};
use dirs::home_dir;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::adapter::{CompileInput, VendorAdapter};

pub struct CodexAdapter;

impl VendorAdapter for CodexAdapter {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn compile(&self, input: CompileInput<'_>) -> Result<Value> {
        let merged = input.merged;
        let codex = merged
            .get("vendors")
            .and_then(|v| v.get("codex"))
            .ok_or_else(|| anyhow!("No vendors.codex section found"))?;

        let mut out = json!({
            "enabled": codex.get("enabled").cloned().unwrap_or(json!(true)),
            "model": codex.get("model").cloned().unwrap_or(json!("gpt-4.1")),
            "temperature": codex.get("temperature").cloned().unwrap_or(json!(0.2)),
            "allow_shell": codex.get("allow_shell").cloned().unwrap_or(json!(false)),
            "agentic_version": "0.1.0",
        });

        if let Some(prompt) = input.resolved_agent_prompt {
            out["system_prompt"] = Value::String(prompt.to_string());
        }

        Ok(out)
    }

    fn default_output_path(&self) -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(home.join(".codex").join("config.json"))
    }
}

use anyhow::{anyhow, Result};
use serde_json::{json, Value};

pub fn compile(merged: &Value) -> Result<Value> {
    let claude = merged
        .get("vendors")
        .and_then(|v| v.get("claude"))
        .ok_or_else(|| anyhow!("No vendors.claude section found"))?;

    let enabled = claude.get("enabled").cloned().unwrap_or(json!(true));
    let model = claude.get("model").cloned().unwrap_or(json!("claude-3-opus"));
    let temperature = claude.get("temperature").cloned().unwrap_or(json!(0.2));
    let allow_shell = claude.get("allow_shell").cloned().unwrap_or(json!(false));

    Ok(json!({
        "enabled": enabled,
        "model": model,
        "temperature": temperature,
        "allow_shell": allow_shell,
        "agentic_version": "0.1.0"
    }))
}

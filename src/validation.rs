use anyhow::{anyhow, Result};
use serde_json::Value;

use jsonschema::validator_for;

pub fn validate_config(merged: &Value) -> Result<()> {
    let schema_text = include_str!("../schema/agentic.schema.json");

    let schema_json: Value = serde_json::from_str(schema_text)?;

    let validator = validator_for(&schema_json)
        .map_err(|e| anyhow!("Failed to compile schema: {e}"))?;

    let errors: Vec<String> = validator
        .iter_errors(merged)
        .map(|e| e.to_string())
        .collect();

    if !errors.is_empty() {
        return Err(anyhow!(
            "Configuration validation failed:\n{}",
            errors.join("\n")
        ));
    }

    Ok(())
}

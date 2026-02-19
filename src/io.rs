use anyhow::{Context, Result};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::Path;

pub fn read_yaml(path: &Path) -> Result<YamlValue> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    let val: YamlValue = serde_yaml::from_str(&text)
        .with_context(|| format!("parsing YAML {}", path.display()))?;
    Ok(val)
}

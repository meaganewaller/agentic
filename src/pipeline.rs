use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;

use crate::config::layer::LayerSpec;
use crate::convert::yaml_to_json;
use crate::io::read_yaml;
use crate::merge::deep_merge;

pub fn build_merged_config(
    layers_dir: &str,
    profile: Option<&str>,
    machine: Option<&str>,
    project: Option<&str>,
) -> Result<Value> {
    let layers_dir = PathBuf::from(layers_dir);

    let spec = LayerSpec {
        base: layers_dir.join("base.yaml"),
        profile: profile.map(|p| layers_dir.join(format!("profile-{p}.yaml"))),
        machine: machine.map(|m| layers_dir.join(format!("machine-{m}.yaml"))),
        project: project.map(PathBuf::from),
    };

    let mut merged = Value::Object(Default::default());

    for path in spec.ordered_paths() {
        if !path.exists() {
            continue;
        }

        let yaml = read_yaml(&path)?;
        let json = yaml_to_json(yaml);
        merged = deep_merge(merged, json);
    }

    Ok(merged)
}

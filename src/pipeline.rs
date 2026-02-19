use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;

use crate::config::layer::LayerSpec;
use crate::convert::yaml_to_json;
use crate::io::read_yaml;
use crate::merge::deep_merge;
use crate::validation::validate_config;

pub struct PipelineOutput {
    pub merged: Value,
    pub resolved_agent_prompt: Option<String>,
    pub layers: Vec<(String, Value)>, // (filename, json value)
}

pub fn build_output(
    layers_dir: &str,
    profile: Option<&str>,
    machine: Option<&str>,
    project: Option<&str>,
) -> Result<PipelineOutput> {
    let (merged, layers) = build_with_layers(layers_dir, profile, machine, project)?;

    // Resolve agent bundle (optional)
    let resolved = crate::agents::resolver::resolve_agent_bundle(&merged)?
        .map(|ra| ra.system_prompt);

    Ok(PipelineOutput {
        merged,
        resolved_agent_prompt: resolved,
        layers,
    })
}

fn build_with_layers(
    layers_dir: &str,
    profile: Option<&str>,
    machine: Option<&str>,
    project: Option<&str>,
) -> Result<(Value, Vec<(String, Value)>)> {
    let layers_dir = PathBuf::from(layers_dir);

    let spec = LayerSpec {
        base: layers_dir.join("base.yaml"),
        profile: profile.map(|p| layers_dir.join(format!("profile-{p}.yaml"))),
        machine: machine.map(|m| layers_dir.join(format!("machine-{m}.yaml"))),
        project: project.map(PathBuf::from),
    };

    let mut merged = Value::Object(Default::default());
    let mut layers: Vec<(String, Value)> = Vec::new();

    for path in spec.ordered_paths() {
        if !path.exists() {
            continue;
        }

        let yaml = read_yaml(&path)?;
        let json = yaml_to_json(yaml);

        layers.push((
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            json.clone(),
        ));

        merged = deep_merge(merged, json);
        validate_config(&merged)?;
    }

    Ok((merged, layers))
}

pub fn build_merged_config(
    layers_dir: &str,
    profile: Option<&str>,
    machine: Option<&str>,
    project: Option<&str>,
) -> Result<Value> {
    let (merged, _) = build_with_layers(layers_dir, profile, machine, project)?;
    Ok(merged)
}


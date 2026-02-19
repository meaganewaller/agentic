use std::fs;
use std::path::Path;

use serde_json::Value;

#[test]
fn full_pipeline_builds_expected_claude_config() {
    // Create a temporary layers directory
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let layers_path = temp_dir.path();

    // Write base.yaml
    fs::write(
        layers_path.join("base.yaml"),
        r#"
vendors:
  claude:
    enabled: true
    model: "claude-3-opus"
    temperature: 0.2
"#,
    )
    .unwrap();

    // Write profile-work.yaml
    fs::write(
        layers_path.join("profile-work.yaml"),
        r#"
vendors:
  claude:
    temperature: 0.0
"#,
    )
    .unwrap();

    let merged = agentic::pipeline::build_merged_config(
        layers_path.to_str().unwrap(),
        Some("work"),
        None,
        None,
    )
    .unwrap();

    let compiled = agentic::adapters::claude::compile(&merged).unwrap();

    assert_eq!(compiled["model"], "claude-3-opus");
    assert_eq!(compiled["temperature"], 0.0);
    assert_eq!(compiled["enabled"], true);
}

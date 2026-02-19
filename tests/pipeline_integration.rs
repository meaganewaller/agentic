use std::fs;

use agentic::adapters::adapter::VendorAdapter;
use tempfile::tempdir;

#[test]
fn full_pipeline_merges_layers_and_compiles_multiple_vendors() {
    // Create temp directory
    let temp = tempdir().expect("create temp dir");
    let layers_path = temp.path();

    // ------------------------
    // Write base.yaml
    // ------------------------
    fs::write(
        layers_path.join("base.yaml"),
        r#"
vendors:
  claude:
    enabled: true
    model: "claude-3-opus"
    temperature: 0.2

  codex:
    model: "gpt-4.1"
    temperature: 0.3
"#,
    )
    .unwrap();

    // ------------------------
    // Write profile-work.yaml
    // ------------------------
    fs::write(
        layers_path.join("profile-work.yaml"),
        r#"
vendors:
  claude:
    temperature: 0.0
"#,
    )
    .unwrap();

    // ------------------------
    // Build merged config
    // ------------------------
    let merged = agentic::pipeline::build_merged_config(
        layers_path.to_str().unwrap(),
        Some("work"),
        None,
        None,
    )
    .expect("merge should succeed");

    // ------------------------
    // Claude compilation
    // ------------------------
    let claude_adapter = agentic::adapters::claude::ClaudeAdapter;
    let claude_compiled = claude_adapter.compile(&merged).unwrap();

    assert_eq!(claude_compiled["enabled"], true);
    assert_eq!(claude_compiled["model"], "claude-3-opus");
    assert_eq!(claude_compiled["temperature"], 0.0); // overridden by profile

    // ------------------------
    // Codex compilation
    // ------------------------
    let codex_adapter = agentic::adapters::codex::CodexAdapter;
    let codex_compiled = codex_adapter.compile(&merged).unwrap();

    assert_eq!(codex_compiled["model"], "gpt-4.1");
    assert_eq!(codex_compiled["temperature"], 0.3);

    // ------------------------
    // Ensure agentic version injected
    // ------------------------
    assert_eq!(claude_compiled["agentic_version"], "0.1.0");
    assert_eq!(codex_compiled["agentic_version"], "0.1.0");
}

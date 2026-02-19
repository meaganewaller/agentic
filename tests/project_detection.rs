use std::fs;
use tempfile::tempdir;

#[test]
fn detects_agentic_yaml_in_current_directory() {
    let temp = tempdir().unwrap();
    let project_dir = temp.path();

    // Write project config
    fs::write(
        project_dir.join("agentic.yaml"),
        r#"
vendors:
  claude:
    temperature: 0.0
"#,
    )
    .unwrap();

    // Change cwd for test
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(project_dir).unwrap();

    let detected = agentic::context::detect_project_config();
    assert!(detected.is_some());

    std::env::set_current_dir(original).unwrap();
}

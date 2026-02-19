use std::fs;
use tempfile::tempdir;

use agentic::adapters::adapter::VendorAdapter;
use agentic::adapters::claude::ClaudeAdapter;

#[test]
fn resolves_agent_and_skills_into_system_prompt() {
    // ----------------------------
    // 1. Create temporary agents repo
    // ----------------------------
    let repo_dir = tempdir().unwrap();
    let repo_path = repo_dir.path();

    fs::create_dir_all(repo_path.join("agents")).unwrap();
    fs::create_dir_all(repo_path.join("skills")).unwrap();

    // ----------------------------
    // 2. Write agent definition
    // ----------------------------
    fs::write(
        repo_path.join("agents").join("rails-architect.yaml"),
        r#"
name: rails-architect
description: Senior Rails architect

system_prompt: |
  You are a strict Rails architect.

skills:
  - tdd
"#,
    )
    .unwrap();

    // ----------------------------
    // 3. Write skill with frontmatter
    // ----------------------------
    fs::write(
        repo_path.join("skills").join("tdd.md"),
        r#"
---
slug: tdd
version: 1
---

# TDD Rules

1. Write a failing test.
2. Make it pass.
3. Refactor.
"#,
    )
    .unwrap();

    // ----------------------------
    // 4. Create layered config
    // ----------------------------
    let layers_dir = tempdir().unwrap();
    let layers_path = layers_dir.path();

    fs::write(
        layers_path.join("base.yaml"),
        format!(
            r#"
agents_repo: "{}"

agent:
  active: "rails-architect"

vendors:
  claude:
    model: "claude-3-opus"
"#,
            repo_path.display()
        ),
    )
    .unwrap();

    // ----------------------------
    // 5. Run full pipeline
    // ----------------------------
    let output = agentic::pipeline::build_output(
        layers_path.to_str().unwrap(),
        None,
        None,
        None,
    )
    .unwrap();

    // ----------------------------
    // 6. Ensure adapter embeds prompt (use as_ref() before moving resolved_agent_prompt)
    // ----------------------------
    let adapter = ClaudeAdapter;

    let compiled = adapter
        .compile(agentic::adapters::adapter::CompileInput {
            merged: &output.merged,
            resolved_agent_prompt: output.resolved_agent_prompt.as_ref(),
        })
        .unwrap();

    assert!(compiled["system_prompt"]
        .as_str()
        .unwrap()
        .contains("You are a strict Rails architect."));

    // Ensure agent was resolved
    let prompt = output
        .resolved_agent_prompt
        .expect("agent should resolve");

    assert!(prompt.contains("You are a strict Rails architect."));
    assert!(prompt.contains("# TDD Rules"));
    assert!(prompt.contains("Write a failing test."));
}

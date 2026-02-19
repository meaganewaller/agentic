use serde_json::json;
use agentic::merge::deep_merge;

#[test]
fn merges_agents_by_name() {
    let base = json!({
        "agents": [
            { "name": "rails", "temperature": 0.2 },
            { "name": "python", "temperature": 0.3 }
        ]
    });

    let override_layer = json!({
        "agents": [
            { "name": "rails", "temperature": 0.0 }
        ]
    });

    let merged = deep_merge(base, override_layer);

    let agents = merged["agents"].as_array().unwrap();

    let rails = agents.iter().find(|a| a["name"] == "rails").unwrap();
    let python = agents.iter().find(|a| a["name"] == "python").unwrap();

    assert_eq!(rails["temperature"], 0.0);
    assert_eq!(python["temperature"], 0.3);
}

use serde_json::json;

#[test]
fn deep_merges_objects_and_overrides_scalars() {
    let a = json!({"x": 1, "o": {"a": 1, "b": 2}});
    let b = json!({"x": 2, "o": {"b": 999, "c": 3}});
    let merged = agentic::merge::deep_merge(a, b);
    assert_eq!(merged, json!({"x": 2, "o": {"a": 1, "b": 999, "c": 3}}));
}

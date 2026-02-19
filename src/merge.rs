use serde_json::Value;

pub fn deep_merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut a_map), Value::Object(b_map)) => {
            for (k, v) in b_map {
                let old = a_map.remove(&k);

                let merged_value = match (old, v) {
                    (Some(Value::Array(a_arr)), Value::Array(b_arr))
                        if is_keyed_array(&k) =>
                    {
                        Value::Array(merge_keyed_arrays(a_arr, b_arr))
                    }
                    (Some(prev), new) => deep_merge(prev, new),
                    (None, new) => new,
                };

                a_map.insert(k, merged_value);
            }

            Value::Object(a_map)
        }
        (_, b) => b,
    }
}

fn is_keyed_array(key: &str) -> bool {
    matches!(key, "agents" | "skills")
}

fn merge_keyed_arrays(a: Vec<Value>, b: Vec<Value>) -> Vec<Value> {
    let mut result = a;

    for new_item in b {
        if let Some(name) = new_item.get("name").and_then(|v| v.as_str()) {
            let mut replaced = false;

            for existing in &mut result {
                if existing
                    .get("name")
                    .and_then(|v| v.as_str())
                    == Some(name)
                {
                    let merged = deep_merge(existing.clone(), new_item.clone());
                    *existing = merged;
                    replaced = true;
                    break;
                }
            }

            if !replaced {
                result.push(new_item);
            }
        } else {
            result.push(new_item);
        }
    }

    result
}

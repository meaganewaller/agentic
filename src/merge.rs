use serde_json::Value;

pub fn deep_merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut a_map), Value::Object(b_map)) => {
            for (k, v) in b_map {
                let old = a_map.remove(&k);
                a_map.insert(k, match old {
                    Some(prev) => deep_merge(prev, v),
                    None => v,
                });
            }
            Value::Object(a_map)
        }
        // v0.1: arrays + scalars override
        (_, b) => b,
    }
}

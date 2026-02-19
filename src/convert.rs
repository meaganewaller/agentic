use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

pub fn yaml_to_json(y: YamlValue) -> JsonValue {
    match y {
        YamlValue::Null => JsonValue::Null,
        YamlValue::Bool(b) => JsonValue::Bool(b),
        YamlValue::Number(n) => {
            if let Some(i) = n.as_i64() { JsonValue::from(i) }
            else if let Some(f) = n.as_f64() { JsonValue::from(f) }
            else { JsonValue::Null }
        }
        YamlValue::String(s) => JsonValue::String(s),
        YamlValue::Sequence(seq) => JsonValue::Array(seq.into_iter().map(yaml_to_json).collect()),
        YamlValue::Mapping(map) => {
            let mut m = serde_json::Map::new();
            for (k, v) in map {
                let key = match k {
                    YamlValue::String(s) => s,
                    _ => {
                        panic!("YAML mapping keys must be strings. Found non-string key: {:?}", k);
                    }
                };
                m.insert(key, yaml_to_json(v));
            }
            JsonValue::Object(m)
        }
        
        _ => JsonValue::Null,
    }
}

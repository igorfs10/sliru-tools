use serde::Serialize;
use serde_json::Value;
use yaml_rust2::YamlLoader;

use crate::services::json_converter;

pub fn yaml_to_json(yaml_str: &str) -> Result<String, String> {
    let docs = YamlLoader::load_from_str(yaml_str)
        .map_err(|e| format!("Erro ao fazer parse do YAML: {}", e))?;
    if docs.is_empty() {
        return Err("YAML vazio".to_string());
    }
    let doc = &docs[0];
    // Converte Yaml para Value
    let value = yaml_to_json_value(doc);
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value
        .serialize(&mut ser)
        .map_err(|e| format!("Erro ao serializar JSON: {}", e))?;
    String::from_utf8(buf).map_err(|e| format!("Erro ao converter JSON para UTF-8: {}", e))
}

fn yaml_to_json_value(yaml: &yaml_rust2::Yaml) -> Value {
    use yaml_rust2::Yaml;
    match yaml {
        Yaml::Null => Value::Null,
        Yaml::Boolean(b) => Value::Bool(*b),
        Yaml::Integer(i) => Value::Number((*i).into()),
        Yaml::Real(s) => serde_json::Number::from_f64(s.parse::<f64>().unwrap_or(0.0))
            .map(Value::Number)
            .unwrap_or(Value::Null),
        Yaml::String(s) => Value::String(s.clone()),
        Yaml::Array(arr) => Value::Array(arr.iter().map(yaml_to_json_value).collect()),
        Yaml::Hash(h) => {
            let mut map = serde_json::Map::new();
            for (k, v) in h {
                let key = match k {
                    Yaml::String(s) => s.clone(),
                    _ => format!("{:?}", k),
                };
                map.insert(key, yaml_to_json_value(v));
            }
            Value::Object(map)
        }
        _ => Value::Null,
    }
}

pub fn yaml_to_csv(yaml_str: &str) -> Result<String, String> {
    yaml_to_json(yaml_str).and_then(|json_str| json_converter::json_to_csv(&json_str))
}

pub fn yaml_to_xml(yaml_str: &str) -> Result<String, String> {
    yaml_to_json(yaml_str).and_then(|json_str| json_converter::json_to_xml(&json_str))
}

pub fn pretty_yaml(yaml_str: &str) -> Result<String, String> {
    yaml_to_json(yaml_str).and_then(|json_str| json_converter::json_to_yaml(&json_str))
}

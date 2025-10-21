use std::collections::BTreeSet;

use csv::WriterBuilder;
use serde::Serialize;
use serde_json::Value;
use yaml_rust2::{YamlEmitter, YamlLoader};
use quick_xml::Writer;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use std::io::Cursor;

pub fn pretty_json(input: &str) -> Result<String, String> {
    let v: Value =
        serde_json::from_str(input).map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;

    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    v.serialize(&mut ser)
        .map_err(|e| format!("Erro ao serializar JSON: {}", e))?;
    String::from_utf8(buf).map_err(|e| format!("Erro ao converter JSON para UTF-8: {}", e))
}

pub fn json_to_csv(json_str: &str) -> Result<String, String> {
    let v: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;

    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    match v {
        Value::Array(arr) => {
            let mut headers = BTreeSet::new();
            for item in &arr {
                if let Value::Object(map) = item {
                    for k in map.keys() {
                        headers.insert(k.clone());
                    }
                }
            }

            let headers_vec: Vec<String> = headers.iter().cloned().collect();
            wtr.write_record(&headers_vec)
                .map_err(|e| format!("Erro ao escrever cabeçalho CSV: {}", e))?;

            for item in arr {
                if let Value::Object(map) = item {
                    let row: Vec<String> = headers_vec
                        .iter()
                        .map(|h| map.get(h).map(|val| value_to_cell(val)).unwrap_or_default())
                        .collect();
                    wtr.write_record(&row)
                        .map_err(|e| format!("Erro ao escrever linha CSV: {}", e))?;
                } else {
                    wtr.write_record(&[item.to_string()])
                        .map_err(|e| format!("Erro ao escrever linha CSV: {}", e))?;
                }
            }
        }
        Value::Object(map) => {
            let headers_vec: Vec<String> = map.keys().cloned().collect();
            wtr.write_record(&headers_vec)
                .map_err(|e| format!("Erro ao escrever cabeçalho CSV: {}", e))?;
            let row: Vec<String> = headers_vec
                .iter()
                .map(|h| map.get(h).map(|val| value_to_cell(val)).unwrap_or_default())
                .collect();
            wtr.write_record(&row)
                .map_err(|e| format!("Erro ao escrever linha CSV: {}", e))?;
        }
        _ => {
            wtr.write_record(&["value"])
                .map_err(|e| format!("Erro ao escrever cabeçalho CSV: {}", e))?;
            wtr.write_record(&[v.to_string()])
                .map_err(|e| format!("Erro ao escrever linha CSV: {}", e))?;
        }
    }

    wtr.flush()
        .map_err(|e| format!("Erro ao finalizar CSV: {}", e))?;
    let data = wtr
        .into_inner()
        .map_err(|e| format!("Erro ao extrair CSV: {}", e))?;
    let csv_string =
        String::from_utf8(data).map_err(|e| format!("Erro ao converter CSV para UTF-8: {}", e))?;
    Ok(csv_string)
}

fn value_to_cell(v: &Value) -> String {
    match v {
        Value::Null => "".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => serde_json::to_string(v).unwrap_or_default(),
    }
}

pub fn json_to_yaml(json_str: &str) -> Result<String, String> {
    // Parse JSON para Value
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;
    // Converte Value para string YAML
    let yaml_str = serde_json::to_string(&value)
        .map_err(|e| format!("Erro ao converter JSON para string: {}", e))?;
    let docs = YamlLoader::load_from_str(&yaml_str)
        .map_err(|e| format!("Erro ao carregar YAML: {}", e))?;
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    for doc in docs {
        emitter
            .dump(&doc)
            .map_err(|e| format!("Erro ao emitir YAML: {}", e))?;
    }
    Ok(out_str)
}

pub fn json_to_xml(json_str: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    write_value_as_xml(&mut writer, "root", &value)?;
    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| format!("Erro ao converter XML para UTF-8: {}", e))
}

fn write_value_as_xml<W: std::io::Write>(writer: &mut Writer<W>, tag: &str, value: &Value) -> Result<(), String> {
    let elem = BytesStart::new(tag);
    writer.write_event(Event::Start(elem)).map_err(|e| e.to_string())?;
    match value {
        Value::Null => {},
        Value::Bool(b) => {
            writer.write_event(Event::Text(BytesText::new(&b.to_string()))).map_err(|e| e.to_string())?;
        },
        Value::Number(n) => {
            writer.write_event(Event::Text(BytesText::new(&n.to_string()))).map_err(|e| e.to_string())?;
        },
        Value::String(s) => {
            writer.write_event(Event::Text(BytesText::new(s))).map_err(|e| e.to_string())?;
        },
        Value::Array(arr) => {
            for v in arr {
                write_value_as_xml(writer, "item", v)?;
            }
        },
        Value::Object(map) => {
            for (k, v) in map {
                write_value_as_xml(writer, k, v)?;
            }
        }
    }
    writer.write_event(Event::End(BytesEnd::new(tag))).map_err(|e| e.to_string())?;
    Ok(())
}

use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};

pub fn xml_to_json(xml_str: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml_str);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut stack: Vec<(String, Value)> = Vec::new();
    let mut current_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                stack.push((tag, Value::Object(Map::new())));
            }
            Ok(Event::Text(e)) => {
                let cow = reader
                    .decoder()
                    .decode(e.as_ref())
                    .map_err(|er| format!("Erro ao decodificar texto: {}", er))?;
                current_text = cow.into_owned();
            }
            Ok(Event::End(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let (elem_tag, mut elem_val) = stack.pop().unwrap_or_else(|| (tag.clone(), Value::Object(Map::new())));
                if !current_text.is_empty() {
                    if let Value::Object(ref mut map) = elem_val {
                        map.insert("_text".to_string(), Value::String(current_text.clone()));
                    }
                    current_text.clear();
                }
                // Se o elemento contém apenas texto, colapsa para string pura
                if let Value::Object(ref mut map) = elem_val {
                    if map.len() == 1 {
                        if let Some(Value::String(s)) = map.remove("_text") {
                            elem_val = Value::String(s);
                        }
                    }
                }
                if let Some((_, Value::Object(parent))) = stack.last_mut() {
                    if let Some(existing) = parent.get_mut(&elem_tag) {
                        match existing {
                            Value::Array(arr) => arr.push(elem_val),
                            v => *v = Value::Array(vec![v.clone(), elem_val]),
                        }
                    } else {
                        parent.insert(elem_tag, elem_val);
                    }
                } else {
                    // root
                    stack.push((elem_tag, elem_val));
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erro ao ler XML: {}", e)),
            _ => (),
        }
        buf.clear();
    }
    if let Some((_, root)) = stack.pop() {
        serde_json::to_string_pretty(&root).map_err(|e| format!("Erro ao serializar JSON: {}", e))
    } else {
        Err("XML vazio ou inválido".to_string())
    }
}
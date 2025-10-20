use csv::ReaderBuilder;
use serde::Serialize;
use serde_json::{Map, Value};

pub fn csv_to_json(csv_str: &str) -> Result<String, String> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_reader(csv_str.as_bytes());

    let headers = rdr
        .headers()
        .map_err(|e| format!("Erro ao ler cabeçalhos do CSV: {}", e))?
        .clone();

    let mut out = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| format!("Erro ao ler linha do CSV: {}", e))?;
        let mut map = Map::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header.to_string(), Value::String(field.to_string()));
        }
        out.push(Value::Object(map));
    }

    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    out.serialize(&mut ser)
        .map_err(|e| format!("Erro ao serializar JSON: {}", e))?;
    String::from_utf8(buf).map_err(|e| format!("Erro ao converter JSON para UTF-8: {}", e))
}

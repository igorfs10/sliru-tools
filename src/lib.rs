use std::collections::BTreeSet;

use csv::{ReaderBuilder, WriterBuilder};
use serde::Serialize;
use serde_json::{Map, Value};

slint::include_modules!();

// fn abrir_nova_janela() {
//     let nova_janela = AppWindow::new().unwrap();
//     // nova_janela.run().unwrap();
//     nova_janela.show().unwrap();
// }

#[cfg(not(target_arch = "wasm32"))]
pub fn start_desktop() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_format_converter_inverter({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let current_input_format = ui.get_formatConverterInputFormat();
            let current_output_format = ui.get_formatConverterOutputFormat();
            ui.set_formatConverterInputFormat(current_output_format);
            ui.set_formatConverterOutputFormat(current_input_format);
        }
    });

    ui.on_format_converter_execute({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let input_format = ui.get_formatConverterInputFormat();
            let output_format = ui.get_formatConverterOutputFormat();
            let input_text = ui.get_formatConverterInputText();

            match (input_format, output_format) {
                (0, 0) => {
                    // Pretty JSON
                    match pretty_json(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (0, 1) => {
                    // JSON para CSV
                    match json_to_csv(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (1, 0) => {
                    // CSV para JSON
                    match csv_to_json(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                _ => {
                    ui.set_formatConverterOutputText("Formato de entrada/saída inválido".into());
                }
            }
        }
    });

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}

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
    // Tenta fazer parse do JSON
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

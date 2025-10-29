use crate::enums::format_converter::FormatConverter;
use crate::services::csv_converter::*;
use crate::services::json_converter::*;
use crate::services::xml_converter::*;
use crate::services::yaml_converter::*;
use md5::{Digest, Md5};
use rfd::AsyncFileDialog;
use sha1::Sha1;
use sha2::Sha256;

pub mod enums;
pub mod services;

slint::include_modules!();

// fn abrir_nova_janela() {
//     let nova_janela = AppWindow::new().unwrap();
//     // nova_janela.run().unwrap();
//     nova_janela.show().unwrap();
// }

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
#[cfg(target_arch = "wasm32")]
pub fn start_wasm() -> Result<(), slint::PlatformError> {
    start()
}

pub fn start() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_open_file_verify({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let _ = slint::spawn_local(async move {
                if let Some(handle) = AsyncFileDialog::new()
                    .add_filter("*", &["*"])
                    .pick_file()
                    .await
                {
                    let data = handle.read().await;

                    let hash = match ui.get_fileVerifyOutputFormat() {
                        0 => {
                            let mut hasher = Md5::new();
                            hasher.update(data);
                            hasher.finalize().to_vec()
                        }
                        1 => {
                            let mut sha1_hasher = Sha1::new();
                            sha1_hasher.update(data);
                            sha1_hasher.finalize().to_vec()
                        }
                        _ => {
                            let mut sha256_hasher = Sha256::new();
                            sha256_hasher.update(data);
                            sha256_hasher.finalize().to_vec()
                        }
                    };
                    let hex: String = hash
                        .iter()
                        .map(|b| format!("{:02x}", b).to_string())
                        .collect::<Vec<String>>()
                        .join("");
                    ui.set_fileVerifyOutputText(hex.into());
                }
            });
        }
    });

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
            let input_format = FormatConverter::from(ui.get_formatConverterInputFormat());
            let output_format = FormatConverter::from(ui.get_formatConverterOutputFormat());
            let input_text = ui.get_formatConverterInputText();

            match (input_format, output_format) {
                (FormatConverter::Json, FormatConverter::Json) => match pretty_json(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Json, FormatConverter::Csv) => match json_to_csv(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Json, FormatConverter::Yaml) => match json_to_yaml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Json, FormatConverter::Xml) => match json_to_xml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Csv, FormatConverter::Json) => match csv_to_json(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Csv, FormatConverter::Csv) => match pretty_csv(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Csv, FormatConverter::Yaml) => match csv_to_yaml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Csv, FormatConverter::Xml) => match csv_to_xml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Yaml, FormatConverter::Json) => match yaml_to_json(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Yaml, FormatConverter::Csv) => match yaml_to_csv(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Yaml, FormatConverter::Yaml) => match pretty_yaml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Yaml, FormatConverter::Xml) => match yaml_to_xml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Xml, FormatConverter::Json) => match xml_to_json(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Xml, FormatConverter::Csv) => match xml_to_csv(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Xml, FormatConverter::Yaml) => match xml_to_yaml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
                (FormatConverter::Xml, FormatConverter::Xml) => match pretty_xml(&input_text) {
                    Ok(v) => ui.set_formatConverterOutputText(v.into()),
                    Err(e) => ui.set_formatConverterOutputText(e.into()),
                },
            }
        }
    });

    ui.run()?;

    Ok(())
}

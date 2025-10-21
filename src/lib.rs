use crate::enums::format_converter::FormatConverter;
use crate::services::csv_converter::*;
use crate::services::json_converter::*;
use crate::services::yaml_converter::*;

pub mod enums;
pub mod services;

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
            let input_format = FormatConverter::from(ui.get_formatConverterInputFormat());
            let output_format = FormatConverter::from(ui.get_formatConverterOutputFormat());
            let input_text = ui.get_formatConverterInputText();

            match (input_format, output_format) {
                (FormatConverter::Json, FormatConverter::Json) => {
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
                (FormatConverter::Json, FormatConverter::Csv) => {
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
                (FormatConverter::Json, FormatConverter::Yaml) => {
                    // JSON para YAML
                    match json_to_yaml(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (FormatConverter::Csv, FormatConverter::Json) => {
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
                (FormatConverter::Csv, FormatConverter::Csv) => {
                    // pretty CSV
                    match pretty_csv(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (FormatConverter::Csv, FormatConverter::Yaml) => {
                    // CSV para YAML
                    match csv_to_yaml(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (FormatConverter::Yaml, FormatConverter::Json) => {
                    // YAML para JSON
                    match yaml_to_json(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (FormatConverter::Yaml, FormatConverter::Csv) => {
                    // YAML para CSV
                    match yaml_to_csv(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
                (FormatConverter::Yaml, FormatConverter::Yaml) => {
                    // pretty YAML
                    match pretty_yaml(&input_text) {
                        Ok(v) => {
                            ui.set_formatConverterOutputText(v.into());
                        }
                        Err(e) => {
                            ui.set_formatConverterOutputText(e.into());
                        }
                    }
                }
            }
        }
    });

    ui.run()?;

    Ok(())
}

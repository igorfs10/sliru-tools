// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    sliru_tools_lib::start()?;
    Ok(())
}

// Para wasm32, `start_wasm` está em lib.rs com #[wasm_bindgen(start)]
#[cfg(target_arch = "wasm32")]
fn main() {}

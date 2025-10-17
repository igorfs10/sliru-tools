slint::include_modules!();


// fn abrir_nova_janela() {
//     let nova_janela = AppWindow::new().unwrap();
//     // nova_janela.run().unwrap();
//     nova_janela.show().unwrap();
// }

#[cfg(not(target_arch = "wasm32"))]
pub fn start_desktop() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

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
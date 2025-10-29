fn main() {
    let cfg = slint_build::CompilerConfiguration::new()
        .with_style("cosmic-dark".into())
        .with_bundled_translations("translations");
    slint_build::compile_with_config("ui/app-window.slint", cfg)
        .expect("Failed to compile Slint UI");
}

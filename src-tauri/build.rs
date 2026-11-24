fn main() {
    let mut attributes = tauri_build::Attributes::new();
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        // Native dialogs import TaskDialogIndirect. Link the same Common Controls v6
        // manifest into the application and Cargo's separate Windows test executable.
        attributes =
            attributes.windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("windows.manifest");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
    }
    tauri_build::try_build(attributes).expect("failed to configure Tauri");
}

#[cfg(target_os = "windows")]
fn main() {
    // Remove UAC requirement for Windows
    winres::WindowsResource::new()
        .set_manifest_file("./core/manifest.xml")
        .compile()
        .unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}

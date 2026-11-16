//! Build script: embeds the application icon into the Windows executable.
//!
//! The window icon set at runtime (`src/viz/icon.rs`) covers the taskbar and
//! alt-tab. The file manager and Explorer read the icon from the executable
//! resources instead, so the same artwork is also compiled into the .exe.
//!
//! Other platforms need no build-time work.

fn main() {
    println!("cargo:rerun-if-changed=assets/icons/icon.ico");

    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/icons/icon.ico");
        resource.set("ProductName", "Entropy Forge");
        resource.set("FileDescription", "Entropy Forge");
        resource
            .compile()
            .expect("failed to embed the Windows executable icon");
    }
}

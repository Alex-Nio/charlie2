use tauri_build::{try_build, Attributes, WindowsAttributes};

fn main() {
    // link to Vosk lib
    println!("cargo:rustc-link-search=native=libs");
    // println!("cargo:rustc-link-lib=dylib=vosk");

    if let Err(error) = try_build(
        Attributes::new()
            .windows_attributes(WindowsAttributes::new().window_icon_path("icons/icon.ico")),
    ) {
        println!("Error during try_build: {}", error);
        panic!("Error: {}", error);
    }

    // Tauri build
    tauri_build::build();
}

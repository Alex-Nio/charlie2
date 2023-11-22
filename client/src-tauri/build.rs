fn main() {
    // link to Vosk lib
    println!("cargo:rustc-link-search=native=libs");
    println!("cargo:rustc-link-lib=dylib=vosk");

    // Tauri build
    tauri_build::build()
}

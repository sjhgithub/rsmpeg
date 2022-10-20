#[cfg(target_os = "windows")]
fn main() {
    let extra_libs = [
        "dxva2",
        "evr",
        "mf",
        "mfplat",
        "mfplay",
        "mfreadwrite",
        "mfuuid",
        "bcrypt",
        "ws2_32",
        "Secur32",
        "Crypt32",
        "Strmiids",
        "ole32",
        "user32",
    ];

    for lib in extra_libs.iter() {
        println!("cargo:rustc-flags=-l {}", &lib);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}

use std::{path::PathBuf, env};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search={}/lib/linux/x64", dir);

    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    println!("cargo:rustc-link-search={}/lib/linux/x86", dir);

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    println!("cargo:rustc-link-search={}/lib/linux/armv8", dir);

    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    println!("cargo:rustc-link-search={}/lib/linux/armv6", dir);

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search={}/lib/macos/mac", dir);

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search={}/lib/windows/x64", dir);

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    println!("cargo:rustc-link-search={}/lib/windows/x86", dir);

    #[cfg(all(target_os = "android", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search={}/lib/android/x86_64", dir);

    #[cfg(all(target_os = "android", target_arch = "x86"))]
    println!("cargo:rustc-link-search={}/lib/android/x86", dir);

    #[cfg(all(target_os = "android", target_arch = "aarch64"))]
    println!("cargo:rustc-link-search={}/lib/android/arm64-v8a", dir);

    #[cfg(all(target_os = "android", target_arch = "arm"))]
    println!("cargo:rustc-link-search={}/lib/android/armeabi", dir);

    println!("cargo:rustc-link-lib=ASICamera2");
    println!("cargo:rerun-if-changed=include/ASICamera2.h");
    let bindings = bindgen::Builder::default()
        .header("include/ASICamera2.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

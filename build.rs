fn main() {
    // Look for the library in both the package root and the build directory
    let paths = vec![
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/build",
    ];

    for path in paths {
        println!("cargo:rustc-link-search=native={}", path);
    }

    println!("cargo:rustc-link-lib=static=dbmaters");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    // Watch for changes in the library files
    println!("cargo:rerun-if-changed=build/libdbmaters.a");
    println!("cargo:rerun-if-changed=build/libdbmaters.h");
}

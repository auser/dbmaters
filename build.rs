fn main() {
    let path = "./build";
    let lib = "dbmaters";

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static={}", lib);
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    println!("cargo:rerun-if-changed=build/libdbmaters.a");
    println!("cargo:rerun-if-changed=build/libdbmaters.h");
}

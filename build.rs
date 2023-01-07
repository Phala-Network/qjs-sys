fn main() {
    let rootdir = std::env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR");
    let target = std::env::var("TARGET").unwrap();
    let is_wasm32 = target.starts_with("wasm32"); // == "wasm32-unknown-unknown";
    if is_wasm32 {
        let status = std::process::Command::new("make")
            .arg("libc")
            .spawn()
            .expect("Failed to run make libc")
            .wait()
            .expect("Failed to wait for make libc");
        if !status.success() {
            panic!("Failed to run make libc");
        }
        println!(
            "cargo:rustc-link-search={}/pink-libc/sysroot/lib/wasm32-pink",
            rootdir
        );
        println!("cargo:rustc-link-lib=c");
    }

    let cfiles = [
        "csrc/cutils.c",
        "csrc/libregexp.c",
        "csrc/libunicode.c",
        "csrc/quickjs.c",
        "csrc/qjs-pink.c",
        "csrc/libbf.c",
    ];
    let mut cc = cc::Build::new();
    for file in cfiles.iter() {
        println!("cargo:rerun-if-changed={}", file);
        cc.file(file);
    }
    cc.flag("-funsigned-char")
        .define("CONFIG_BIGNUM", None)
        .define("_GNU_SOURCE", None)
        .define("__pink__", "1")
        .flag("-w");

    if is_wasm32 {
        cc.include("pink-libc/sysroot/include");
        cc.archiver("llvm-ar");
        cc.warnings(false);
    }
    cc.compile("qjs");

    println!("cargo:rerun-if-changed=csrc/qjs-pink.h");
    let mut builder = bindgen::Builder::default()
        .header("csrc/qjs-pink.h")
        .clang_arg("-funsigned-char")
        .clang_arg("-DCONFIG_BIGNUM")
        .clang_arg("-D_GNU_SOURCE")
        .clang_arg("-D__pink__=1")
        .clang_arg("-w")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    if is_wasm32 {
        builder = builder
            .clang_arg("-fvisibility=default")
            .clang_arg(format!("-I{}/pink-libc/sysroot/include", rootdir));
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

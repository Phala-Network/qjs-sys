fn main() {
    let rootdir = std::env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR");
    let target = std::env::var("TARGET").unwrap();
    let is_pink = target.starts_with("wasm32"); // == "wasm32-unknown-unknown";
    if is_pink {
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
        .flag("-Wno-unknown-attributes")
        .define("CONFIG_BIGNUM", "");

    if is_pink {
        cc.define("__pink__", "1");
        cc.define("_GNU_SOURCE", "");
        cc.include("pink-libc/sysroot/include");
        cc.archiver("llvm-ar");
        cc.warnings(false);
    }
    cc.compile("qjs");

    println!("cargo:rerun-if-changed=wrapper.h");
    let mut builder = bindgen::Builder::default()
        .header("csrc/qjs-pink.h")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    if target.starts_with("wasm32") {
        builder = builder.clang_arg("-fvisibility=default")
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

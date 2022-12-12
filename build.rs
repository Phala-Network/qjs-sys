fn main() {
    let target = std::env::var("TARGET").unwrap();

    let mut cc = cc::Build::new();
    cc.flag_if_supported("-funsigned-char")
        .flag_if_supported("-w")
        .file("csrc/cutils.c")
        .file("csrc/libregexp.c")
        .file("csrc/libunicode.c")
        .file("csrc/quickjs.c")
        .file("csrc/qjs-pink.c");

    if target.starts_with("wasm32") {
        cc.define("__WASM32__", "1");
        cc.define("__PINK__", "1");
        cc.define("_GNU_SOURCE", "");
        cc.warnings(false);
    }
    cc.compile("qjs");

    println!("cargo:rerun-if-changed=wrapper.h");
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    if target.starts_with("wasm32") {
        builder = builder
            .clang_arg("-D__WASM32__=1")
            .clang_arg("-D__PINK__=1")
            .clang_arg("-D_GNU_SOURCE")
            .clang_arg("-fvisibility=default")
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let rootdir = std::env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR");
    let target = std::env::var("TARGET").unwrap();
    let is_wasm32 = target.starts_with("wasm32");
    let use_pink_libc = target.starts_with("wasm32-unknown-unknown");
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
    }
    if use_pink_libc {
        println!(
            "cargo:rustc-link-search={}/pink-libc/sysroot/lib/wasm32-pink",
            rootdir
        );
        println!("cargo:rustc-link-lib=c");
    }

    let cfiles = [
        "csrc/quickjs/cutils.c",
        "csrc/quickjs/libregexp.c",
        "csrc/quickjs/libunicode.c",
        "csrc/quickjs/quickjs.c",
        "csrc/quickjs/libbf.c",
        "csrc/qjs-pink.c",
        "csrc/quickjs-opaque.c",
    ];
    let c_flags = [
        "-funsigned-char",
        "-D_GNU_SOURCE",
        "-D__pink__=1",
        "-Icsrc/quickjs",
        "-Werror",
        "-Wno-unknown-warning-option",
        "-Wno-attributes",
        "-Wno-unused-parameter",
        "-Wno-sign-compare",
        "-Wno-unused-function",
        "-Wno-shift-op-parentheses",
    ];
    let mut cc = cc::Build::new();
    for file in cfiles.iter() {
        println!("cargo:rerun-if-changed={}", file);
        cc.file(file);
    }
    for flag in c_flags {
        cc.flag(flag);
    }
    #[cfg(feature = "sanitize-address")]
    {
        cc.flag("-fsanitize=address");
        println!("cargo:rustc-link-lib=asan");
    }
    #[cfg(feature = "classic-host-call")]
    cc.define("WITH_CLASSIC_HOST_CALL", None);
    #[cfg(feature = "pink-allocator")]
    cc.define("CONFIG_PINK_ALLOCATOR", None);

    if is_wasm32 {
        cc.include("pink-libc/sysroot/include");
        cc.archiver("llvm-ar");
        cc.warnings(false);
    }
    cc.compile("qjs");

    println!("cargo:rerun-if-changed=csrc/qjs-pink.h");
    println!("cargo:rerun-if-changed=csrc/quickjs/quickjs.h");
    let mut builder = bindgen::Builder::default()
        .header("csrc/qjs-pink.h")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for flag in c_flags {
        builder = builder.clang_arg(flag);
    }
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

fn main() {
    cc::Build::new()
        .flag_if_supported("-funsigned-char")
        .flag_if_supported("-w")
        .file("csrc/cutils.c")
        .file("csrc/libregexp.c")
        .file("csrc/libunicode.c")
        .file("csrc/quickjs.c")
        .file("csrc/qjs-env.c")
        .compile("qjs");
}

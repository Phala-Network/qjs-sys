fn main() {
    let js = r#"
    (function(){
        console.log("Hello, World!");
        return "Powered by QuickJS in ink!";
    })()
    "#;
    dbg!(qjs_sys::eval(js));
}

mod polyfill {
    use core::ffi::{c_int, c_uchar};

    #[no_mangle]
    extern "C" fn __pink_fd_write(fd: c_int, buf: *const c_uchar, len: usize) -> usize {
        let bin = unsafe { core::slice::from_raw_parts(buf, len) };
        let message = core::str::from_utf8(bin).unwrap_or("<Invalid UTF-8 string>");
        match fd {
            1 => print!("{}", message),
            2 => print!("{}", message),
            _ => {}
        }
        len
    }
}


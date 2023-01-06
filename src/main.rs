fn main() {
    let js = qjs_sys::JsCode::Source(
        r#"
            (function(){
                console.log("Hello, World!");
                __hostCall(123, 100);
                __hostCall(123, 101n);
                // __hostCall(123, 10000000000000000000000000000000000000000001n);
                // __hostCall(123, new Uint8Array(100));
                // __hostCall(123, new Uint8Array([1, 2, 3]));
                // __hostCall(123, new Uint8Array(new Uint8Array([1, 2, 3])));
                return __hostCall(123, 456);
            })()
        "#,
    );

    _ = dbg!(qjs_sys::eval(js, &[]));
}

mod polyfill {
    use core::ffi::{c_int, c_uchar};
    use qjs_sys::c;

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

    #[no_mangle]
    extern "C" fn __pink_clock_time_get(_id: u32, _precision: u64, retptr0: *mut u64) -> u16 {
        unsafe {
            *retptr0 = 0;
        }
        0
    }

    #[no_mangle]
    fn __pink_host_call(
        id: u32,
        ctx: *mut c::JSContext,
        this: c::JSValueConst,
        args: &[c::JSValueConst],
    ) -> c::JSValue {
        use qjs_sys::convert;
        println!("__pink_host_call({id}, argc={})", args.len());

        println!("js_val_into_bytes = {:?})", convert::js_val_into_bytes(ctx, args[0]));
        println!("js_val_into_u128 = {:?})", convert::js_val_into_u128(ctx, args[0]));
        println!("js_val_into_string = {:?})", convert::js_val_into_string(ctx, args[0]));

        c::JS_NULL
    }
}

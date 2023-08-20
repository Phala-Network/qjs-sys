use std::ffi::CString;

use qjs_sys::JsCode;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let script_file = &args[1];
    let script = std::fs::read(script_file).unwrap();
    let c_script = CString::new(script).unwrap();
    let args: Vec<_> = args[2..].into_iter().cloned().collect();
    qjs_sys::eval(&[JsCode::Source(&c_script)], &args).unwrap();
}

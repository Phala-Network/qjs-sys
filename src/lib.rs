mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[no_mangle]
pub fn eval(script: &str) -> bool {
    let bytes = script.as_bytes();
    let rv = unsafe { sys::js_eval_oneshot(&bytes[0] as *const u8 as _, bytes.len() as _) };
    rv == 0
}

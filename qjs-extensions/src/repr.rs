use alloc::string::{String, ToString};

pub fn setup(ns: &js::Value) -> js::Result<()> {
    ns.define_property_fn("repr", host_repr)?;
    Ok(())
}

#[derive(js::FromJsValue, Default, Debug)]
#[qjsbind(rename_all = "camelCase")]
pub struct ReprConfig {
    depth: Option<u8>,
    indent: Option<js::JsString>,
}

impl ReprConfig {
    fn indent_or(&self, default: &'static str) -> &str {
        match &self.indent {
            Some(indent) => indent.as_str(),
            None => default,
        }
    }
}

#[js::host_call]
fn host_repr(obj: js::Value, config: Option<ReprConfig>) -> String {
    let config = config.unwrap_or_default();
    let mut buf = String::new();
    let depth = config.depth.unwrap_or(5);
    js::recursive_to_string(&obj, depth, true, &mut buf, config.indent_or(""), 0);
    buf
}

pub fn print(args: &[js::Value], config: &ReprConfig) -> String {
    let mut buf = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i != 0 {
            buf.push(' ');
        }
        let depth = config.depth.unwrap_or(0);
        if depth == 0 {
            buf.push_str(&arg.to_string());
        } else {
            js::recursive_to_string(arg, depth, false, &mut buf, config.indent_or(""), 0);
        }
    }
    buf
}

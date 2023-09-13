use alloc::string::{String, ToString};

pub fn setup(ns: &js::Value) -> js::Result<()> {
    ns.define_property_fn("repr", host_repr)?;
    Ok(())
}

#[derive(js::FromJsValue, Default)]
#[qjsbind(rename_all = "camelCase")]
pub struct ReprConfig {
    depth: Option<u8>,
    indent: Option<js::JsString>,
}

impl ReprConfig {
    fn depth(&self) -> u8 {
        self.depth.unwrap_or(5)
    }
    fn indent(&self) -> &str {
        match &self.indent {
            Some(indent) => indent.as_str(),
            None => "  ",
        }
    }
}

#[js::host_call]
fn host_repr(obj: js::Value, config: Option<ReprConfig>) -> String {
    repr(&obj, &config.unwrap_or_default(), true)
}

fn repr(obj: &js::Value, config: &ReprConfig, escape: bool) -> String {
    let mut buf = String::new();
    js::recursive_to_string(obj, config.depth(), escape, &mut buf, config.indent(), 0);
    buf
}

pub fn print(args: &[js::Value], config: &ReprConfig) -> String {
    let mut buf = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i != 0 {
            buf.push(' ');
        }
        if config.depth() == 0 {
            buf.push_str(&arg.to_string());
        } else {
            js::recursive_to_string(arg, config.depth(), false, &mut buf, config.indent(), 0);
        }
    }
    buf
}

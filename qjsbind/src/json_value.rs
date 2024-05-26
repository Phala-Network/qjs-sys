use crate::{traits::ToJsValue, Context, Error, Result, Value};
use serde_json::Value as JsonValue;

impl ToJsValue for JsonValue {
    fn to_js_value(&self, ctx: &Context) -> Result<Value> {
        match self {
            JsonValue::Null => Ok(Value::null()),
            JsonValue::Bool(v) => v.to_js_value(ctx),
            JsonValue::Number(n) => n.to_js_value(ctx),
            JsonValue::String(s) => s.to_js_value(ctx),
            JsonValue::Array(arr) => arr.to_js_value(ctx),
            JsonValue::Object(obj) => {
                let js_object = Value::new_object(ctx);
                for (key, value) in obj.iter() {
                    js_object.set_property(key, &value.to_js_value(ctx)?)?;
                }
                Ok(js_object)
            }
        }
    }
}

impl ToJsValue for serde_json::Number {
    fn to_js_value(&self, ctx: &Context) -> Result<Value> {
        if let Some(v) = self.as_u64() {
            v.to_js_value(ctx)
        } else if let Some(v) = self.as_i64() {
            v.to_js_value(ctx)
        } else if let Some(v) = self.as_f64() {
            v.to_js_value(ctx)
        } else {
            Err(Error::Static("can not convert json number to js value"))
        }
    }
}

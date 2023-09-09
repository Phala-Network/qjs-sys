use alloc::{format, string::String, sync::Arc, vec::Vec};
use core::ptr::NonNull;
use parity_scale_codec::{Compact, Decode, Encode, Output};

use qjs::{self as js, c, AsBytes, BytesOrHex, FromJsValue, ToJsValue};

use self::parser::{EnumType, PrimitiveType, ScaleType};

mod parser;

pub fn setup(obj: &js::Value) -> js::Result<()> {
    obj.define_property_fn("scaleParseTypes", parse_types)?;
    obj.define_property_fn("scaleEncode", encode)?;
    obj.define_property_fn("scaleEncodeAll", encode_all)?;
    obj.define_property_fn("scaleDecode", decode)?;
    Ok(())
}

impl EnumType {
    fn get_variant_by_name(&self, name: &str) -> js::Result<(&str, Option<usize>, usize)> {
        for (ind, (variant_name, tid, scale_ind)) in self.variants.iter().enumerate() {
            if variant_name == name {
                return Ok((variant_name.as_str(), tid.clone(), scale_ind.unwrap_or(ind)));
            }
        }
        Err(js::Error::Custom(format!("Unknown variant {}", name)))
    }

    fn get_variant_by_index(&self, tag: usize) -> js::Result<(&str, Option<usize>)> {
        match self.variants.get(tag) {
            Some((name, tid, ind)) => match ind {
                Some(ind) => {
                    if tag == *ind {
                        return Ok((name.as_str(), tid.clone()));
                    }
                }
                None => return Ok((name.as_str(), tid.clone())),
            },
            None => (),
        };
        // fallback to linear search for custom index
        for (name, tid, ind) in self.variants.iter() {
            if let Some(ind) = ind {
                if tag == *ind {
                    return Ok((name.as_str(), tid.clone()));
                }
            }
        }
        Err(js::Error::Custom(format!("Unknown variant {}", tag)))
    }
}

#[derive(Debug, Clone)]
struct TypeRegistry {
    types: Arc<Vec<parser::ScaleType>>,
}

impl TypeRegistry {
    fn get_type(&self, id: usize) -> js::Result<&ScaleType> {
        self.types
            .get(id)
            .ok_or(js::Error::Custom(format!("Unknown type id {id}")))
    }
}

impl js::FromJsValue for TypeRegistry {
    fn from_js_value(value: js::Value) -> js::Result<Self> {
        let me = value
            .opaque_object_data::<Self>()
            .ok_or(js::Error::Expect("TypeRegistry"))?;
        Ok(me.clone())
    }
}

impl js::ToJsValue for TypeRegistry {
    fn to_js_value(&self, ctx: NonNull<c::JSContext>) -> js::Result<js::Value> {
        Ok(js::Value::new_opaque_object(ctx, self.clone()))
    }
}

fn to_js_error(errs: Vec<impl core::fmt::Debug>) -> js::Error {
    let mut output = String::new();
    for err in errs {
        output.push_str(&format!("{err:?}\n"));
    }
    js::Error::Custom(output)
}

#[js::host_call]
fn parse_types(typelist: js::JsString) -> js::Result<TypeRegistry> {
    use chumsky::Parser;
    let types = parser::parser()
        .parse(typelist.as_str())
        .into_result()
        .map_err(to_js_error)?;
    Ok(TypeRegistry {
        types: types.into(),
    })
}

#[js::host_call]
fn encode_all(
    value: js::Value,
    type_ids: Vec<u32>,
    type_registry: TypeRegistry,
) -> js::Result<AsBytes<Vec<u8>>> {
    let mut out = Vec::new();
    for (ind, type_id) in type_ids.iter().enumerate() {
        let sub_value = value.index(ind as _)?;
        encode_value(sub_value, *type_id as _, &type_registry, &mut out)?;
    }
    Ok(AsBytes(out))
}

#[js::host_call]
fn encode(
    value: js::Value,
    type_id: u32,
    type_registry: TypeRegistry,
) -> js::Result<AsBytes<Vec<u8>>> {
    let mut out = Vec::new();
    encode_value(value, type_id as _, &type_registry, &mut out)?;
    Ok(AsBytes(out))
}

fn u8a_or_hex<T>(
    value: &js::Value,
    f: impl FnOnce(&[u8]) -> js::Result<T>,
) -> Option<js::Result<T>> {
    if value.is_uint8_array() {
        let arr = match js::JsUint8Array::from_js_value(value.clone()) {
            Ok(arr) => arr,
            Err(err) => return Some(Err(err)),
        };
        let bytes = arr.as_bytes();
        return Some(f(bytes));
    }
    if value.is_string() {
        let bytes = match BytesOrHex::<Vec<u8>>::from_js_value(value.clone()) {
            Ok(bytes) => bytes.0,
            Err(err) => return Some(Err(err)),
        };
        return Some(f(&bytes));
    }
    None
}

fn encode_value(
    value: js::Value,
    type_id: usize,
    type_registry: &TypeRegistry,
    out: &mut impl Output,
) -> js::Result<()> {
    let t = type_registry.get_type(type_id)?;
    match t {
        ScaleType::Primitive(t) => encode_primitive(value, t, out),
        ScaleType::Compact(tid) => {
            let t = type_registry.get_type(*tid)?;
            match t {
                ScaleType::Primitive(t) => encode_compact_primitive(value, t, out),
                ScaleType::Tuple(t) if t.is_empty() => {
                    Compact(()).encode_to(out);
                    Ok(())
                }
                _ => compactable_err(),
            }
        }
        ScaleType::Seq(tid) => {
            let t = type_registry.get_type(*tid)?;
            if matches!(t, ScaleType::Primitive(PrimitiveType::U8)) {
                let result = u8a_or_hex(&value, |bytes| {
                    bytes.encode_to(out);
                    Ok(())
                });
                if let Some(result) = result {
                    return result;
                }
            }
            let length = value.get_property("length")?.decode_u32()?;
            Compact(length).encode_to(out);
            for i in 0..length {
                encode_value(value.index(i as _)?, *tid, type_registry, out)?;
            }
            Ok(())
        }
        ScaleType::Tuple(ids) => {
            for (ind, tid) in ids.iter().enumerate() {
                let sub_value = value.index(ind)?;
                encode_value(sub_value, *tid, type_registry, out)?;
            }
            Ok(())
        }
        ScaleType::Array(tid, len) => {
            let t = type_registry.get_type(*tid)?;
            if matches!(t, ScaleType::Primitive(PrimitiveType::U8)) {
                let result = u8a_or_hex(&value, |bytes| {
                    if bytes.len() != *len {
                        return Err(js::Error::Custom(format!(
                            "Expected array of length {}, got {}",
                            len,
                            bytes.len()
                        )));
                    }
                    out.write(bytes);
                    Ok(())
                });
                if let Some(result) = result {
                    return result;
                }
            }
            for ind in 0..*len {
                let sub_value = value.index(ind)?;
                encode_value(sub_value, *tid, type_registry, out)?;
            }
            Ok(())
        }
        ScaleType::Enum(def) => {
            for entry in value.entries()? {
                let (k, v) = entry?;
                let key = js::JsString::from_js_value(k)?;
                if let Some((_name, tid, ind)) = def.get_variant_by_name(key.as_str()).ok() {
                    let Ok(ind) = u8::try_from(ind) else {
                        return Err(js::Error::Custom(format!(
                            "Variant index {} is too large",
                            ind
                        )));
                    };
                    ind.encode_to(out);
                    if let Some(tid) = tid {
                        encode_value(v, tid, type_registry, out)?;
                    }
                    return Ok(());
                }
            }
            Err(js::Error::Custom(format!(
                "Enum with any variant of {}",
                def.variants
                    .iter()
                    .map(|(name, _, _)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )))
        }
        ScaleType::Struct(fields) => {
            for (name, tid) in fields.iter() {
                let sub_value = value.get_property(name)?;
                encode_value(sub_value, *tid, type_registry, out)?;
            }
            Ok(())
        }
    }
}

fn encode_primitive(value: js::Value, t: &PrimitiveType, out: &mut impl Output) -> js::Result<()> {
    match t {
        PrimitiveType::U8 => {
            value.decode_u8()?.encode_to(out);
        }
        PrimitiveType::U16 => {
            value.decode_u16()?.encode_to(out);
        }
        PrimitiveType::U32 => {
            value.decode_u32()?.encode_to(out);
        }
        PrimitiveType::U64 => {
            value.decode_u64()?.encode_to(out);
        }
        PrimitiveType::U128 => {
            value.decode_u128()?.encode_to(out);
        }
        PrimitiveType::I8 => {
            value.decode_i8()?.encode_to(out);
        }
        PrimitiveType::I16 => {
            value.decode_i16()?.encode_to(out);
        }
        PrimitiveType::I32 => {
            value.decode_i32()?.encode_to(out);
        }
        PrimitiveType::I64 => {
            value.decode_i64()?.encode_to(out);
        }
        PrimitiveType::I128 => {
            value.decode_i128()?.encode_to(out);
        }
        PrimitiveType::Bool => {
            value.decode_bool()?.encode_to(out);
        }
        PrimitiveType::Str => {
            js::JsString::from_js_value(value)?.as_str().encode_to(out);
        }
    }
    Ok(())
}

fn compactable_err<T>() -> js::Result<T> {
    Err(js::Error::Expect("A number or () for compact"))
}

fn encode_compact_primitive(
    value: js::Value,
    t: &PrimitiveType,
    out: &mut impl Output,
) -> js::Result<()> {
    match t {
        PrimitiveType::U8 => Compact(value.decode_u8()?).encode_to(out),
        PrimitiveType::U16 => Compact(value.decode_u16()?).encode_to(out),
        PrimitiveType::U32 => Compact(value.decode_u32()?).encode_to(out),
        PrimitiveType::U64 => Compact(value.decode_u64()?).encode_to(out),
        PrimitiveType::U128 => Compact(value.decode_u128()?).encode_to(out),
        _ => return compactable_err(),
    }
    Ok(())
}

#[js::host_call(with_context)]
fn decode(
    ctx: js::Context,
    _this: js::Value,
    value: js::JsUint8Array,
    type_id: u32,
    type_registry: TypeRegistry,
) -> js::Result<js::Value> {
    decode_valude(&ctx, &mut value.as_bytes(), type_id as _, &type_registry)
}

fn decode_valude(
    ctx: &js::Context,
    buf: &mut &[u8],
    type_id: usize,
    type_registry: &TypeRegistry,
) -> js::Result<js::Value> {
    let t = type_registry.get_type(type_id as _)?;
    match t {
        ScaleType::Primitive(t) => decode_primitive(ctx, buf, t),
        ScaleType::Compact(tid) => {
            let t = type_registry.get_type(*tid)?;
            match t {
                ScaleType::Primitive(t) => decode_compact_primitive(ctx, buf, t),
                ScaleType::Tuple(t) if t.is_empty() => {
                    Compact::<()>::decode(buf)
                        .map_err(|_| js::Error::Static("Unexpected end of buffer"))?;
                    Ok(ctx.new_array())
                }
                _ => compactable_err(),
            }
        }
        ScaleType::Seq(tid) => {
            let t = type_registry.get_type(*tid)?;
            if matches!(t, ScaleType::Primitive(PrimitiveType::U8)) {
                let value = Vec::<u8>::decode(buf)
                    .map_err(|_| js::Error::Static("Unexpected end of buffer"))?;
                return AsBytes(value).to_js_value(ctx.ptr());
            }
            let length = Compact::<u32>::decode(buf)
                .map_err(|_| js::Error::Static("Unexpected end of buffer"))?
                .0;
            let out = ctx.new_array();
            for _ in 0..length {
                let sub_value = decode_valude(ctx, buf, *tid, type_registry)?;
                out.array_push(&sub_value)?;
            }
            Ok(out)
        }
        ScaleType::Tuple(tids) => {
            let out = ctx.new_array();
            for tid in tids {
                let sub_value = decode_valude(ctx, buf, *tid, type_registry)?;
                out.array_push(&sub_value)?;
            }
            Ok(out)
        }
        ScaleType::Array(tid, len) => {
            let t = type_registry.get_type(*tid)?;
            if matches!(t, ScaleType::Primitive(PrimitiveType::U8)) {
                if buf.len() < *len {
                    return Err(js::Error::Static("Unexpected end of buffer"));
                }
                let value = buf[..*len].to_vec();
                *buf = &buf[*len..];
                return AsBytes(value).to_js_value(ctx.ptr());
            }
            let out = ctx.new_array();
            for _ in 0..*len {
                let sub_value = decode_valude(ctx, buf, *tid, type_registry)?;
                out.array_push(&sub_value)?;
            }
            Ok(out)
        }
        ScaleType::Enum(variants) => {
            let tag = u8::decode(buf).map_err(|_| js::Error::Static("Unexpected end of buffer"))?;
            let (variant_name, variant_type) = variants.get_variant_by_index(tag as usize)?;
            let out = ctx.new_object();
            if let Some(variant_type) = variant_type {
                let sub_value = decode_valude(ctx, buf, variant_type, type_registry)?;
                out.set_property(variant_name, &sub_value)?;
            } else {
                out.set_property(variant_name, &js::Value::Null)?;
            }
            Ok(out)
        }
        ScaleType::Struct(fields) => {
            let out = ctx.new_object();
            for (name, tid) in fields {
                let sub_value = decode_valude(ctx, buf, *tid, type_registry)?;
                out.set_property(name, &sub_value)?;
            }
            Ok(out)
        }
    }
}

fn decode_primitive(
    ctx: &js::Context,
    buf: &mut &[u8],
    t: &PrimitiveType,
) -> js::Result<js::Value> {
    macro_rules! decode_num {
        ($t: ident) => {{
            let value =
                <$t>::decode(buf).map_err(|_| js::Error::Static("Unexpected end of buffer"))?;
            value.to_js_value(ctx.ptr())
        }};
    }
    match t {
        PrimitiveType::U8 => decode_num!(u8),
        PrimitiveType::U16 => decode_num!(u16),
        PrimitiveType::U32 => decode_num!(u32),
        PrimitiveType::U64 => decode_num!(u64),
        PrimitiveType::U128 => decode_num!(u128),
        PrimitiveType::I8 => decode_num!(i8),
        PrimitiveType::I16 => decode_num!(i16),
        PrimitiveType::I32 => decode_num!(i32),
        PrimitiveType::I64 => decode_num!(i64),
        PrimitiveType::I128 => decode_num!(i128),
        PrimitiveType::Bool => decode_num!(bool),
        PrimitiveType::Str => String::decode(buf)
            .map_err(|_| js::Error::Static("Unexpected end of buffer"))?
            .to_js_value(ctx.ptr()),
    }
}

fn decode_compact_primitive(
    ctx: &js::Context,
    buf: &mut &[u8],
    t: &PrimitiveType,
) -> js::Result<js::Value> {
    macro_rules! decode_num {
        ($t: ident) => {{
            let value = Compact::<$t>::decode(buf)
                .map_err(|_| js::Error::Static("Unexpected end of buffer"))?;
            value.0.to_js_value(ctx.ptr())
        }};
    }
    match t {
        PrimitiveType::U8 => decode_num!(u8),
        PrimitiveType::U16 => decode_num!(u16),
        PrimitiveType::U32 => decode_num!(u32),
        PrimitiveType::U64 => decode_num!(u64),
        PrimitiveType::U128 => decode_num!(u128),
        _ => compactable_err(),
    }
}

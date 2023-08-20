use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use core::fmt::Display;
use core::result;
use serde::ser::{Impossible, Serialize};

use super::{c, Error, Value};

type Result<T> = result::Result<T, Error>;

struct Serializer {
    ctx: *mut c::JSContext,
}

impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::bool(self.ctx, value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> Result<Value> {
        if let Ok(value) = i32::try_from(value) {
            Ok(Value::int(self.ctx, value))
        } else {
            Ok(Value::bigint(self.ctx, value))
        }
    }

    fn serialize_i128(self, value: i128) -> Result<Value> {
        if let Ok(value) = i64::try_from(value) {
            self.serialize_i64(value)
        } else {
            Ok(Value::bigint_from_str(self.ctx, &value.to_string()))
        }
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Value> {
        if value <= i32::MAX as u64 {
            self.serialize_i32(value as i32)
        } else if value <= i64::MAX as u64 {
            Ok(Value::bigint(self.ctx, value as i64))
        } else {
            Ok(Value::bigint_from_str(self.ctx, &value.to_string()))
        }
    }

    fn serialize_u128(self, value: u128) -> Result<Value> {
        {
            if let Ok(value) = u64::try_from(value) {
                self.serialize_u64(value)
            } else {
                Ok(Value::bigint_from_str(self.ctx, &value.to_string()))
            }
        }
    }

    #[inline]
    fn serialize_f32(self, float: f32) -> Result<Value> {
        Ok(Value::float(self.ctx, float as f64))
    }

    #[inline]
    fn serialize_f64(self, float: f64) -> Result<Value> {
        Ok(Value::float(self.ctx, float))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Value> {
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::string(self.ctx, value))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        Ok(Value::bytes(self.ctx, value))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::null(self.ctx))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        let obj = Value::object(self.ctx);
        obj.set_property("tag", &Value::string(self.ctx, variant))?;
        obj.set_property("value", &to_value(self.ctx, value)?)?;
        Ok(obj)
    }

    #[inline]
    fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Value::array(self.ctx),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: String::from(variant),
            vec: Value::array(self.ctx),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap {
            map: Value::object(self.ctx),
            next_key: None,
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(SerializeMap::Number { out_value: None }),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(SerializeMap::RawValue { out_value: None }),
            _ => self.serialize_map(Some(len)),
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: String::from(variant),
            map: Value::object(self.ctx),
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Display,
    {
        Ok(Value::string(self.ctx, &value.to_string()))
    }
}

pub struct SerializeVec {
    vec: Value,
}

pub struct SerializeTupleVariant {
    name: String,
    vec: Value,
}

pub struct SerializeMap {
    map: Value,
    next_key: Option<String>,
}

pub struct SerializeStructVariant {
    name: String,
    map: Value,
}

impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.array_push(&to_value(self.vec.ctx, value)?)
    }

    fn end(self) -> Result<Value> {
        Ok(self.vec)
    }
}

impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.array_push(&to_value(self.vec.ctx, value)?)
    }

    fn end(self) -> Result<Value> {
        let obj = Value::object(self.vec.ctx);
        obj.set_property("tag", &Value::string(self.vec.ctx, &self.name))?;
        obj.set_property("value", &self.vec)?;
        Ok(obj)
    }
}

impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.next_key = Some(key.serialize(MapKeySerializer)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect("serialize_value called before serialize_key");
        self.map.set_property(&key, &to_value(self.map.ctx, value)?)
    }

    fn end(self) -> Result<Value> {
        Ok(self.map)
    }
}

struct MapKeySerializer;

fn key_must_be_a_string() -> Error {
    Error::Static("Key must be a string")
}

impl serde::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {
        Ok(variant.to_owned())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_bool(self, value: bool) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i8(self, value: i8) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, _value: f32) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_f64(self, _value: f64) -> Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<String> {
        Ok({
            let mut s = String::new();
            s.push(value);
            s
        })
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<String> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }

    fn collect_str<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Display,
    {
        Ok(value.to_string())
    }
}

impl serde::ser::SerializeStruct for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeMap::end(self)
    }
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map.set_property(key, &to_value(self.map.ctx, value)?)
    }

    fn end(self) -> Result<Value> {
        let obj = Value::object(self.map.ctx);
        obj.set_property("tag", &Value::string(self.map.ctx, &self.name))?;
        obj.set_property("value", &self.map)?;
        Ok(obj)
    }
}

pub fn to_value<T: Serialize + ?Sized>(ctx: *mut c::JSContext, value: &T) -> Result<Value> {
    value.serialize(Serializer { ctx })
}

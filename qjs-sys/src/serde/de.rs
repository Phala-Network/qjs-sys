use alloc::string::String;

use serde::de::{
    Deserialize, DeserializeSeed, EnumAccess, Expected, IntoDeserializer, MapAccess, SeqAccess,
    Unexpected, VariantAccess, Visitor,
};

use super::{Error, Iter, PairIter, Value};

macro_rules! deserialize_number {
    ($deserialize_method:ident, $visit_method:ident) => {
        fn $deserialize_method<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            if self.is_number() || self.is_big_int() {
                if let Some(value) = self.decode_number() {
                    return visitor.$visit_method(value);
                }
            }
            Err(self.invalid_type(&visitor))
        }
    };
}

fn visit_array<'de, V>(array: Value, visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = array.length().ok_or(array.invalid_type(&visitor))?;
    let mut deserializer =
        SeqDeserializer::new(array.clone()).ok_or(array.invalid_type(&visitor))?;
    let seq = visitor.visit_seq(&mut deserializer)?;
    if deserializer.decoded_items != len {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in array",
        ))
    }
}

fn visit_object<'de, V>(object: Value, visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let mut deserializer =
        MapDeserializer::new(object.clone()).ok_or(object.invalid_type(&visitor))?;
    let map = visitor.visit_map(&mut deserializer)?;
    Ok(map)
}

impl<'de> serde::Deserializer<'de> for Value {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.is_null() || self.is_undefined() {
            return visitor.visit_unit();
        }
        if let Some(value) = self.decode_bool() {
            return visitor.visit_bool(value);
        }
        if let Some(value) = self.decode_string() {
            return visitor.visit_string(value);
        }
        if let Some(value) = self.decode_u8() {
            return visitor.visit_u8(value);
        }
        if let Some(value) = self.decode_u32() {
            return visitor.visit_u32(value);
        }
        if let Some(value) = self.decode_number() {
            return visitor.visit_u64(value);
        }
        if let Some(value) = self.decode_number() {
            return visitor.visit_u128(value);
        }
        if let Some(value) = self.decode_i8() {
            return visitor.visit_i8(value);
        }
        if let Some(value) = self.decode_i32() {
            return visitor.visit_i32(value);
        }
        if let Some(value) = self.decode_i64() {
            return visitor.visit_i64(value);
        }
        if let Some(value) = self.decode_number() {
            return visitor.visit_i128(value);
        }
        if let Some(value) = self.decode_bytes() {
            return visitor.visit_byte_buf(value);
        }
        if self.is_object() {
            return visit_object(self, visitor);
        }
        Err(self.invalid_type(&visitor))
    }

    deserialize_number!(deserialize_i8, visit_i8);
    deserialize_number!(deserialize_i16, visit_i16);
    deserialize_number!(deserialize_i32, visit_i32);
    deserialize_number!(deserialize_i64, visit_i64);
    deserialize_number!(deserialize_i128, visit_i128);
    deserialize_number!(deserialize_u8, visit_u8);
    deserialize_number!(deserialize_u16, visit_u16);
    deserialize_number!(deserialize_u32, visit_u32);
    deserialize_number!(deserialize_u64, visit_u64);
    deserialize_number!(deserialize_u128, visit_u128);
    deserialize_number!(deserialize_f32, visit_f32);
    deserialize_number!(deserialize_f64, visit_f64);

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.is_null() || self.is_undefined() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = if self.is_string() {
            (
                self.decode_string().ok_or(self.invalid_type(&visitor))?,
                None,
            )
        } else if self.is_object() {
            let variant = self
                .get_property("tag")
                .decode_string()
                .ok_or(self.invalid_type(&visitor))?;
            let value = self.get_property("value");
            if value.is_null() || value.is_undefined() {
                (variant, None)
            } else {
                (variant, Some(value))
            }
        } else {
            return Err(self.invalid_type(&visitor));
        };
        visitor.visit_enum(EnumDeserializer { variant, value })
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let _ = name;
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.decode_bool() {
            Some(v) => visitor.visit_bool(v),
            None => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.decode_string() {
            Some(v) => visitor.visit_string(v),
            None => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let data = self.decode_bytes().ok_or(self.invalid_type(&visitor))?;
        visitor.visit_byte_buf(data)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.is_null() || self.is_undefined() {
            visitor.visit_unit()
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.is_array() {
            visit_array(self, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.is_object() {
            visit_object(self, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.is_array() {
            visit_array(self, visitor)
        } else if self.is_object() {
            visit_object(self, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }
}

struct EnumDeserializer {
    variant: String,
    value: Option<Value>,
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

impl<'de> IntoDeserializer<'de, Error> for Value {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

struct VariantDeserializer {
    value: Option<Value>,
}

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Some(value) => Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(v) => {
                if v.is_array() {
                    if v.length().unwrap_or(0) == 0 {
                        visitor.visit_unit()
                    } else {
                        visit_array(v, visitor)
                    }
                } else {
                    Err(serde::de::Error::invalid_type(
                        v.unexpected(),
                        &"tuple variant",
                    ))
                }
            }
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(v) => {
                if v.is_object() {
                    visit_object(v, visitor)
                } else {
                    Err(serde::de::Error::invalid_type(
                        v.unexpected(),
                        &"struct variant",
                    ))
                }
            }
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

struct SeqDeserializer {
    iter: Iter,
    decoded_items: usize,
}

impl SeqDeserializer {
    fn new(vec: Value) -> Option<Self> {
        Some(SeqDeserializer {
            iter: vec.values()?,
            decoded_items: 0,
        })
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => {
                self.decoded_items += 1;
                seed.deserialize(value).map(Some)
            }
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

struct MapDeserializer {
    iter: PairIter,
    value: Option<Value>,
}

impl MapDeserializer {
    fn new(map: Value) -> Option<Self> {
        Some(MapDeserializer {
            iter: map.entries()?,
            value: None,
        })
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl Value {
    #[cold]
    fn invalid_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(self.unexpected(), exp)
    }

    #[cold]
    fn unexpected(&self) -> Unexpected {
        if self.is_null() || self.is_undefined() {
            return Unexpected::Unit;
        }
        if let Some(b) = self.decode_bool() {
            return Unexpected::Bool(b);
        }
        if self.is_number() {
            return Unexpected::Other("number");
        }
        if self.is_string() {
            return Unexpected::Other("string");
        }
        if self.is_array() {
            return Unexpected::Seq;
        }
        if self.is_object() {
            return Unexpected::Map;
        }
        return Unexpected::Other("other value");
    }
}

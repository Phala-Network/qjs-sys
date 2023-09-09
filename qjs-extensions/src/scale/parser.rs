use chumsky::{error::Error, prelude::*};

pub type String = tinyvec_string::TinyString<[u8; 24]>;

#[derive(Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    Bool,
    Str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumType {
    pub variants: Vec<(String, Option<usize>, Option<usize>)>,
}

impl EnumType {
    pub fn new(variants: Vec<(String, Option<usize>, Option<usize>)>) -> Self {
        Self { variants }
    }
}

impl core::str::FromStr for PrimitiveType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "u8" => Ok(PrimitiveType::U8),
            "u16" => Ok(PrimitiveType::U16),
            "u32" => Ok(PrimitiveType::U32),
            "u64" => Ok(PrimitiveType::U64),
            "u128" => Ok(PrimitiveType::U128),
            "i8" => Ok(PrimitiveType::I8),
            "i16" => Ok(PrimitiveType::I16),
            "i32" => Ok(PrimitiveType::I32),
            "i64" => Ok(PrimitiveType::I64),
            "i128" => Ok(PrimitiveType::I128),
            "bool" => Ok(PrimitiveType::Bool),
            "str" => Ok(PrimitiveType::Str),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScaleType {
    Primitive(PrimitiveType),
    Compact(usize),
    Seq(usize),
    Tuple(Vec<usize>),
    Array(usize, usize),
    Enum(EnumType),
    Struct(Vec<(String, usize)>),
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<ScaleType>, extra::Err<Simple<'a, char>>> {
    let number = text::int(10).map(|s: &str| s.parse::<usize>().unwrap());
    let primitive_def = just("#")
        .ignore_then(text::ident())
        .try_map(|s: &str, span| {
            s.parse()
                .map(ScaleType::Primitive)
                .or(Err(Error::<&str>::expected_found(
                    [],
                    s.chars().next().map(Into::into),
                    span,
                )))
        });
    let compact_def = just("@")
        .ignore_then(number)
        .map(|len| ScaleType::Compact(len));
    let tuple_def = just("(")
        .ignore_then(
            number
                .separated_by(just(","))
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(")"))
        .map(|ids| ScaleType::Tuple(ids));
    let array_def = just("[")
        .ignore_then(number.then_ignore(just(";")).then(number))
        .then_ignore(just("]"))
        .map(|(tid, len)| ScaleType::Array(tid, len));
    let seq_def = just("[")
        .ignore_then(number)
        .then_ignore(just("]"))
        .map(|len| ScaleType::Seq(len));
    let enum_variant = text::ident()
        .map(String::from)
        .then(just(":").ignore_then(number.or_not()).or_not())
        .then(just(":").ignore_then(number).or_not())
        .map(|((name, t), i)| (name, t.flatten(), i));
    let enum_def = just("<")
        .ignore_then(
            enum_variant
                .separated_by(just(","))
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .map(|vec| ScaleType::Enum(EnumType::new(vec)))
        .then_ignore(just(">"));
    let struct_field = text::ident()
        .map(String::from)
        .then(just(":").ignore_then(number))
        .map(|(name, tid)| (name, tid));
    let struct_def = just("{")
        .ignore_then(struct_field.separated_by(just(",")).collect::<Vec<_>>())
        .then_ignore(just("}"))
        .map(|vec| ScaleType::Struct(vec));
    choice((
        primitive_def,
        compact_def,
        seq_def,
        array_def,
        tuple_def,
        enum_def,
        struct_def,
    ))
    .separated_by(text::newline())
    .collect::<Vec<_>>()
    .then_ignore(text::newline().or_not())
    .then_ignore(end())
}

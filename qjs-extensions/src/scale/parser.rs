use alloc::vec::Vec;
use chumsky::{error::Error, prelude::*};

pub type String = tinyvec_string::TinyString<[u8; 24]>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl PrimitiveType {
    pub fn from_str(s: &str) -> Option<&'static PrimitiveType> {
        let ty = match s {
            "u8" => &PrimitiveType::U8,
            "u16" => &PrimitiveType::U16,
            "u32" => &PrimitiveType::U32,
            "u64" => &PrimitiveType::U64,
            "u128" => &PrimitiveType::U128,
            "i8" => &PrimitiveType::I8,
            "i16" => &PrimitiveType::I16,
            "i32" => &PrimitiveType::I32,
            "i64" => &PrimitiveType::I64,
            "i128" => &PrimitiveType::I128,
            "bool" => &PrimitiveType::Bool,
            "str" => &PrimitiveType::Str,
            _ => return None,
        };
        Some(ty)
    }
}

impl core::str::FromStr for PrimitiveType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).cloned().ok_or(())
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
    let number = text::int(10).try_map(|s: &str, span| {
        s.parse::<usize>().or(Err(Error::<&str>::expected_found(
            [],
            s.chars().next().map(Into::into),
            span,
        )))
    });
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
    let separator = text::whitespace().then(just(";").then(text::whitespace()).or_not());
    choice((
        primitive_def,
        compact_def,
        seq_def,
        array_def,
        tuple_def,
        enum_def,
        struct_def,
    ))
    .separated_by(separator)
    .collect::<Vec<_>>()
    .padded()
    .then_ignore(end())
}

#[test]
fn test_parser() {
    let input = "\n#u8;@2[3](4,5)<foo:6:7,bar::9,baz:3,quz>\n{foo:10,bar:11}()";
    let result = parser().parse(input).into_result();
    println!("{result:?}");
    assert!(result.is_ok());
}

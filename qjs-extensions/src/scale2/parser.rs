use alloc::vec::Vec;
use chumsky::{error::Error, prelude::*};
use core::fmt;
use tinyvec_string::TinyString;

//use crate::scale::PrimitiveType;

pub type String = TinyString<[u8; 24]>;

type Span = SimpleSpan<usize>;

#[derive(Clone, Debug, PartialEq)]
enum Token<'src> {
    Num(u32),
    Op(char),
    Ident(&'src str),
}

impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Num(n) => write!(f, "{}", n),
            Token::Op(c) => write!(f, "{}", c),
            Token::Ident(s) => write!(f, "{}", s),
        }
    }
}

fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Simple<'src, char, Span>>> {
    // A parser for numbers
    let num = text::int(10)
        .try_map(|s: &str, span| {
            s.parse::<u32>().or(Err(Error::<&str>::expected_found(
                [],
                s.chars().next().map(Into::into),
                span,
            )))
        })
        .map(Token::Num);
    // A parser for control characters (delimiters, semicolons, etc.)
    let op = one_of("=@:;,#()[]{}<>").map(Token::Op);
    // A parser for identifiers and keywords
    let ident = text::ident().map(Token::Ident);
    // A single token can be one of the above
    let token = num.or(op).or(ident);
    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();
    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Id {
    Name(String),
    Num(u32),
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self::Name(s.into())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self::Name(s)
    }
}

impl From<u32> for Id {
    fn from(n: u32) -> Self {
        Self::Num(n)
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub variants: Vec<(String, Option<Id>, Option<u32>)>,
}

impl Enum {
    pub fn new(variants: Vec<(String, Option<Id>, Option<u32>)>) -> Self {
        Self { variants }
    }
}

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

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    Compact(Id),
    Seq(Id),
    Tuple(Vec<Id>),
    Array(Id, u32),
    Enum(Enum),
    Struct(Vec<(String, Id)>),
    Alias(Id),
}

impl Type {
    pub fn primitive(s: &str) -> Option<&'static Self> {
        match s {
            "u8" => Some(&Self::Primitive(PrimitiveType::U8)),
            "u16" => Some(&Self::Primitive(PrimitiveType::U16)),
            "u32" => Some(&Self::Primitive(PrimitiveType::U32)),
            "u64" => Some(&Self::Primitive(PrimitiveType::U64)),
            "u128" => Some(&Self::Primitive(PrimitiveType::U128)),
            "i8" => Some(&Self::Primitive(PrimitiveType::I8)),
            "i16" => Some(&Self::Primitive(PrimitiveType::I16)),
            "i32" => Some(&Self::Primitive(PrimitiveType::I32)),
            "i64" => Some(&Self::Primitive(PrimitiveType::I64)),
            "i128" => Some(&Self::Primitive(PrimitiveType::I128)),
            "bool" => Some(&Self::Primitive(PrimitiveType::Bool)),
            "str" => Some(&Self::Primitive(PrimitiveType::Str)),
            _ => None,
        }
    }

    pub fn is_alias(&self) -> bool {
        matches!(self, Self::Alias(_))
    }
}

impl From<PrimitiveType> for Type {
    fn from(ty: PrimitiveType) -> Self {
        Self::Primitive(ty)
    }
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    pub name: Option<String>,
    pub ty: Type,
}

type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token<'src>, Span, &'tokens [(Token<'src>, Span)]>;

fn type_parser<'tokens, 'src: 'tokens, E>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Type, E> + Clone
where
    E: extra::ParserExtra<'tokens, ParserInput<'tokens, 'src>>,
{
    use Token::*;
    let ident = select! { Ident(ident) => String::from(ident) };
    let tid = select! {
        Ident(ident) => Id::Name(ident.into()),
        Num(n) => Id::Num(n),
    };
    let num = select! { Num(v) => v };
    // A list of type identifiers
    let tids = tid
        .separated_by(just(Op(',')))
        .allow_trailing()
        .collect::<Vec<_>>();
    let compact_def = just(Op('@')).ignore_then(tid).map(|tid| Type::Compact(tid));
    let tuple_def = just(Op('('))
        .ignore_then(tids)
        .then_ignore(just(Op(')')))
        .map(|ty| Type::Tuple(ty));
    let array_def = just(Op('['))
        .ignore_then(tid.then_ignore(just(Op(';'))).then(num))
        .then_ignore(just(Op(']')))
        .map(|(ty, len)| Type::Array(ty, len));
    let seq_def = just(Op('['))
        .ignore_then(tid)
        .then_ignore(just(Op(']')))
        .map(|len| Type::Seq(len));
    let enum_variant = ident
        .then(just(Op(':')).ignore_then(tid.or_not()).or_not())
        .then(just(Op(':')).ignore_then(num).or_not())
        .map(|((name, t), i)| (name, t.flatten(), i));
    let enum_def = just(Op('<'))
        .ignore_then(
            enum_variant
                .separated_by(just(Op(',')))
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .map(|vec| Type::Enum(Enum::new(vec)))
        .then_ignore(just(Op('>')));
    let struct_field = ident
        .then(just(Op(':')).ignore_then(tid))
        .map(|(name, tid)| (name, tid));
    let struct_def = just(Op('{'))
        .ignore_then(
            struct_field
                .separated_by(just(Op(',')))
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(Op('}')))
        .map(|vec| Type::Struct(vec));
    let alias_def = tid.map(Type::Alias);
    let primitive_types = choice((
        just(Ident("u8")).map(|_| PrimitiveType::U8),
        just(Ident("u16")).map(|_| PrimitiveType::U16),
        just(Ident("u32")).map(|_| PrimitiveType::U32),
        just(Ident("u64")).map(|_| PrimitiveType::U64),
        just(Ident("u128")).map(|_| PrimitiveType::U128),
        just(Ident("i8")).map(|_| PrimitiveType::I8),
        just(Ident("i16")).map(|_| PrimitiveType::I16),
        just(Ident("i32")).map(|_| PrimitiveType::I32),
        just(Ident("i64")).map(|_| PrimitiveType::I64),
        just(Ident("i128")).map(|_| PrimitiveType::I128),
        just(Ident("bool")).map(|_| PrimitiveType::Bool),
        just(Ident("str")).map(|_| PrimitiveType::Str),
    ));
    let primitive_def = just(Op('#'))
        .ignore_then(primitive_types)
        .map(|ty| Type::Primitive(ty));
    choice((
        primitive_def,
        alias_def,
        compact_def,
        seq_def,
        array_def,
        tuple_def,
        enum_def,
        struct_def,
    ))
}

fn parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Vec<TypeDef>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    use Token::*;
    let ty = type_parser();
    let ident = select! { Ident(ident) => String::from(ident) };
    let stmt = ident
        .then_ignore(just(Op('=')))
        .or_not()
        .then(ty)
        .map(|(name, ty)| TypeDef { name, ty });
    stmt.separated_by(just(Op(';')).or_not())
        .allow_trailing()
        .collect::<Vec<_>>()
        .then_ignore(end())
}

pub fn parse_types(src: &str) -> js::Result<Vec<TypeDef>> {
    let tokens = lexer()
        .parse(src)
        .into_result()
        .map_err(super::to_js_error)?;
    let ast = parser()
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_result()
        .map_err(super::to_js_error)?;
    Ok(ast)
}

#[test]
fn it_works() {
    let src = "foo=[u8;32];bar=(u8,foo)";
    let tokens = lexer().parse(src).unwrap();
    let ast = parser()
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_result();
    println!("{:#?}", ast);
    assert!(ast.is_ok());
}

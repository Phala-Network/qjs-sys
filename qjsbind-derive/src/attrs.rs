use std::borrow::Cow;

use syn::{DeriveInput, Error, ExprPath, Field, Ident, LitStr, Path, Result};

#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum RenameAll {
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
    Keep,
}

impl RenameAll {
    pub fn parse(lit: &LitStr) -> Result<RenameAll> {
        match lit.value().as_str() {
            "lowercase" => Ok(RenameAll::LowerCase),
            "UPPERCASE" => Ok(RenameAll::UpperCase),
            "PascalCase" => Ok(RenameAll::PascalCase),
            "camelCase" => Ok(RenameAll::CamelCase),
            "snake_case" => Ok(RenameAll::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(RenameAll::ScreamingSnakeCase),
            "kebab-case" => Ok(RenameAll::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(RenameAll::ScreamingKebabCase),
            "keep" => Ok(RenameAll::Keep),
            _ => Err(Error::new_spanned(lit, "invalid value")),
        }
    }
    pub fn rename(&self, ident: &Ident) -> Ident {
        let name = ident.to_string();
        let renamed = match self {
            RenameAll::Keep => name,
            RenameAll::LowerCase | RenameAll::SnakeCase => name.to_ascii_lowercase(),
            RenameAll::UpperCase | RenameAll::ScreamingSnakeCase => name.to_ascii_uppercase(),
            RenameAll::PascalCase => {
                let mut pascal = String::new();
                let mut capitalize = true;
                for ch in name.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        pascal.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        pascal.push(ch);
                    }
                }
                pascal
            }
            RenameAll::CamelCase => {
                let mut camel = String::new();
                let mut capitalize = false;
                for ch in name.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        camel.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        camel.push(ch);
                    }
                }
                camel
            }
            RenameAll::KebabCase => name.replace('_', "-"),
            RenameAll::ScreamingKebabCase => name.replace('_', "-").to_ascii_uppercase(),
        };
        Ident::new(&renamed, ident.span())
    }
}

#[derive(Clone)]
pub enum TypeDefault {
    Implicit,
    Explicit(ExprPath),
}

pub struct ContainerAttrs<'a> {
    ident: &'a Ident,
    rename_all: Option<RenameAll>,
    allow_default: bool,
}

pub(crate) fn respan(
    span: proc_macro2::Span,
    stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    stream
        .into_iter()
        .map(|mut token| {
            if let proc_macro2::TokenTree::Group(g) = &mut token {
                *g = proc_macro2::Group::new(g.delimiter(), respan(span, g.stream()));
            }
            token.set_span(span);
            token
        })
        .collect()
}

fn parse_lit_into_expr_path(lit: &LitStr) -> Result<ExprPath> {
    let token_stream = syn::parse_str(&lit.value())?;
    syn::parse2(respan(lit.span(), token_stream))
}

impl<'a> ContainerAttrs<'a> {
    pub fn of(input: &'a DeriveInput) -> Result<ContainerAttrs<'a>> {
        let mut rv = ContainerAttrs {
            ident: &input.ident,
            rename_all: None,
            allow_default: false,
        };

        for attr in input.attrs.iter() {
            if !attr.path().is_ident("qjs") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename_all") {
                    ensure_none!(rv.rename_all, meta.path, "duplicate rename_all attribute");
                    let lit: LitStr = meta.value()?.parse()?;
                    rv.rename_all = Some(RenameAll::parse(&lit)?);
                } else if meta.path.is_ident("default") {
                    rv.allow_default = true;
                } else {
                    syn_bail!(meta.path, "unsupported attribute");
                }
                Ok(())
            })?;
        }
        Ok(rv)
    }

    pub fn get_field_js_name(&self, field: &Field) -> Ident {
        let ident = field.ident.as_ref().expect("No field name found").clone();
        let ident = trim_rust_raw(ident);
        if let Some(rename_all) = self.rename_all {
            rename_all.rename(&ident)
        } else {
            ident
        }
    }

    pub fn ident(&self) -> &Ident {
        self.ident
    }

    pub fn allow_default(&self) -> bool {
        self.allow_default
    }
}

pub fn trim_rust_raw(name: Ident) -> Ident {
    let name_str = name.to_string();
    if name_str.starts_with("r#") {
        Ident::new(&name_str[2..], name.span())
    } else {
        name
    }
}

pub struct FieldAttrs<'a> {
    field: &'a Field,
    rename: Option<String>,
    default: Option<TypeDefault>,
    as_bytes: bool,
    bytes_or_hex: bool,
}

impl<'a> FieldAttrs<'a> {
    pub fn of(field: &'a Field) -> Result<FieldAttrs<'a>> {
        let mut rv = FieldAttrs {
            field,
            rename: None,
            default: None,
            as_bytes: false,
            bytes_or_hex: false,
        };

        for attr in field.attrs.iter() {
            if !attr.path().is_ident("qjs") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    ensure_none!(rv.rename, meta.path, "duplicate rename attribute");
                    let lit: LitStr = meta.value()?.parse()?;
                    rv.rename = Some(lit.value());
                } else if meta.path.is_ident("default") {
                    ensure_none!(rv.default, meta.path, "duplicate default attribute");
                    if let Ok(value) = meta.value() {
                        let lit: LitStr = value.parse()?;
                        let path = parse_lit_into_expr_path(&lit)?;
                        rv.default = Some(TypeDefault::Explicit(path));
                    } else {
                        rv.default = Some(TypeDefault::Implicit);
                    }
                } else if meta.path.is_ident("as_bytes") {
                    if rv.bytes_or_hex || rv.as_bytes {
                        syn_bail!(meta.path, "duplicate as_bytes attribute");
                    }
                    rv.as_bytes = true;
                } else if meta.path.is_ident("bytes_or_hex") {
                    if rv.bytes_or_hex || rv.as_bytes {
                        syn_bail!(meta.path, "duplicate bytes_or_hex attribute");
                    }
                    rv.bytes_or_hex = true;
                } else {
                    syn_bail!(meta.path, "unsupported attribute");
                }
                Ok(())
            })?;
        }
        Ok(rv)
    }

    pub fn field(&self) -> &Field {
        self.field
    }

    pub fn js_name(&self, container_attrs: &ContainerAttrs) -> Cow<'_, str> {
        self.rename
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| {
                container_attrs
                    .get_field_js_name(self.field)
                    .to_string()
                    .into()
            })
    }

    pub fn as_bytes(&self) -> bool {
        self.as_bytes
    }

    pub fn bytes_or_hex(&self) -> bool {
        self.bytes_or_hex
    }

    pub fn decoder_fn(&self, crate_qjsbind: &Ident) -> Path {
        if self.as_bytes {
            syn::parse_quote!(#crate_qjsbind::decode_as_bytes)
        } else if self.bytes_or_hex {
            syn::parse_quote!(#crate_qjsbind::decode_as_bytes_maybe_hex)
        } else {
            syn::parse_quote!(FromJsValue::from_js_value)
        }
    }

    pub fn default_fn(&self) -> Option<ExprPath> {
        match &self.default {
            Some(TypeDefault::Implicit) => Some(syn::parse_quote!(Default::default)),
            Some(TypeDefault::Explicit(path)) => Some(path.clone()),
            None => None,
        }
    }
}

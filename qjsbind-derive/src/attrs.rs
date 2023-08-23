use std::borrow::Cow;

#[derive(Copy, Clone)]
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
}

impl RenameAll {
    fn parse(lit: &syn::Lit) -> Result<RenameAll, syn::Error> {
        if let syn::Lit::Str(s) = &lit {
            match s.value().as_str() {
                "lowercase" => Ok(RenameAll::LowerCase),
                "UPPERCASE" => Ok(RenameAll::UpperCase),
                "PascalCase" => Ok(RenameAll::PascalCase),
                "camelCase" => Ok(RenameAll::CamelCase),
                "snake_case" => Ok(RenameAll::SnakeCase),
                "SCREAMING_SNAKE_CASE" => Ok(RenameAll::ScreamingSnakeCase),
                "kebab-case" => Ok(RenameAll::KebabCase),
                "SCREAMING-KEBAB-CASE" => Ok(RenameAll::ScreamingKebabCase),
                _ => Err(syn::Error::new_spanned(lit, "")),
            }
        } else {
            Err(syn::Error::new_spanned(lit, "rename expects a string"))
        }
    }
}

#[derive(Clone)]
pub enum TypeDefault {
    Implicit,
    Explicit(syn::ExprPath),
}

pub struct ContainerAttrs<'a> {
    ident: &'a syn::Ident,
    rename_all: Option<RenameAll>,
}

pub fn get_meta_items(attr: &syn::Attribute) -> syn::Result<Vec<syn::NestedMeta>> {
    if !attr.path.is_ident("qjsbind") {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(syn::Meta::List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(_) => Err(syn::Error::new_spanned(attr, "expected #[qjsbind(...)]")),
        Err(err) => Err(err),
    }
}

fn respan(stream: proc_macro2::TokenStream, span: proc_macro2::Span) -> proc_macro2::TokenStream {
    stream
        .into_iter()
        .map(|mut token| {
            if let proc_macro2::TokenTree::Group(g) = &mut token {
                *g = proc_macro2::Group::new(g.delimiter(), respan(g.stream(), span));
            }
            token.set_span(span);
            token
        })
        .collect()
}

fn get_lit_str(attr_name: &str, lit: &syn::Lit) -> syn::Result<String> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit.value())
    } else {
        Err(syn::Error::new_spanned(
            lit,
            format!(
                "expected attribute to be a string: `{} = \"...\"`",
                attr_name,
            ),
        ))
    }
}

fn parse_lit_into_expr_path(attr_name: &str, lit: &syn::Lit) -> syn::Result<syn::ExprPath> {
    let string = get_lit_str(attr_name, lit)?;
    let token_stream = syn::parse_str(&string)?;
    syn::parse2(respan(token_stream, lit.span()))
}

impl<'a> ContainerAttrs<'a> {
    pub fn of(input: &'a syn::DeriveInput) -> syn::Result<ContainerAttrs<'a>> {
        let mut rv = ContainerAttrs {
            ident: &input.ident,
            rename_all: None,
        };

        for meta_item in input.attrs.iter().flat_map(get_meta_items).flatten() {
            if let syn::NestedMeta::Meta(meta) = meta_item {
                match &meta {
                    syn::Meta::NameValue(nv) if nv.path.is_ident("rename_all") => {
                        if rv.rename_all.is_some() {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "duplicate rename_all attribute",
                            ));
                        }
                        rv.rename_all = Some(RenameAll::parse(&nv.lit)?);
                    }
                    _ => return Err(syn::Error::new_spanned(meta, "unsupported attribute")),
                }
            } else {
                return Err(syn::Error::new_spanned(meta_item, "unsupported attribute"));
            }
        }

        Ok(rv)
    }

    pub fn get_field_name(&self, field: &syn::Field) -> String {
        let name = field.ident.as_ref().unwrap().to_string();
        if let Some(rename_all) = self.rename_all {
            match rename_all {
                RenameAll::LowerCase | RenameAll::SnakeCase => name,
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
                RenameAll::KebabCase => name.replace("_", "-"),
                RenameAll::ScreamingKebabCase => name.replace("_", "-").to_ascii_uppercase(),
            }
        } else {
            name
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        self.ident
    }
}

pub struct FieldAttrs<'a> {
    field: &'a syn::Field,
    rename: Option<String>,
    default: Option<TypeDefault>,
    as_bytes: bool,
}

impl<'a> FieldAttrs<'a> {
    pub fn of(field: &'a syn::Field) -> syn::Result<FieldAttrs<'a>> {
        let mut rv = FieldAttrs {
            field,
            rename: None,
            default: None,
            as_bytes: false,
        };

        for meta_item in field.attrs.iter().flat_map(get_meta_items).flatten() {
            if let syn::NestedMeta::Meta(meta) = meta_item {
                match &meta {
                    syn::Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                        if rv.rename.is_some() {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "duplicate rename attribute",
                            ));
                        }
                        rv.rename = Some(get_lit_str("rename", &nv.lit)?);
                    }
                    syn::Meta::NameValue(nv) if nv.path.is_ident("default") => {
                        if rv.default.is_some() {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "duplicate default attribute",
                            ));
                        }
                        rv.default = Some(TypeDefault::Explicit(parse_lit_into_expr_path(
                            "default", &nv.lit,
                        )?));
                    }
                    syn::Meta::Path(path) if path.is_ident("as_bytes") => {
                        if rv.as_bytes {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "duplicate as_bytes attribute",
                            ));
                        }
                        rv.as_bytes = true;
                    }
                    syn::Meta::Path(path) if path.is_ident("default") => {
                        if rv.default.is_some() {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "duplicate default attribute",
                            ));
                        }
                        rv.default = Some(TypeDefault::Implicit);
                    }
                    _ => return Err(syn::Error::new_spanned(meta, "unsupported attribute")),
                }
            } else {
                return Err(syn::Error::new_spanned(meta_item, "unsupported attribute"));
            }
        }

        Ok(rv)
    }

    pub fn field(&self) -> &syn::Field {
        self.field
    }

    pub fn name(&self, container_attrs: &ContainerAttrs) -> Cow<'_, str> {
        self.rename
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| container_attrs.get_field_name(self.field).into())
    }

    pub fn as_bytes(&self) -> bool {
        self.as_bytes
    }

    pub fn decoder_fn(&self, crate_qjsbind: &syn::Ident) -> syn::Path {
        if self.as_bytes {
            syn::parse_quote!(#crate_qjsbind::decode_as_bytes)
        } else {
            syn::parse_quote!(FromJsValue::from_js_value)
        }
    }

    pub fn default_fn(&self) -> Option<syn::ExprPath> {
        match &self.default {
            Some(TypeDefault::Implicit) => Some(syn::parse_quote!(Default::default)),
            Some(TypeDefault::Explicit(path)) => Some(path.clone()),
            None => None,
        }
    }
}

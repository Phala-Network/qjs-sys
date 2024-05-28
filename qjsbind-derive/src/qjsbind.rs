use proc_macro2::TokenStream;
use std::collections::BTreeMap;
use syn::{Attribute, Field, Ident, ImplItemFn, Item, ItemMod, ItemStruct, LitStr, Path, Result};
use template_quote::{quote, ToTokens};

macro_rules! extract_qjs_attrs {
    ($item: ident) => {{
        let (qjs_attrs, attrs) = partition_attributes($item.attrs.clone())?;
        $item.attrs = attrs;
        if (qjs_attrs.is_empty()) {
            None
        } else {
            Some(qjs_attrs)
        }
    }};
}

struct Mod {
    classes: BTreeMap<String, Class>,
}

impl ToTokens for Mod {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!("Implement ToTokens for Mod")
    }
}

struct Class {
    name: Ident,
    derived_properties: Vec<DerivedProperty>,
    methods: Vec<Method>,
    attrs: ClassAttrs,
}

struct ClassAttrs {
    js_name: Option<LitStr>,
    rename_all: Option<LitStr>,
}

struct DerivedProperty {
    name: Ident,
    ty: Path,
    attrs: FieldAttrs,
}

struct FieldAttrs {
    rename: Option<LitStr>,
    is_getter: bool,
    is_setter: bool,
}

struct Method {
    name: Ident,
    fn_item: ImplItemFn,
    attrs: FnAttrs,
}

struct FnAttrs {
    js_name: Option<LitStr>,
    is_getter: bool,
    is_setter: bool,
}

impl Class {
    fn from_struct(item_struct: &mut ItemStruct) -> Result<Option<Self>> {
        let Some(qjs_attrs) = extract_qjs_attrs!(item_struct) else {
            return Ok(None);
        };
        let name = item_struct.ident.clone();
        let attrs = ClassAttrs::from_attributes(&qjs_attrs)?;

        let mut derived_properties = Vec::new();

        for field in &mut item_struct.fields.iter_mut() {
            if let Some(derived_prop) = DerivedProperty::from_field(field)? {
                derived_properties.push(derived_prop);
            }
        }

        Ok(Some(Self {
            name,
            derived_properties,
            methods: Vec::new(),
            attrs,
        }))
    }
}

impl ClassAttrs {
    fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut js_name = None;
        let mut rename_all = None;
        let mut is_class = false;

        for attr in attrs {
            if attr.path().is_ident("qjs") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("class") {
                        is_class = true;
                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("js_name") {
                                js_name = Some(meta.value()?.parse::<LitStr>()?);
                            } else if meta.path.is_ident("rename_all") {
                                rename_all = Some(meta.value()?.parse::<LitStr>()?);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    meta.path,
                                    "Unknown attribute",
                                ));
                            }
                            Ok(())
                        })?;
                    } else {
                        return Err(syn::Error::new_spanned(meta.path, "Unknown attribute"));
                    }
                    Ok(())
                })?;
            }
        }

        if !is_class {
            return Err(syn::Error::new_spanned(
                attrs[0].clone(),
                "Expected `class` attribute",
            ));
        }
        Ok(Self {
            js_name,
            rename_all,
        })
    }
}

impl DerivedProperty {
    fn from_field(field: &mut Field) -> Result<Option<Self>> {
        let Some(qjs_attrs) = extract_qjs_attrs!(field) else {
            return Ok(None);
        };
        let attrs = FieldAttrs::from_attributes(&qjs_attrs)?;
        if attrs.is_getter || attrs.is_setter {
            let name = field.ident.clone().unwrap();
            let ty = if let syn::Type::Path(ty) = &field.ty {
                ty.path.clone()
            } else {
                return Err(syn::Error::new_spanned(&field.ty, "Expected path type"));
            };
            Ok(Some(Self { name, ty, attrs }))
        } else {
            Ok(None)
        }
    }
}

impl FieldAttrs {
    fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut rename = None;
        let mut is_getter = false;
        let mut is_setter = false;

        for attr in attrs {
            if attr.path().is_ident("qjs") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("getter") {
                        is_getter = true;
                    } else if meta.path.is_ident("setter") {
                        is_setter = true;
                    } else if meta.path.is_ident("rename") {
                        rename = Some(meta.value()?.parse::<LitStr>()?);
                    } else {
                        return Err(syn::Error::new_spanned(meta.path, "Unknown attribute"));
                    }
                    Ok(())
                })?;
            }
        }

        Ok(Self {
            rename,
            is_getter,
            is_setter,
        })
    }
}

impl Method {
    fn from_item_fn(item_fn: &mut ImplItemFn) -> Result<Option<Self>> {
        let Some(qjs_attrs) = extract_qjs_attrs!(item_fn) else {
            return Ok(None);
        };
        let attrs = FnAttrs::from_attributes(&qjs_attrs)?;
        let name = item_fn.sig.ident.clone();
        Ok(Some(Self {
            name,
            fn_item: item_fn.clone(),
            attrs,
        }))
    }
}

impl FnAttrs {
    fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut js_name = None;
        let mut is_getter = false;
        let mut is_setter = false;
        let mut is_method = false;

        for attr in attrs {
            if attr.path().is_ident("qjs") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("method") {
                        is_method = true;
                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("getter") {
                                is_getter = true;
                            } else if meta.path.is_ident("setter") {
                                is_setter = true;
                            } else if meta.path.is_ident("js_name") {
                                js_name = Some(meta.value()?.parse::<LitStr>()?);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    meta.path,
                                    "Unknown attribute",
                                ));
                            }
                            Ok(())
                        })?;
                    } else {
                        return Err(syn::Error::new_spanned(meta.path, "Unknown attribute"));
                    }
                    Ok(())
                })?;
            }
        }

        if !is_method {
            return Err(syn::Error::new_spanned(
                attrs[0].clone(),
                "Expected `method` attribute",
            ));
        }

        Ok(Self {
            js_name,
            is_getter,
            is_setter,
        })
    }
}

impl Mod {
    fn from_mod(item_mod: &mut ItemMod) -> Result<Self> {
        let mut classes = BTreeMap::new();
        if let Some((_, ref mut items)) = item_mod.content {
            for item in items {
                match item {
                    Item::Struct(item_struct) => {
                        let Some(class) = Class::from_struct(item_struct)? else {
                            continue;
                        };
                        classes.insert(class.name.to_string(), class);
                    }
                    Item::Impl(item_impl) => {
                        if item_impl.trait_.is_some() {
                            continue;
                        }
                        let syn::Type::Path(ty) = item_impl.self_ty.as_ref() else {
                            continue;
                        };
                        let Some(ident) = ty.path.get_ident() else {
                            continue;
                        };
                        let Some(for_class) = classes.get_mut(&ident.to_string()) else {
                            continue;
                        };

                        for item in &mut item_impl.items {
                            match item {
                                syn::ImplItem::Fn(item_method) => {
                                    let Some(method) = Method::from_item_fn(item_method)? else {
                                        continue;
                                    };
                                    for_class.methods.push(method);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(Self { classes })
    }
}

pub(crate) fn patch(config: TokenStream, input: TokenStream) -> TokenStream {
    match patch_or_err(config, input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn patch_or_err(_config: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut the_mod: ItemMod = syn::parse2(input)?;
    let crate_js = crate::find_crate_name("qjsbind")?;
    let qjs_mod = Mod::from_mod(&mut the_mod)?;
    Ok(quote! {
        #the_mod
        #qjs_mod
    })
}

pub fn partition_attributes<I>(attrs: I) -> Result<(Vec<Attribute>, Vec<Attribute>)>
where
    I: IntoIterator<Item = Attribute>,
{
    use itertools::{Either, Itertools as _};
    let (qjs_attrs, others) = attrs.into_iter().partition_map(|attr| {
        if attr.path().is_ident("qjs") {
            Either::Left(attr)
        } else {
            Either::Right(attr)
        }
    });
    Ok((qjs_attrs, others))
}

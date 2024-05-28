use syn::{FnArg, Pat};

use super::*;

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

    fn validate(&self) -> Result<()> {
        if self
            .methods
            .iter()
            .find(|m| m.attrs.fn_type == FnType::Constructor)
            .is_none()
        {
            return Err(syn::Error::new_spanned(
                self.name.clone(),
                "Missing constructor method",
            ));
        }
        Ok(())
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
                                let lit_rename_all = meta.value()?.parse::<LitStr>()?;
                                rename_all = Some(RenameAll::parse(&lit_rename_all)?);
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
            let Some(name) = field.ident.clone() else {
                return Err(syn::Error::new_spanned(field, "Expected named field"));
            };
            let ty = field.ty.clone();
            Ok(Some(Self { name, ty, attrs }))
        } else {
            Ok(None)
        }
    }
}

impl FieldAttrs {
    fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut js_name = None;
        let mut is_getter = false;
        let mut is_setter = false;

        for attr in attrs {
            if attr.path().is_ident("qjs") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("getter") {
                        is_getter = true;
                    } else if meta.path.is_ident("setter") {
                        is_setter = true;
                    } else if meta.path.is_ident("js_name") {
                        js_name = Some(meta.value()?.parse::<LitStr>()?);
                    } else {
                        return Err(syn::Error::new_spanned(meta.path, "Unknown attribute"));
                    }
                    Ok(())
                })?;
            }
        }

        Ok(Self {
            js_name,
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
        let mut is_mut_self = false;
        let mut inputs = item_fn.sig.inputs.iter();
        if attrs.fn_type != FnType::Constructor {
            let Some(first) = inputs.next() else {
                return Err(syn::Error::new_spanned(
                    name,
                    "Expected at least one argument",
                ));
            };
            let FnArg::Receiver(celf) = first else {
                return Err(syn::Error::new_spanned(
                    first,
                    "Expected a receiver argument",
                ));
            };
            if celf.mutability.is_some() {
                is_mut_self = true;
            }
        }
        let mut args = vec![];
        for arg in inputs {
            let FnArg::Typed(pat) = arg else {
                return Err(syn::Error::new_spanned(arg, "Expected a typed argument"));
            };
            let ident = match &*pat.pat {
                Pat::Ident(ident) => ident.ident.clone(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        pat.pat.clone(),
                        "Expected an identifier",
                    ))
                }
            };
            args.push((ident, *pat.ty.clone()));
        }
        let return_ty = item_fn.sig.output.clone();
        // validate
        match attrs.fn_type {
            FnType::Getter => {
                if is_mut_self {
                    return Err(syn::Error::new_spanned(
                        name,
                        "Getter method cannot take `&mut self`",
                    ));
                }
                if !args.is_empty() {
                    return Err(syn::Error::new_spanned(
                        name,
                        "Getter method cannot take arguments",
                    ));
                }
            }
            FnType::Setter => {
                if !is_mut_self {
                    return Err(syn::Error::new_spanned(
                        name,
                        "Setter method must take `&mut self`",
                    ));
                }
                if args.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        name,
                        "Setter method must take exactly one argument",
                    ));
                }
            }
            FnType::Method | FnType::Constructor => (),
        }
        Ok(Some(Self {
            name,
            attrs,
            args,
            return_ty,
            is_mut: is_mut_self,
        }))
    }
}

impl FnAttrs {
    fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut js_name = None;
        let mut fn_type = None;

        for attr in attrs {
            if attr.path().is_ident("qjs") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("method") {
                        fn_type = Some(FnType::Method);
                    } else if meta.path.is_ident("getter") {
                        fn_type = Some(FnType::Getter);
                    } else if meta.path.is_ident("setter") {
                        fn_type = Some(FnType::Setter);
                    } else if meta.path.is_ident("constructor") {
                        fn_type = Some(FnType::Constructor);
                    } else if meta.path.is_ident("js_name") {
                        js_name = Some(meta.value()?.parse::<LitStr>()?);
                    } else {
                        return Err(syn::Error::new_spanned(meta.path, "Unknown attribute"));
                    }
                    Ok(())
                })?;
            }
        }

        let Some(fn_type) = fn_type else {
            return Err(syn::Error::new_spanned(
                attrs[0].clone(),
                "Expected exactly one of `getter`, `setter`, `method`, or `constructor`",
            ));
        };

        Ok(Self { js_name, fn_type })
    }
}

impl Mod {
    pub(crate) fn from_mod(item_mod: &mut ItemMod, js_crate: Option<Path>) -> Result<Self> {
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
        for cls in classes.values() {
            cls.validate()?;
        }
        Ok(Self { classes, js_crate })
    }
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

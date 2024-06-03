use syn::{FnArg, Pat, Type};

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
    fn from_struct(item_struct: &mut ItemStruct, js_crate: &syn::Path) -> Result<Option<Self>> {
        let Some(qjs_attrs) = extract_qjs_attrs!(item_struct) else {
            return Ok(None);
        };
        item_struct
            .attrs
            .push(syn::parse_quote!(#[derive(#js_crate::GcMark)]));
        let name = item_struct.ident.clone();
        let attrs = ClassAttrs::from_attributes(&qjs_attrs)?;

        let mut fields = vec![];
        for field in &mut item_struct.fields.iter_mut() {
            let qjs_property = DerivedProperty::from_field(field)?;
            fields.push(ClassField {
                field: field.clone(),
                qjs_property,
            });
        }

        Ok(Some(Self {
            name,
            methods: Vec::new(),
            attrs,
            fields,
            constructor: None,
        }))
    }

    fn validate(&self) -> Result<()> {
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
                        if meta.input.is_empty() {
                            return Ok(());
                        }
                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("js_name") {
                                ensure_none!(js_name, meta.path, "duplicate `js_name` attribute");
                                js_name = Some(meta.value()?.parse::<LitStr>()?);
                            } else if meta.path.is_ident("rename_all") {
                                let lit_rename_all = meta.value()?.parse::<LitStr>()?;
                                ensure_none!(
                                    rename_all,
                                    meta.path,
                                    "duplicate `rename_all` attribute"
                                );
                                rename_all = Some(RenameAll::parse(&lit_rename_all)?);
                            } else {
                                syn_bail!(meta.path, "unknown attribute");
                            }
                            Ok(())
                        })?;
                    } else {
                        syn_bail!(meta.path, "unknown attribute");
                    }
                    Ok(())
                })?;
            }
        }

        if !is_class {
            syn_bail!(attrs[0], "expected `class` attribute");
        }
        Ok(Self {
            js_name,
            rename_all: rename_all.or(Some(RenameAll::CamelCase)),
        })
    }
}

impl DerivedProperty {
    fn from_field(field: &mut Field) -> Result<Option<Self>> {
        let Some(qjs_attrs) = extract_qjs_attrs!(field) else {
            return Ok(None);
        };
        let attrs = FieldAttrs::from_attributes(&qjs_attrs)?;
        let Some(name) = field.ident.clone() else {
            syn_bail!(field, "expected named field");
        };
        let ty = field.ty.clone();
        Ok(Some(Self { name, ty, attrs }))
    }
}

impl FieldAttrs {
    fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut js_name = None;
        let mut getter = None;
        let mut setter = None;

        for attr in attrs {
            if attr.path().is_ident("qjs") {
                attr.parse_nested_meta(|meta| {
                    let Some(ident) = meta.path.get_ident() else {
                        syn_bail!(meta.path, "expected an identifier");
                    };
                    match ident.to_string().as_str() {
                        "getter" => {
                            getter = Some(ident.clone());
                        }
                        "setter" => {
                            setter = Some(ident.clone());
                        }
                        "js_name" => {
                            ensure_none!(js_name, meta.path, "duplicate `js_name` attribute");
                            js_name = Some(meta.value()?.parse::<LitStr>()?);
                        }
                        _ => {
                            syn_bail!(meta.path, "unknown attribute");
                        }
                    }
                    Ok(())
                })?;
            }
        }

        Ok(Self {
            js_name,
            getter,
            setter,
        })
    }
}

enum FnItem {
    Constructor(Constructor),
    Method(Method),
}

fn parse_fn_item(item_fn: &mut ImplItemFn) -> Result<Option<FnItem>> {
    let Some(qjs_attrs) = extract_qjs_attrs!(item_fn) else {
        return Ok(None);
    };
    match parse_fn_attributes(&qjs_attrs)? {
        FnAttrs::Method(attrs) => Method::from_item_fn(item_fn, attrs)
            .map(FnItem::Method)
            .map(Some),
        // Currently, None means it's a constructor
        FnAttrs::Constructor(attrs) => Constructor::from_item_fn(item_fn, attrs)
            .map(FnItem::Constructor)
            .map(Some),
    }
}

impl Method {
    fn from_item_fn(item_fn: &mut ImplItemFn, attrs: MethodAttrs) -> Result<Self> {
        let name = item_fn.sig.ident.clone();
        let args = parse_fn_args(item_fn.sig.inputs.iter_mut())?;
        if let Some(receiver) = &args.receiver {
            if !receiver.is_ref {
                syn_bail!(receiver.token, "expected a reference receiver");
            }
        }
        let is_static = args.receiver.is_none();
        let return_ty = item_fn.sig.output.clone();
        // validate
        match &attrs.fn_type {
            MethodType::Getter => {
                if !args.args.is_empty() {
                    syn_bail!(attrs.marker_token, "getter method cannot take arguments");
                }
            }
            MethodType::Setter => {
                if args.args.len() != 1 {
                    syn_bail!(
                        attrs.marker_token,
                        "setter method must take exactly one argument"
                    );
                }
            }
            _ => (),
        }
        Ok(Self {
            name,
            attrs,
            is_mut: args.receiver.as_ref().map_or(false, |r| r.is_mut),
            is_static,
            args,
            return_ty,
        })
    }
}

enum FnAttrs {
    Constructor(ConstructorAttrs),
    Method(MethodAttrs),
}

fn parse_fn_attributes(attrs: &[Attribute]) -> Result<FnAttrs> {
    let mut js_name = None;
    let mut fn_type = None;

    for attr in attrs {
        if attr.path().is_ident("qjs") {
            attr.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    syn_bail!(meta.path, "expected an identifier");
                };
                match ident.to_string().as_str() {
                    "method" => {
                        fn_type = Some((MethodType::Method, ident.clone()));
                    }
                    "getter" => {
                        fn_type = Some((MethodType::Getter, ident.clone()));
                    }
                    "setter" => {
                        fn_type = Some((MethodType::Setter, ident.clone()));
                    }
                    "constructor" => {
                        fn_type = Some((MethodType::Constructor, ident.clone()));
                    }
                    "js_name" => {
                        ensure_none!(js_name, meta.path, "duplicate `js_name` attribute");
                        js_name = Some(meta.value()?.parse::<LitStr>()?);
                    }
                    _ => {
                        syn_bail!(meta.path, "unknown attribute");
                    }
                }
                Ok(())
            })?;
        }
    }

    let Some((fn_type, marker_token)) = fn_type else {
        syn_bail!(
            attrs[0],
            "expected exactly one of `getter`, `setter`, `method`, or `constructor"
        );
    };
    match fn_type {
        MethodType::Constructor => {
            if js_name.is_some() {
                syn_bail!(js_name, "constructor cannot have `js_name` attribute");
            }
            Ok(FnAttrs::Constructor(ConstructorAttrs { marker_token }))
        }
        _ => Ok(FnAttrs::Method(MethodAttrs {
            js_name,
            fn_type,
            marker_token,
        })),
    }
}

fn parse_fn_args<'a>(mut inputs: impl Iterator<Item = &'a mut FnArg>) -> Result<Args> {
    let mut args = Args::default();
    let mut next = inputs.next();
    if let Some(FnArg::Receiver(receiver)) = next {
        args.receiver = Some(ArgSelf {
            is_ref: receiver.reference.is_some(),
            is_mut: receiver.mutability.is_some(),
            token: receiver.clone(),
        });
        next = inputs.next();
    }
    while let Some(arg) = next {
        let FnArg::Typed(pat) = arg else {
            syn_bail!(arg, "expected a typed argument");
        };
        let ident = match &*pat.pat {
            Pat::Ident(ident) => ident.ident.clone(),
            _ => {
                syn_bail!(pat.pat, "expected an identifier");
            }
        };
        let mut from_context = None;
        if let Some(qjs_attrs) = extract_qjs_attrs!(pat) {
            for attr in qjs_attrs {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("from_context") {
                        from_context = meta.path.get_ident().cloned();
                    } else {
                        syn_bail!(meta.path, "unknown attribute");
                    }
                    Ok(())
                })?
            }
        }
        args.args.push(Arg {
            name: ident,
            ty: *pat.ty.clone(),
            from_context,
        });
        next = inputs.next();
    }
    Ok(args)
}

impl Constructor {
    fn from_item_fn(item_fn: &mut ImplItemFn, attrs: ConstructorAttrs) -> Result<Self> {
        let name = item_fn.sig.ident.clone();
        let args = parse_fn_args(item_fn.sig.inputs.iter_mut())?;
        if args.receiver.is_some() {
            syn_bail!(item_fn.sig.inputs, "constructor cannot take `self`");
        }
        Ok(Self { name, args, attrs })
    }
}

impl Mod {
    pub(crate) fn from_mod(item_mod: &mut ItemMod, js_crate: Option<Path>) -> Result<Self> {
        let js_crate = match js_crate {
            Some(js_crate) => js_crate,
            None => crate::find_crate_name("qjsbind")?.into(),
        };
        let mut classes = BTreeMap::new();
        if let Some((_, ref mut items)) = item_mod.content {
            for item in items {
                match item {
                    Item::Struct(item_struct) => {
                        let Some(class) = Class::from_struct(item_struct, &js_crate)? else {
                            continue;
                        };
                        classes.insert(class.name.to_string(), class);
                    }
                    Item::Impl(item_impl) => {
                        if item_impl.trait_.is_some() {
                            continue;
                        }
                        let Type::Path(ty) = item_impl.self_ty.as_ref() else {
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
                                    let Some(method) = parse_fn_item(item_method)? else {
                                        continue;
                                    };
                                    match method {
                                        FnItem::Constructor(constructor) => {
                                            ensure_none!(
                                                for_class.constructor,
                                                constructor.attrs.marker_token,
                                                "Only one constructor is allowed per class"
                                            );
                                            for_class.constructor = Some(constructor);
                                        }
                                        FnItem::Method(method) => {
                                            for_class.methods.push(method);
                                        }
                                    }
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

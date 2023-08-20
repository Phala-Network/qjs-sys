use proc_macro2::TokenStream;

pub fn where_clause_with_bound(generics: &syn::Generics, bound: TokenStream) -> syn::WhereClause {
    let new_predicates = generics
        .type_params()
        .map::<syn::WherePredicate, _>(|param| {
            let param = &param.ident;
            syn::parse_quote!(#param : #bound)
        });

    let mut generics = generics.clone();
    generics
        .make_where_clause()
        .predicates
        .extend(new_predicates);
    generics.where_clause.unwrap()
}

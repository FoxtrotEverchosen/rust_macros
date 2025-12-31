// comp: mapping for_if_clause
//
// mapping: expression
//
// for_if_clause:
//  | 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

use syn;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};
use quote::quote;
use proc_macro2::{TokenStream as TokenStream2, };

//TODO: try to add handling of multiple for_if_clauses

struct Comp{
    mapping: Mapping,
    for_if_clause: ForIfClause
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Comp> {
        Ok(Self{
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            expression,
            conditions
        } = &self.for_if_clause;

        let conditions = conditions.iter().map(|c|{
            let inner = &c.0;
            quote!{#inner}
        });

        tokens.extend(quote!{
            core::iter::IntoIterator::into_iter(#expression).filter_map(|#pattern|{
                (true #(&& (#conditions))*).then(|| #mapping)
            } )
        })
    }
}

struct Mapping(syn::Expr);

impl syn::parse::Parse for Mapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Mapping> {
        Ok(Self(input.parse()?))
    }
}

impl quote::ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pattern,
    expression: syn::Expr,
    conditions: Vec<Condition>,
}

impl syn::parse::Parse for ForIfClause {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<ForIfClause> {
        _ = input.parse::<Token![for]>()?;
        let pattern = input.parse()?;
        _ = input.parse::<Token![in]>()?;
        let expression = input.parse()?;
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            pattern,
            expression,
            conditions })
    }
}

fn parse_zero_or_more<T: syn::parse::Parse>(input: ParseStream) -> Vec<T>{
    let mut result = Vec::new();
    while let Ok(item) = input.parse(){
        result.push(item);
    }
    result
}

struct Pattern(syn::Pat);

impl syn::parse::Parse for Pattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.call(syn::Pat::parse_single).map(Self)
    }
}

impl quote::ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct Condition(syn::Expr);

impl syn::parse::Parse for Condition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: Token![if] = input.parse()?;
        input.parse().map(Self)
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let c = parse_macro_input!(input as Comp);
    quote! { #c }.into()
}
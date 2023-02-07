use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, LitInt, Result, Token, Type};

use quote::quote;

struct Args {
    ty: Type,
    repeat: usize,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        let _: Token![;] = input.parse()?;
        let repeat = input.parse::<LitInt>()?.base10_parse()?;

        Ok(Args { ty, repeat })
    }
}

#[proc_macro]
pub fn repeat(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as Args);
    let ty = args.ty;
    let mut res = quote! {};
    for _ in 0..args.repeat {
        res = quote! { #res #ty, };
    }
    quote! { ( #res ) }.into()
}

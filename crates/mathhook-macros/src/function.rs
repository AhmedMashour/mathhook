//! Procedural function!() macro implementation
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, Token};

struct FunctionCall {
    name: Ident,
    args: Vec<Expr>,
}

impl Parse for FunctionCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let args = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let parsed_args = Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)?;
            parsed_args.into_iter().collect()
        } else if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "Expected comma before arguments",
            ));
        } else {
            Vec::new()
        };

        Ok(FunctionCall { name, args })
    }
}

pub(crate) fn function_impl(input: TokenStream) -> TokenStream {
    match syn::parse::<FunctionCall>(input) {
        Ok(func) => {
            let name_str = func.name.to_string();
            let args = &func.args;

            let tokens: TokenStream2 = quote! {
                mathhook_core::Expression::function(#name_str, vec![#(#args),*])
            };

            tokens.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

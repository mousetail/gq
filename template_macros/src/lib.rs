use dsl::{dsl_to_half_tokens, dsl_to_tokens};
use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr,
};

mod dsl;

struct Template {
    value: LitStr,
}

impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Template> {
        let value: LitStr = input.parse()?;

        Ok(Template { value })
    }
}

#[proc_macro]
pub fn fragment(input: TokenStream) -> TokenStream {
    let Template { value } = parse_macro_input!(input as Template);

    let inner_value = value.value();

    let tokens = dsl_to_tokens(&inner_value);

    TokenStream::from(tokens)
}

#[proc_macro]
pub fn half_fragment(input: TokenStream) -> TokenStream {
    let Template { value } = parse_macro_input!(input as Template);

    let inner_value = value.value();

    let tokens = dsl_to_half_tokens(&inner_value);

    TokenStream::from(tokens)
}

#[cfg(feature = "proc_macro")]
use proc_macro2::TokenStream;
#[cfg(feature = "proc_macro")]
use quote::{format_ident, quote, ToTokens};

#[derive(Clone, Debug)]
pub enum Output<'a> {
    String(&'a str),
    NewLine,
    Indent,
    Dedent,
}

impl Output<'static> {
    pub const fn str(s: &'static str) -> Output {
        Output::String(s)
    }
}

#[derive(Clone, Debug)]
pub enum TemplateToken<'a> {
    InVar(usize),
    OutVar(usize),
    String(Output<'a>),
    LocalVar(usize),
    #[allow(unused)]
    PreviousOuput,
}

impl TemplateToken<'static> {
    pub const fn str(s: &'static str) -> Self {
        TemplateToken::String(Output::str(s))
    }
}

#[cfg(feature = "proc_macro")]
impl<'a> ToTokens for Output<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = match self {
            Output::Dedent => "Dedent",
            Output::Indent => "Indent",
            Output::NewLine => "NewLine",
            Output::String(s) => {
                tokens.extend(quote!(
                    template_types::Output::<'static>::String( #s)
                ));
                return;
            }
        };
        let name_ident = format_ident!("{}", name);

        tokens.extend(quote! {
            template_types::Output::<'static>:: #name_ident
        })
    }
}

#[cfg(feature = "proc_macro")]
impl<'a> ToTokens for TemplateToken<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (name, value) = match self {
            TemplateToken::InVar(a) => ("InVar", quote! {(#a)}),
            TemplateToken::OutVar(a) => ("OutVar", quote! {(#a)}),
            TemplateToken::String(a) => ("String", quote! {(#a)}),
            TemplateToken::LocalVar(a) => ("LocalVar", quote! {(#a)}),
            TemplateToken::PreviousOuput => ("PreviousOutput", quote!()),
        };

        let name_ident = format_ident!("{name}");

        tokens.extend(quote! {
            template_types::TemplateToken::#name_ident #value
        })
    }
}

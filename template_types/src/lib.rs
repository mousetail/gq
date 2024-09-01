#[cfg(feature = "proc_macro")]
use proc_macro2::TokenStream;
#[cfg(feature = "proc_macro")]
use quote::{format_ident, quote, ToTokens};

#[derive(Clone, Debug, Copy)]
pub enum Output<'a> {
    String(&'a str),
    NewLine,
    Indent,
    Dedent,
}

impl<'a> Output<'a> {
    pub const fn str(s: &'a str) -> Output {
        Output::String(s)
    }
}

#[derive(Clone, Debug, Copy)]
pub enum TemplateToken<'a> {
    InVar(usize),
    OutVar(usize),
    String(Output<'a>),
    LocalVar(&'a str),
}

impl<'a> TemplateToken<'a> {
    pub const fn str(s: &'a str) -> Self {
        TemplateToken::String(Output::str(s))
    }

    pub fn get_local_var_names(&self) -> Option<&'a str> {
        match self {
            TemplateToken::LocalVar(k) => Some(k),
            _ => None,
        }
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
        };

        let name_ident = format_ident!("{name}");

        tokens.extend(quote! {
            template_types::TemplateToken::#name_ident #value
        })
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct ProgramFragment<'a> {
    pub init_tokens: &'a [TemplateToken<'a>],
    pub destruct_tokens: &'a [TemplateToken<'a>],
    pub arguments_popped: usize,
    pub arguments_pushed: usize,
}

impl<'a> ProgramFragment<'a> {
    pub fn get_local_var_names(&self) -> impl Iterator<Item = &'a str> {
        self.init_tokens
            .get_local_var_names()
            .chain(self.destruct_tokens.get_local_var_names())
    }
}

#[cfg(feature = "proc_macro")]
impl<'a> ToTokens for ProgramFragment<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ProgramFragment {
            init_tokens,
            destruct_tokens,
            arguments_popped,
            arguments_pushed,
        } = self;

        tokens.extend(quote! {
            template_types::ProgramFragment::<'static> {
                init_tokens: &[
                    #(#init_tokens),*
                ],
                destruct_tokens: &[
                    #(#destruct_tokens),*
                ],
                arguments_pushed: #arguments_pushed,
                arguments_popped: #arguments_popped
            }
        })
    }
}

pub trait HighestVarNumbers<'a> {
    fn get_number_of_input_vars(&self) -> usize;
    #[allow(unused)]
    fn get_number_of_output_vars(&self) -> usize;
    fn get_local_var_names(&self) -> impl Iterator<Item = &'a str>;
}

impl<'a> HighestVarNumbers<'a> for [TemplateToken<'a>] {
    fn get_number_of_input_vars(&self) -> usize {
        self.into_iter()
            .flat_map(|k| match k {
                TemplateToken::InVar(n) => Some(n),
                _ => None,
            })
            .max()
            .map(|d| d + 1)
            .unwrap_or(0)
    }

    fn get_number_of_output_vars(&self) -> usize {
        self.into_iter()
            .flat_map(|k| match k {
                TemplateToken::OutVar(n) => Some(n),
                _ => None,
            })
            .max()
            .map(|d| d + 1)
            .unwrap_or(0)
    }

    fn get_local_var_names(&self) -> impl Iterator<Item = &'a str> {
        self.into_iter().flat_map(|d| d.get_local_var_names())
    }
}

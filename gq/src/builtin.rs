use crate::{fragment::ProgramFragment, output_writer::Output};

#[derive(Clone, Debug)]
pub enum TemplateToken {
    InVar(usize),
    OutVar(usize),
    String(Output<'static>),
    LocalVar(usize),
    #[allow(unused)]
    PreviousOuput,
}

impl TemplateToken {
    pub const fn str(value: &'static str) -> TemplateToken {
        TemplateToken::String(Output::str(value))
    }
}

pub struct BracketHandler {
    pub fragment: ProgramFragment,
    pub output_handler: Option<&'static [TemplateToken]>,
}

#[derive(Default)]
pub struct Builtin {
    pub local_vars: usize,
    pub token: char,
    pub template: ProgramFragment,
    pub brachet_handlers: &'static [BracketHandler],
}

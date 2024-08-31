use template_types::TemplateToken;

use crate::fragment::ProgramFragment;

#[derive(Debug)]
pub struct BracketHandler {
    pub fragment: ProgramFragment,
    pub output_handler: Option<&'static [TemplateToken<'static>]>,
}

#[derive(Default, Debug)]
pub struct Builtin {
    pub local_vars: usize,
    pub token: char,
    pub template: ProgramFragment,
    pub brachet_handlers: &'static [BracketHandler],
}

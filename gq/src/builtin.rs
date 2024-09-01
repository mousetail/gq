use template_types::{HighestVarNumbers, ProgramFragment, TemplateToken};

#[derive(Debug)]
pub struct BracketHandler {
    pub fragment: ProgramFragment<'static>,
    pub output_handler: Option<&'static [TemplateToken<'static>]>,
}

impl BracketHandler {
    fn get_local_var_names(&self) -> impl Iterator<Item = &'static str> {
        return self.fragment.get_local_var_names().chain(
            self.output_handler
                .into_iter()
                .flat_map(|k| k.get_local_var_names()),
        );
    }
}

#[derive(Default, Debug)]
pub struct Builtin {
    pub token: char,
    pub template: ProgramFragment<'static>,
    pub bracket_handlers: &'static [BracketHandler],
}

impl Builtin {
    pub fn get_local_var_names(&self) -> impl Iterator<Item = &'static str> {
        self.template.get_local_var_names().chain(
            self.bracket_handlers
                .iter()
                .flat_map(|handler| handler.get_local_var_names()),
        )
    }
}

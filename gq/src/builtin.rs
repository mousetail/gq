use template_types::{ProgramFragment, TemplateToken};

#[derive(Debug)]
pub struct BracketHandler {
    pub fragment: ProgramFragment<'static>,
    pub output_handler: Option<&'static [TemplateToken<'static>]>,
}

#[derive(Default, Debug)]
pub struct Builtin {
    pub token: char,
    pub template: ProgramFragment<'static>,
    pub brachet_handlers: &'static [BracketHandler],
}

impl Builtin {
    pub fn get_local_var_names(&self) -> impl Iterator<Item = &'static str> {
        self.template
            .get_local_var_names()
            .chain(self.brachet_handlers.iter().flat_map(|handler| {
                handler.fragment.get_local_var_names().chain(
                    handler
                        .output_handler
                        .iter()
                        .flat_map(|z| z.iter().flat_map(|m| m.get_local_var_names())),
                )
            }))
    }
}

use template_types::{HighestVarNumbers, ProgramFragment, TemplateToken};

#[derive(Debug, Clone, Copy)]
pub enum MultiOutputBehavior {
    #[allow(unused)]
    OnlyFirst,
    FlattenAll,
    #[allow(unused)]
    Array,
    Variadic,
    HalfZip,
}

#[derive(Debug, Clone, Copy)]
pub struct OutputHandler {
    pub fragment: &'static [TemplateToken<'static>],
    pub behavior: MultiOutputBehavior,
}

#[derive(Debug)]
pub struct BracketHandler {
    pub fragment: ProgramFragment<'static>,
    pub output_handler: Option<OutputHandler>,
    pub flags: BracketContextFlags,
}

impl BracketHandler {
    fn get_local_var_names(&self) -> impl Iterator<Item = &'static str> {
        return self.fragment.get_local_var_names().chain(
            self.output_handler
                .into_iter()
                .flat_map(|k| k.fragment.get_local_var_names()),
        );
    }
}

#[derive(Default, Debug)]
pub struct Builtin {
    pub name: &'static str,
    #[allow(unused)]
    pub description: &'static str,
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

#[derive(Copy, Clone, Debug)]
pub struct BracketContextFlags {
    pub no_pop: bool,
}

impl BracketContextFlags {
    pub const fn new() -> BracketContextFlags {
        BracketContextFlags { no_pop: false }
    }

    pub const fn set_no_pop(mut self, no_pop: bool) -> Self {
        self.no_pop = no_pop;
        return self;
    }
}

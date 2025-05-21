use std::ops::Deref;

use template_types::{HighestVarNumbers, ProgramFragment, TemplateToken};

use super::lexer::{LexerValue, StackMovement};

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
    fn get_local_var_names(&self) -> impl Iterator<Item = &'static str> + use<> {
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
    pub fn get_local_var_names(&self) -> impl Iterator<Item = &'static str> + use<> {
        self.template.get_local_var_names().chain(
            self.bracket_handlers
                .iter()
                .flat_map(|handler| handler.get_local_var_names()),
        )
    }

    pub fn uses_all_ins(&self) -> bool {
        self.template.uses_all_ins()
            || self
                .bracket_handlers
                .iter()
                .any(|e| e.fragment.uses_all_ins())
    }

    pub fn get_bracket_largest_final_stack_size<'a>(
        &self,
        next: &mut impl Iterator<Item = impl Deref<Target = LexerValue>>,
    ) -> usize {
        let mut max = 0;
        for _bracket_handler in self.bracket_handlers {
            let value = next.next().unwrap();
            let StackMovement { pops, .. } = value.get_stack_movement(next);

            max = max.max(pops);
        }

        max
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

use std::{cell::Cell, collections::HashMap, iter::once};

use template_types::{Output, ProgramFragment, TemplateToken};

use crate::language::{
    builtin::{MultiOutputBehavior, OutputHandler},
    fragment::Destructor,
    varnames::VarNames,
};

use super::builtin::BracketContextFlags;

#[derive(Clone, Debug)]
pub struct OutputHandlerInstance {
    pub inner: OutputHandler,
    pub max_variadic_outputs: Cell<Option<usize>>,
    pub child_variadic_outputs: Cell<Option<usize>>,
}

#[derive(Clone, Debug)]
pub struct StackBracketGroup {
    pub brackent_end_fragment: ProgramFragment<'static>,
    pub local_variables: HashMap<String, String>,
    pub output_handler: Option<OutputHandlerInstance>,

    pub destructors: Vec<Destructor>,
    pub stack: Vec<String>,
    pub no_pop_index: Option<usize>,
}

impl StackBracketGroup {
    pub fn get_output_handler_context(
        &self,
    ) -> Option<(&OutputHandlerInstance, &HashMap<String, String>)> {
        self.output_handler
            .as_ref()
            .map(|output_handler| (output_handler, &self.local_variables))
    }
}

#[derive(Clone, Debug)]
pub struct Stack {
    frames: Vec<StackBracketGroup>,
    pub current_group: StackBracketGroup,
    var_names: VarNames,
    inputs_used: usize,
}

const DEFAULT_OUTPUT_HANDLER: OutputHandler = OutputHandler {
    fragment: &[
        TemplateToken::str("output("),
        TemplateToken::InVar(0),
        TemplateToken::str("?? null);"),
        TemplateToken::String(Output::NewLine),
    ],
    behavior: MultiOutputBehavior::FlattenAll,
};

impl Stack {
    pub fn new() -> Stack {
        Stack {
            frames: vec![],
            var_names: VarNames::default(),
            current_group: StackBracketGroup {
                brackent_end_fragment: ProgramFragment::default(),
                local_variables: HashMap::new(),
                output_handler: Some(OutputHandlerInstance {
                    inner: DEFAULT_OUTPUT_HANDLER,
                    max_variadic_outputs: Cell::new(None),
                    child_variadic_outputs: Cell::new(None),
                }),
                destructors: vec![],
                stack: vec![],
                no_pop_index: None,
            },
            inputs_used: 0,
        }
    }

    pub fn push(&mut self) -> String {
        let name = self.var_names.next().unwrap();
        self.current_group.stack.push(name.clone());

        return name;
    }

    fn pop_after_no_pop_index<'a>(
        no_pop_index: &mut usize,
        inputs_used: usize,
        frames: impl Iterator<Item = &'a mut StackBracketGroup>,
    ) -> String {
        let mut frames_left = *no_pop_index;
        *no_pop_index += 1;

        for frame in frames {
            if frame.stack.len() > frames_left {
                return frame.stack[frame.stack.len() - 1 - frames_left].to_owned();
            }
            frames_left -= frame.stack.len();
        }

        let value = format!("args[{}]", frames_left + inputs_used);
        return value;
    }

    pub fn pop(&mut self) -> String {
        if let Some(value) = self.current_group.stack.pop() {
            return value;
        }

        let mut iter = self.frames.iter_mut().rev();
        if let Some(k) = &mut self.current_group.no_pop_index {
            return Stack::pop_after_no_pop_index(k, self.inputs_used, iter);
        }

        while let Some(item) = iter.next() {
            if let Some(value) = item.stack.pop() {
                return value;
            }

            if let Some(k) = &mut item.no_pop_index {
                return Stack::pop_after_no_pop_index(k, self.inputs_used, iter);
            }
        }

        let value = format!("args[{}]", self.inputs_used);
        self.inputs_used += 1;
        return value;
    }

    pub fn push_group(
        &mut self,
        local_vars: HashMap<String, String>,
        end_fragment: ProgramFragment<'static>,
        output_handler: Option<OutputHandler>,
        flags: BracketContextFlags,
    ) {
        self.frames.push(std::mem::replace(
            &mut self.current_group,
            StackBracketGroup {
                brackent_end_fragment: end_fragment,
                local_variables: local_vars,
                destructors: vec![],
                stack: vec![],
                output_handler: output_handler.map(|output_handler| OutputHandlerInstance {
                    inner: output_handler,
                    max_variadic_outputs: Cell::new(None),
                    child_variadic_outputs: Cell::new(None),
                }),
                no_pop_index: flags.no_pop.then(|| 0),
            },
        ));
    }

    pub fn has_group(&mut self) -> bool {
        self.frames.len() > 0
    }

    pub fn pop_group(&mut self) -> StackBracketGroup {
        std::mem::replace(&mut self.current_group, self.frames.pop().unwrap())
    }

    pub fn local_var_name(&mut self) -> String {
        self.var_names.next().unwrap()
    }

    pub fn get_output_handler(&self) -> (&OutputHandlerInstance, &HashMap<String, String>) {
        let handler = once(&self.current_group)
            .chain(self.frames.iter().rev())
            .flat_map(StackBracketGroup::get_output_handler_context)
            .next()
            .unwrap();

        return handler;
    }

    pub fn destruct_unused_vars(&mut self) -> Vec<Destructor> {
        let mut number_of_orphaned_stack_frames = 0;

        for item in self.current_group.destructors.iter().rev() {
            if item
                .out_vars
                .iter()
                .any(|out_var| self.current_group.stack.contains(out_var))
            {
                break;
            }
            number_of_orphaned_stack_frames += 1;
        }

        let split_off_values = self
            .current_group
            .destructors
            .split_off(self.current_group.destructors.len() - number_of_orphaned_stack_frames);

        split_off_values
    }
}

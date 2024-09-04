use std::{collections::HashMap, iter::once};

use template_types::{Output, ProgramFragment, TemplateToken};

use crate::language::{
    builtin::{MultiOutputBehavior, OutputHandler},
    fragment::Destructor,
    varnames::VarNames,
};

#[derive(Clone, Debug)]
pub struct StackBracketGroup {
    pub brackent_end_fragment: ProgramFragment<'static>,
    pub local_variables: HashMap<String, String>,
    pub output_handler: Option<OutputHandler>,

    pub destructors: Vec<Destructor>,
    pub stack: Vec<String>,
}

impl StackBracketGroup {
    pub fn get_output_handler_context(&self) -> Option<(&OutputHandler, &HashMap<String, String>)> {
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
        TemplateToken::str(");"),
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
                output_handler: Some(DEFAULT_OUTPUT_HANDLER),
                destructors: vec![],
                stack: vec![],
            },
            inputs_used: 0,
        }
    }

    pub fn push(&mut self) -> String {
        let name = self.var_names.next().unwrap();
        self.current_group.stack.push(name.clone());

        return name;
    }

    pub fn pop(&mut self) -> String {
        if let Some(value) = self.current_group.stack.pop() {
            return value;
        }

        for item in self.frames.iter_mut().rev() {
            if let Some(value) = item.stack.pop() {
                return value;
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
    ) {
        self.frames.push(std::mem::replace(
            &mut self.current_group,
            StackBracketGroup {
                brackent_end_fragment: end_fragment,
                local_variables: local_vars,
                destructors: vec![],
                stack: vec![],
                output_handler,
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

    pub fn get_output_handler(&self) -> (&OutputHandler, &HashMap<String, String>) {
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

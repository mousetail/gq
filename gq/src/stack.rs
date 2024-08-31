use std::iter::once;

use crate::{
    builtin::TemplateToken,
    fragment::{Destructor, ProgramFragment},
    output_writer::Output,
    varnames::VarNames,
};

#[derive(Clone, Debug)]
pub struct StackBracketGroup {
    pub brackent_end_fragment: ProgramFragment,
    pub local_variables: Vec<String>,
    pub output_handler: Option<&'static [TemplateToken]>,

    pub destructors: Vec<Destructor>,
    pub stack: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Stack {
    frames: Vec<StackBracketGroup>,
    pub current_group: StackBracketGroup,
    var_names: VarNames,
}

const DEFAULT_OUTPUT_HANDLER: &'static [TemplateToken] = &[
    TemplateToken::str("console.log("),
    TemplateToken::InVar(0),
    TemplateToken::str(");"),
    TemplateToken::String(Output::NewLine),
];

impl Stack {
    pub fn new() -> Stack {
        Stack {
            frames: vec![],
            var_names: VarNames::default(),
            current_group: StackBracketGroup {
                brackent_end_fragment: ProgramFragment::default(),
                local_variables: vec![],
                output_handler: Some(DEFAULT_OUTPUT_HANDLER),
                destructors: vec![],
                stack: vec![],
            },
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

        panic!("Attempt to pop from empty stack")
    }

    pub fn push_group(
        &mut self,
        local_vars: Vec<String>,
        end_fragment: ProgramFragment,
        output_handler: Option<&'static [TemplateToken]>,
    ) {
        print!("// pushed {}", self.frames.len());

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

    pub fn pop_group(&mut self) -> StackBracketGroup {
        print!("// popped {}", self.frames.len());
        std::mem::replace(&mut self.current_group, self.frames.pop().unwrap())
    }

    pub fn local_var_name(&mut self) -> String {
        self.var_names.next().unwrap()
    }

    pub fn output_handlers(&self) -> impl Iterator<Item = Option<&'static [TemplateToken]>> + '_ {
        once(self.current_group.output_handler.clone())
            .chain((&self.frames).into_iter().map(|k| k.output_handler))
    }
}

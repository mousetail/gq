use std::{collections::HashMap, io::Write, iter::Peekable};

use builtins::BUILTINS;
use fragment::{dispose_bracket_handler, write_comma};
use stack::Stack;

mod builtin;
mod builtins;
mod fragment;
mod output_writer;
mod stack;
mod varnames;
use output_writer::OutputWriter;

pub fn transpile_program(
    iter: &mut impl Iterator<Item = char>,
    output: &mut impl Write,
) -> std::io::Result<()> {
    let mut iter = iter.peekable();
    let mut output = OutputWriter::new(output);
    output.write(template_types::Output::String("((output)=>{"))?;

    let mut stack = Stack::new();

    while let Some(char) = iter.next() {
        if char == ']' {
            let bracket_handler = stack.pop_group();

            fragment::dispose_bracket_handler(&mut output, bracket_handler, &mut stack)?;

            continue;
        }

        if char == ',' {
            write_comma(&mut output, &mut stack)?;
            continue;
        }

        let builtin = BUILTINS.iter().find(|d| d.token == char).unwrap();

        let local_vars: HashMap<_, _> = builtin
            .get_local_var_names()
            .map(|d| (d.to_owned(), stack.local_var_name()))
            .collect();

        fragment::write_fragment(&mut output, builtin.template, &mut stack, &local_vars)?;

        for bracket_handler in builtin.bracket_handlers.iter().rev() {
            stack.push_group(
                local_vars.clone(),
                bracket_handler.fragment,
                bracket_handler.output_handler,
            );
        }
    }

    dispose_bracket_handler(&mut output, stack.current_group.clone(), &mut stack)?;

    output.write(template_types::Output::String("})"))?;

    Ok(())
}

use std::{collections::HashMap, io::Write, iter::Peekable};

use builtins::BUILTINS;
use fragment::dispose_bracket_handler;
use stack::Stack;

mod builtin;
mod builtins;
mod fragment;
mod output_writer;
mod stack;
mod varnames;
use output_writer::OutputWriter;

fn transpile_program(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    output: &mut OutputWriter<impl Write>,
) -> std::io::Result<()> {
    let mut stack = Stack::new();

    while let Some(char) = iter.next() {
        if char == ']' {
            let bracket_handler = stack.pop_group();

            fragment::dispose_bracket_handler(output, bracket_handler, &mut stack)?;

            continue;
        }

        let builtin = BUILTINS.iter().find(|d| d.token == char).unwrap();

        let local_vars: HashMap<_, _> = builtin
            .get_local_var_names()
            .map(|d| (d.to_owned(), stack.local_var_name()))
            .collect();

        fragment::write_fragment(output, builtin.template, &mut stack, &local_vars)?;

        for bracket_handler in builtin.bracket_handlers.iter().rev() {
            stack.push_group(
                local_vars.clone(),
                bracket_handler.fragment,
                bracket_handler.output_handler,
            );
        }
    }

    dispose_bracket_handler(output, stack.current_group.clone(), &mut stack)?;

    Ok(())
}

fn main() {
    //let program = "11+r[,,1+1+r]1]]";
    let program = "111++r,11]";

    let mut chars = program.chars().peekable();

    let mut writer = OutputWriter::new(std::io::stdout());
    println!("(()=>{{");
    transpile_program(&mut chars, &mut writer).unwrap();
    println!();
    println!("}})()");
}

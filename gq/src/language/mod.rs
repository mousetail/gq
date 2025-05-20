use std::{collections::HashMap, fs::OpenOptions, io::Write, iter::Peekable};

use builtin::Builtin;
use builtins::BUILTINS;
use fragment::{dispose_bracket_handler, write_comma};
use lexer::{LexerValue, parse};
use stack::{MockStack, Stack};

mod builtin;
mod builtins;
mod fragment;
mod lexer;
mod output_writer;
mod stack;
mod varnames;

use output_writer::OutputWriter;

fn parse_single_command(
    iter: &mut Peekable<impl Iterator<Item = LexerValue> + Clone>,
    stack: &mut Stack,
    output: &mut OutputWriter<impl Write>,
) -> std::io::Result<()> {
    match iter.next().unwrap() {
        LexerValue::Builtin(builtin) => {
            let local_vars: HashMap<_, _> = builtin
                .get_local_var_names()
                .map(|d| (d.to_owned(), stack.local_var_name()))
                .collect();

            output.write(template_types::Output::String(&format!(
                "// {}",
                builtin.name
            )))?;

            let all_outs = if !builtin.bracket_handlers.is_empty() && builtin.uses_all_ins() {
                let number_of_inputs_shadowed = builtin
                    .get_bracket_largest_final_stack_size(&mut iter.clone().map(|e| Box::new(e)));

                let top_n_values = stack.get_top_n_values(number_of_inputs_shadowed);
                Some(top_n_values)
            } else {
                None
            };

            output.write(template_types::Output::NewLine)?;
            fragment::write_fragment(
                output,
                builtin.template,
                stack,
                &local_vars,
                all_outs.as_ref(),
            )?;

            for bracket_handler in builtin.bracket_handlers.iter().rev() {
                stack.push_group(
                    local_vars.clone(),
                    bracket_handler.fragment,
                    bracket_handler.output_handler,
                    bracket_handler.flags,
                );
            }
            for _ in builtin.bracket_handlers.iter() {
                parse_single_command(iter, stack, output)?;
                fragment::dispose_bracket_handler(output, stack.pop_group(), stack)?;
            }

            Ok(())
        }
        LexerValue::Comma => {
            write_comma(output, stack)?;
            Ok(())
        }
        LexerValue::Group(items) => {
            let mut iterator = items.into_iter().peekable();

            while let Some(_) = iterator.peek() {
                parse_single_command(&mut iterator, stack, output)?;
            }

            Ok(())
        }
        LexerValue::Literal(literal) => Ok(stack.current_group.stack.push(match literal {
            lexer::Literal::Integer(e) => format!("{}", e),
            lexer::Literal::String(k) => serde_json::to_string(&k).unwrap(),
        })),
    }
}

pub fn transpile_program(
    iter: &mut impl Iterator<Item = char>,
    output: &mut impl Write,
) -> std::io::Result<()> {
    let parsed_code = parse(&mut iter.peekable());
    let mut output = OutputWriter::new(output);
    output.write(template_types::Output::String("((output, ...args)=>{"))?;
    output.write(template_types::Output::Indent)?;
    output.write(template_types::Output::NewLine)?;

    let mut stack = Stack::new();

    let mut iter = parsed_code.into_iter().peekable();
    while let Some(_) = iter.peek() {
        parse_single_command(&mut iter, &mut stack, &mut output)?;
    }

    while stack.has_group() {
        dispose_bracket_handler(&mut output, stack.pop_group(), &mut stack)?;
    }
    dispose_bracket_handler(&mut output, stack.current_group.clone(), &mut stack)?;

    output.write(template_types::Output::Dedent)?;
    output.write(template_types::Output::NewLine)?;
    output.write(template_types::Output::String("})"))?;

    Ok(())
}

pub fn get_builtin_tokens() {
    let tokens = BUILTINS
        .iter()
        .map(|k| (k.token, k.name))
        .chain([(',', "comma"), (']', "end group"), ('"', "Start String")])
        .collect::<Vec<_>>();

    let mut first_column: Vec<_> = ('a'..='z').map(|k| k.to_string()).collect();
    let mut second_column: Vec<_> = ('A'..='Z').map(|k| k.to_string()).collect();
    let mut third_column = ('!'..='/')
        .chain(':'..='@')
        .chain('['..='`')
        .chain('{'..='~')
        .map(|k| k.to_string())
        .collect();

    for token in tokens {
        let list = match token.0 {
            'a'..='z' => &mut first_column,
            'A'..='Z' => &mut second_column,
            _ => &mut third_column,
        };

        let index = list
            .iter()
            .position(|d| d.chars().next() == Some(token.0))
            .unwrap();
        list[index] = format!("{} {}", token.0, token.1);
    }

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("available_letters.md")
        .unwrap();

    writeln!(file, "| Upper Case | Lower Case | Symbols |").unwrap();
    writeln!(file, "| ---------- | ---------- | ------- |").unwrap();

    for i in 0..(first_column
        .len()
        .max(second_column.len().max(third_column.len())))
    {
        writeln!(
            file,
            "| {} | {} | \\{} |",
            first_column.get(i).map(String::as_str).unwrap_or(""),
            second_column.get(i).map(String::as_str).unwrap_or(""),
            third_column.get(i).map(String::as_str).unwrap_or(""),
        )
        .unwrap();
    }
}

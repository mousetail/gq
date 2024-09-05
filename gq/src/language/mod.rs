use std::{collections::HashMap, fs::OpenOptions, io::Write};

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
    output.write(template_types::Output::String("((output, ...args)=>{"))?;
    output.write(template_types::Output::Indent)?;
    output.write(template_types::Output::NewLine)?;

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

        if char.is_whitespace() {
            continue;
        }

        if char == '"' {
            let mut value = String::new();
            while let Some(char) = iter.next() {
                if char == '"' {
                    break;
                }
                if char == '\\' {
                    value.extend(iter.next())
                } else {
                    value.push(char);
                }
            }
            stack
                .current_group
                .stack
                .push(serde_json::to_string(&value).unwrap());
            continue;
        }

        if let Some(mut digit_value) = char.to_digit(10) {
            while let Some(next_digit_value) = iter.peek().and_then(|k| k.to_digit(10)) {
                digit_value = digit_value * 10 + next_digit_value;
                iter.next();
            }
            stack.current_group.stack.push(format!("{digit_value}"));
            continue;
        }

        let builtin = BUILTINS
            .iter()
            .find(|d| d.token == char)
            .unwrap_or_else(|| panic!("Unexpected token {char}"));

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
                bracket_handler.flags,
            );
        }
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

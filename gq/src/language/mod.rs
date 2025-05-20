use std::{collections::HashMap, fs::OpenOptions, io::Write, iter::Peekable};

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

fn should_start_group(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    previous_was_pipe: bool,
) -> bool {
    if previous_was_pipe {
        return true;
    } else {
        let next_char = *iter.peek().expect("Modifier expected a function to modify");

        if next_char == '(' {
            iter.next().unwrap();
            return true;
        } else {
            return false;
        }
    }
}

fn parse_literal(iter: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    match iter.peek().copied() {
        Some('"') => {
            let _ = iter.next();
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

            Some(serde_json::to_string(&value).unwrap())
        }
        Some(e) if e.is_digit(10) => {
            let _ = iter.next();
            let mut digit_value = e.to_digit(10).unwrap();
            while let Some(next_digit_value) = iter.peek().and_then(|k| k.to_digit(10)) {
                digit_value = digit_value * 10 + next_digit_value;
                iter.next();
            }
            return Some(format!("{digit_value}"));
        }
        _ => None,
    }
}

fn parse_single_command(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    stack: &mut Stack,
    output: &mut OutputWriter<impl Write>,
    previous_was_pipe: bool,
) -> std::io::Result<bool> {
    while iter.peek().is_some_and(|e| e.is_whitespace()) {
        let _ = iter.next();
    }

    if should_start_group(iter, previous_was_pipe) {
        while match iter.peek() {
            Some('|') => {
                let _ = iter.next();
                return Ok(true);
            }
            Some(')') => {
                let _ = iter.next();
                return Ok(false);
            }
            Some(_) => true,
            None => panic!("Expected closing parenthesis"),
        } {
            let last_char_is_pipe = parse_single_command(iter, stack, output, false)?;
            if last_char_is_pipe {
                panic!("Use of \"|\" pipe char in unmodified context")
            }
        }
        Ok(false)
    } else if let Some(m) = parse_literal(iter) {
        stack.current_group.stack.push(m);
        Ok(false)
    } else if iter.peek().copied() == Some(',') {
        let _ = iter.next();

        write_comma(output, stack)?;
        Ok(false)
    } else if let Some(first_char) = iter.next() {
        let builtin = BUILTINS
            .iter()
            .find(|d| d.token == first_char)
            .unwrap_or_else(|| panic!("Unexpected token {first_char}"));

        let local_vars: HashMap<_, _> = builtin
            .get_local_var_names()
            .map(|d| (d.to_owned(), stack.local_var_name()))
            .collect();

        output.write(template_types::Output::String(&format!(
            "// {}",
            builtin.name
        )))?;
        output.write(template_types::Output::NewLine)?;
        fragment::write_fragment(output, builtin.template, stack, &local_vars)?;

        for bracket_handler in builtin.bracket_handlers.iter().rev() {
            stack.push_group(
                local_vars.clone(),
                bracket_handler.fragment,
                bracket_handler.output_handler,
                bracket_handler.flags,
            );
        }
        let mut last_char_is_pipe = false;
        for _ in builtin.bracket_handlers.iter() {
            last_char_is_pipe = parse_single_command(iter, stack, output, last_char_is_pipe)?;
            fragment::dispose_bracket_handler(output, stack.pop_group(), stack)?;
        }

        Ok(false)
    } else {
        panic!("End of file reached after modifier")
    }
}

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

    while let Some(_) = iter.peek() {
        parse_single_command(&mut iter, &mut stack, &mut output, false)?;
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

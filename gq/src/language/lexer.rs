use crate::language::{Builtin, builtins::BUILTINS};
use std::{iter::Peekable, ops::Deref};

#[derive(Clone)]
pub enum Literal {
    Integer(u32),
    String(String),
}

#[derive(Clone)]
pub enum LexerValue {
    Builtin(&'static Builtin),
    Comma,
    Group(Vec<LexerValue>),
    Literal(Literal),
}

impl LexerValue {
    pub fn get_stack_movement<'a>(
        &self,
        next: &mut impl Iterator<Item = impl Deref<Target = LexerValue>>,
    ) -> (usize, usize) {
        match self {
            LexerValue::Builtin(builtin) => {
                let mut number_popped = builtin.template.arguments_popped;
                let mut number_pushed = builtin.template.arguments_pushed;

                for bracket_handler in builtin.bracket_handlers {
                    let (group_addition, group_subtraction) =
                        next.next().unwrap().get_stack_movement(next);
                    match bracket_handler.output_handler.or_else(|| {
                        builtin
                            .bracket_handlers
                            .iter()
                            .last()
                            .and_then(|k| k.output_handler)
                    }) {
                        None if bracket_handler.flags.no_pop => (),
                        None => number_popped += group_subtraction,
                        Some(l) => match l.behavior {
                            super::builtin::MultiOutputBehavior::Variadic => {
                                number_popped += group_subtraction;
                                number_pushed += group_addition
                            }
                            _ => number_popped += group_subtraction,
                        },
                    };

                    number_popped -= bracket_handler.fragment.arguments_popped;
                    number_popped += bracket_handler.fragment.arguments_pushed;
                }

                (number_popped, number_pushed)
            }
            LexerValue::Comma => (1, 0),
            LexerValue::Group(lexer_values) => {
                let mut iterator = lexer_values.iter();

                let mut number_popped @ mut num_pushed = 0;

                while let Some(value) = iterator.next() {
                    let (subtraction, addition) = value.get_stack_movement(&mut iterator);

                    let original_num_pushed = num_pushed;
                    num_pushed -= subtraction.min(original_num_pushed);
                    number_popped += subtraction.saturating_sub(original_num_pushed);

                    num_pushed += addition;
                }

                (number_popped, num_pushed)
            }
            LexerValue::Literal(_literal) => (0, 1),
        }
    }
}

fn parse_literal(iter: &mut Peekable<impl Iterator<Item = char>>) -> Option<Literal> {
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

            Some(Literal::String(value))
        }
        Some(e) if e.is_digit(10) => {
            let _ = iter.next();
            let mut digit_value = e.to_digit(10).unwrap();
            while let Some(next_digit_value) = iter.peek().and_then(|k| k.to_digit(10)) {
                digit_value = digit_value * 10 + next_digit_value;
                iter.next();
            }
            return Some(Literal::Integer(digit_value));
        }
        _ => None,
    }
}

pub fn parse(code: &mut Peekable<impl Iterator<Item = char>>) -> Vec<LexerValue> {
    let mut last = vec![];
    let mut stack: Vec<Vec<LexerValue>> = vec![];
    loop {
        if let Some(literal) = parse_literal(code) {
            last.push(LexerValue::Literal(literal))
        };

        match code.next() {
            None => break,
            Some(d) if d.is_whitespace() => (),
            Some(',') => last.push(LexerValue::Comma),
            Some('(') => {
                stack.push(last);
                last = vec![];
            }
            Some('|') => {
                let old_last = last;
                last = stack.pop().expect("Empty stack");
                last.push(LexerValue::Group(old_last));
                stack.push(last);
                last = vec![];
            }
            Some(')') => {
                let old_last = last;
                last = stack.pop().expect("Empty stack");
                last.push(LexerValue::Group(old_last));
            }
            Some(builtin_token) => {
                let builtin = BUILTINS
                    .iter()
                    .find(|d| d.token == builtin_token)
                    .unwrap_or_else(|| panic!("Unexpected token {builtin_token}"));

                last.push(LexerValue::Builtin(builtin))
            }
        }
    }

    assert!(stack.len() == 0);
    return last;
}

use crate::language::{Builtin, builtins::BUILTINS};
use std::{fmt::Debug, iter::Peekable, ops::Deref};

#[derive(Clone, Copy, Debug)]
pub struct StackMovement {
    pub pops: usize,
    pub pushes: usize,
}

#[derive(Clone, Debug)]
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

impl Debug for LexerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Builtin(arg0) => f.debug_tuple("Builtin").field(&arg0.name).finish(),
            Self::Comma => write!(f, "Comma"),
            Self::Group(arg0) => f.debug_tuple("Group").field(arg0).finish(),
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
        }
    }
}

impl LexerValue {
    pub fn get_stack_movement<'a>(
        &self,
        next: &mut impl Iterator<Item = impl Deref<Target = LexerValue>>,
    ) -> StackMovement {
        match self {
            LexerValue::Builtin(builtin) => {
                let mut stack_movement = StackMovement {
                    pops: builtin.template.arguments_popped,
                    pushes: builtin.template.arguments_pushed,
                };

                for bracket_handler in builtin.bracket_handlers {
                    println!(
                        "{} handler {} popped={stack_movement:?} {} {}",
                        builtin.name,
                        builtin.bracket_handlers.len(),
                        bracket_handler.fragment.arguments_popped,
                        bracket_handler.fragment.arguments_pushed
                    );

                    let next_builtin = next
                        .next()
                        .expect(&format!("End of stack reached with {}", builtin.name));
                    println!("{:?}", next_builtin.deref());
                    let next_stack_movement = next_builtin.get_stack_movement(next);
                    match bracket_handler.output_handler.or_else(|| {
                        builtin
                            .bracket_handlers
                            .iter()
                            .last()
                            .and_then(|k| k.output_handler)
                    }) {
                        None if bracket_handler.flags.no_pop => (),
                        None => stack_movement.pops += next_stack_movement.pops,
                        Some(l) => match l.behavior {
                            super::builtin::MultiOutputBehavior::Variadic
                                if bracket_handler.flags.no_pop =>
                            {
                                stack_movement.pushes += next_stack_movement.pushes - 1
                            }
                            super::builtin::MultiOutputBehavior::Variadic => {
                                stack_movement.pops += next_stack_movement.pops;
                                stack_movement.pushes += next_stack_movement.pushes - 1
                            }
                            _ => stack_movement.pops += next_stack_movement.pops,
                        },
                    };

                    stack_movement.pops -= bracket_handler.fragment.arguments_popped;
                    stack_movement.pushes += bracket_handler.fragment.arguments_pushed;
                }

                stack_movement
            }
            LexerValue::Comma => StackMovement { pops: 1, pushes: 0 },
            LexerValue::Group(lexer_values) => {
                let mut iterator = lexer_values.iter();

                let mut stack_movement = StackMovement { pops: 0, pushes: 0 };

                while let Some(value) = iterator.next() {
                    let next_value_movement = value.get_stack_movement(&mut iterator);

                    let original_num_pushed = stack_movement.pushes;
                    stack_movement.pushes -= next_value_movement.pops.min(stack_movement.pushes);
                    stack_movement.pops +=
                        next_value_movement.pops.saturating_sub(original_num_pushed);

                    stack_movement.pushes += next_value_movement.pushes;
                }

                stack_movement
            }
            LexerValue::Literal(_literal) => StackMovement { pops: 0, pushes: 1 },
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

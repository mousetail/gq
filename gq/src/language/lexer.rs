use crate::language::{Builtin, builtins::BUILTINS};
use std::iter::Peekable;

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

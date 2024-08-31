use std::iter::Peekable;

use template_types::{Output, TemplateToken};

fn index_or_new(items: &mut Vec<String>, name: &str) -> usize {
    if let Some(index) = items.iter().position(|k| k == name) {
        index
    } else {
        items.push(name.to_string());
        items.len() - 1
    }
}

#[derive(Default)]
struct Variables {
    ins: Vec<String>,
    outs: Vec<String>,
    locals: Vec<String>,
}

impl Variables {
    fn lookup_in(&mut self, name: &str) -> usize {
        index_or_new(&mut self.ins, name)
    }

    fn lookup_out(&mut self, name: &str) -> usize {
        index_or_new(&mut self.outs, name)
    }

    fn lookup_local(&mut self, name: &str) -> usize {
        index_or_new(&mut self.locals, name)
    }
}

pub(crate) fn dsl_to_tokens(tokens: &str) -> [Vec<TemplateToken>; 2] {
    let mut chars = tokens.char_indices().peekable();

    let mut indentation = skip_indents(&mut chars, &mut |_index| ());
    let mut varaibles = Variables::default();

    return [
        half_parse(tokens, &mut chars, &mut indentation, &mut varaibles),
        half_parse(tokens, &mut chars, &mut indentation, &mut varaibles),
    ];
}

fn skip_indents(
    chars: &mut Peekable<impl DoubleEndedIterator<Item = (usize, char)>>,
    skip_to_char: &mut impl FnMut(usize),
) -> usize {
    let mut number_of_spaces = 0;

    while let Some(index) = match chars.peek() {
        Some((index, ' ')) => {
            number_of_spaces += 1;
            Some(*index)
        }
        Some((index, '\n')) => {
            number_of_spaces = 0;
            Some(*index)
        }
        _ => None,
    } {
        chars.next();
        skip_to_char(index);
    }

    return number_of_spaces;
}

fn half_parse<'a>(
    string: &'a str,
    chars: &mut Peekable<impl DoubleEndedIterator<Item = (usize, char)>>,
    indentation: &mut usize,
    variables: &mut Variables,
) -> Vec<TemplateToken<'a>> {
    let Some(mut group_start) = chars.peek().map(|k| k.0) else {
        return vec![];
    };
    let mut tokens = vec![];
    let mut in_substitution = false;

    let mut get_partial_text = |index| {
        let value = &string[group_start..index];
        group_start = index
            + (&string[index..])
                .chars()
                .next()
                .map(|k| k.len_utf8())
                .unwrap_or(0);
        return value;
    };
    let wrap =
        |value: &'a str| (!value.is_empty()).then(|| TemplateToken::String(Output::String(value)));

    while let Some((index, next_char)) = chars.next() {
        if in_substitution {
            if next_char == '}' {
                let text = get_partial_text(index);

                let parts: Vec<_> = text.trim().split(':').map(|k| k.trim()).collect();

                match parts.as_slice() {
                    ["inner"] => return tokens,
                    [name, "in"] => tokens.push(TemplateToken::InVar(variables.lookup_in(&name))),
                    [name, "out"] => {
                        tokens.push(TemplateToken::OutVar(variables.lookup_out(&name)))
                    }
                    [name, "local"] => {
                        tokens.push(TemplateToken::LocalVar(variables.lookup_local(&name)))
                    }
                    _ => panic!("Unexpected variable name"),
                }
                in_substitution = false;
            }
            continue;
        }

        if next_char == '}' {
            if let Some((_, '}')) = chars.next() {
                tokens.extend(wrap(get_partial_text(index)));
            } else {
                panic!("Unexpected }} outside of handlebars block");
            }
        } else if next_char == '{' {
            if let Some((_, '{')) = chars.next() {
                tokens.extend(wrap(get_partial_text(index)));
                continue;
            }

            in_substitution = true;
            tokens.extend(wrap(get_partial_text(index)));
        } else if next_char == '\n' {
            tokens.extend(wrap(get_partial_text(index)));

            let number_of_spaces = skip_indents(chars, &mut |index| {
                get_partial_text(index);
            });
            if number_of_spaces > *indentation {
                tokens.push(TemplateToken::String(Output::Indent))
            } else if number_of_spaces < *indentation {
                tokens.push(TemplateToken::String(Output::Dedent))
            }
            *indentation = number_of_spaces;
        }
    }
    if in_substitution {
        panic!("End of string encountered during substitution");
    }
    tokens.extend(wrap(get_partial_text(string.len())));

    tokens
}

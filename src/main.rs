use std::{collections::HashMap, io::Write};

use fragment::{dispose_bracket_handler, Destructor, ProgramFragment};
use stack::{Stack, StackBracketGroup};
use varnames::VarNames;

mod fragment;
mod stack;
mod varnames;

#[derive(Clone, Copy, Debug)]
enum TemplateToken {
    InVar(usize),
    OutVar(usize),
    String(&'static str),
    LocalVar(usize),
    PreviousOuput,
}

#[derive(Default)]
struct Builtin {
    local_vars: usize,
    token: char,
    template: ProgramFragment,
    brachet_handlers: &'static [BracketHandler],
}

struct BracketHandler {
    fragment: ProgramFragment,
    output_handler: Option<&'static [TemplateToken]>,
}

const BUILTINS: &'static [Builtin] = &[
    Builtin {
        local_vars: 1,
        token: '[',
        template: ProgramFragment {
            init_tokens: &[
                TemplateToken::String("let "),
                TemplateToken::LocalVar(0),
                TemplateToken::String("= [];\n"),
            ],
            destruct_tokens: &[],
            arguments_popped: 0,
            arguments_pushed: 0,
        },
        brachet_handlers: &[BracketHandler {
            output_handler: Some(&[
                TemplateToken::LocalVar(0),
                TemplateToken::String(".push("),
                TemplateToken::InVar(0),
                TemplateToken::String(");\n"),
            ]),
            fragment: ProgramFragment {
                init_tokens: &[
                    TemplateToken::String("const "),
                    TemplateToken::OutVar(0),
                    TemplateToken::String("="),
                    TemplateToken::LocalVar(0),
                    TemplateToken::String(";\n"),
                ],
                destruct_tokens: &[],
                arguments_popped: 0,
                arguments_pushed: 1,
            },
        }],
    },
    Builtin {
        token: 'r',
        template: ProgramFragment {
            init_tokens: &[
                TemplateToken::String("for (let "),
                TemplateToken::OutVar(0),
                TemplateToken::String("=0; "),
                TemplateToken::OutVar(0),
                TemplateToken::String("<("),
                TemplateToken::InVar(0),
                TemplateToken::String("); "),
                TemplateToken::OutVar(0),
                TemplateToken::String("++)\n {"),
            ],
            destruct_tokens: &[TemplateToken::String("}\n")],
            arguments_popped: 1,
            arguments_pushed: 1,
        },
        local_vars: 0,
        brachet_handlers: &[],
    },
    Builtin {
        token: '+',
        template: ProgramFragment {
            init_tokens: &[
                TemplateToken::String("let "),
                TemplateToken::OutVar(0),
                TemplateToken::String(" = ("),
                TemplateToken::InVar(0),
                TemplateToken::String(") + ("),
                TemplateToken::InVar(1),
                TemplateToken::String(");\n"),
            ],
            destruct_tokens: &[],
            arguments_popped: 2,
            arguments_pushed: 1,
        },
        local_vars: 0,
        brachet_handlers: &[],
    },
    Builtin {
        token: '1',
        template: ProgramFragment {
            init_tokens: &[
                TemplateToken::String("const "),
                TemplateToken::OutVar(0),
                TemplateToken::String("= 1;\n"),
            ],
            destruct_tokens: &[],
            arguments_pushed: 1,
            arguments_popped: 0,
        },
        local_vars: 0,
        brachet_handlers: &[],
    },
];

fn transpile_program(
    iter: &mut impl Iterator<Item = char>,
    output: &mut impl Write,
) -> std::io::Result<()> {
    let mut stack = Stack::new();

    while let Some(char) = iter.next() {
        //eprintln!("processing char {char}");

        if char == ']' {
            let bracket_handler = stack.pop_group();

            fragment::dispose_bracket_handler(output, bracket_handler, &mut stack)?;

            continue;
        }

        let builtin = BUILTINS.iter().find(|d| d.token == char).unwrap();

        let local_vars: Vec<_> = (0..builtin.local_vars)
            .map(|_| stack.local_var_name())
            .collect();

        fragment::write_fragment(output, builtin.template, &mut stack, &local_vars)?;

        for bracket_handler in builtin.brachet_handlers {
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
    let program = "11+r[1+1+r]";

    let mut chars = program.chars();
    transpile_program(&mut chars, &mut std::io::stdout()).unwrap();
    println!();
}

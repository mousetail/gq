use std::io::Write;

use fragment::{dispose_bracket_handler, ProgramFragment};
use stack::Stack;

mod fragment;
mod output_writer;
mod stack;
mod varnames;
use output_writer::{Output, OutputWriter};

#[derive(Clone, Debug)]
enum TemplateToken {
    InVar(usize),
    OutVar(usize),
    String(Output<'static>),
    LocalVar(usize),
    #[allow(unused)]
    PreviousOuput,
}

impl TemplateToken {
    const fn str(value: &'static str) -> TemplateToken {
        TemplateToken::String(Output::str(value))
    }
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
        local_vars: 2,
        token: ',',
        template: ProgramFragment {
            init_tokens: &[
                TemplateToken::str("const "),
                TemplateToken::LocalVar(0),
                TemplateToken::str("= ()=>{"),
                TemplateToken::String(Output::NewLine),
                TemplateToken::String(Output::Indent),
            ],
            destruct_tokens: &[],
            arguments_popped: 0,
            arguments_pushed: 0,
        },
        brachet_handlers: &[
            BracketHandler {
                output_handler: Some(&[
                    TemplateToken::LocalVar(1),
                    TemplateToken::str("("),
                    TemplateToken::InVar(0),
                    TemplateToken::str(");"),
                    TemplateToken::String(Output::NewLine),
                ]),
                fragment: ProgramFragment {
                    arguments_popped: 0,
                    arguments_pushed: 0,
                    init_tokens: &[],
                    destruct_tokens: &[],
                },
            },
            BracketHandler {
                output_handler: Some(&[
                    TemplateToken::LocalVar(1),
                    TemplateToken::str("("),
                    TemplateToken::InVar(0),
                    TemplateToken::str(");"),
                    TemplateToken::String(Output::NewLine),
                ]),
                fragment: ProgramFragment {
                    arguments_popped: 0,
                    arguments_pushed: 1,
                    init_tokens: &[
                        TemplateToken::String(Output::Dedent),
                        TemplateToken::str("};"),
                        TemplateToken::String(Output::NewLine),
                        TemplateToken::str("const "),
                        TemplateToken::LocalVar(1),
                        TemplateToken::str(" = ("),
                        TemplateToken::OutVar(0),
                        TemplateToken::str(") => {"),
                        TemplateToken::String(Output::Indent),
                        TemplateToken::String(Output::NewLine),
                    ],
                    destruct_tokens: &[
                        TemplateToken::String(Output::Dedent),
                        TemplateToken::str("};"),
                        TemplateToken::String(Output::NewLine),
                        TemplateToken::LocalVar(0),
                        TemplateToken::str("();"),
                        TemplateToken::String(Output::NewLine),
                    ],
                },
            },
        ],
    },
    Builtin {
        local_vars: 1,
        token: '[',
        template: ProgramFragment {
            init_tokens: &[
                TemplateToken::str("let "),
                TemplateToken::LocalVar(0),
                TemplateToken::str("= [];"),
                TemplateToken::String(Output::NewLine),
            ],
            destruct_tokens: &[],
            arguments_popped: 0,
            arguments_pushed: 0,
        },
        brachet_handlers: &[BracketHandler {
            output_handler: Some(&[
                TemplateToken::LocalVar(0),
                TemplateToken::str(".push("),
                TemplateToken::InVar(0),
                TemplateToken::str(");"),
                TemplateToken::String(Output::NewLine),
            ]),
            fragment: ProgramFragment {
                init_tokens: &[
                    TemplateToken::str("const "),
                    TemplateToken::OutVar(0),
                    TemplateToken::str("="),
                    TemplateToken::LocalVar(0),
                    TemplateToken::str(";"),
                    TemplateToken::String(Output::NewLine),
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
                TemplateToken::str("for (let "),
                TemplateToken::OutVar(0),
                TemplateToken::str("=0; "),
                TemplateToken::OutVar(0),
                TemplateToken::str("<("),
                TemplateToken::InVar(0),
                TemplateToken::str("); "),
                TemplateToken::OutVar(0),
                TemplateToken::str("++) {"),
                TemplateToken::String(Output::Indent),
                TemplateToken::String(Output::NewLine),
            ],
            destruct_tokens: &[
                TemplateToken::String(Output::Dedent),
                TemplateToken::str("}"),
                TemplateToken::String(Output::NewLine),
            ],
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
                TemplateToken::str("let "),
                TemplateToken::OutVar(0),
                TemplateToken::str(" = ("),
                TemplateToken::InVar(0),
                TemplateToken::str(") + ("),
                TemplateToken::InVar(1),
                TemplateToken::str(");"),
                TemplateToken::String(Output::NewLine),
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
                TemplateToken::str("const "),
                TemplateToken::OutVar(0),
                TemplateToken::str("= 1;"),
                TemplateToken::String(Output::NewLine),
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
    output: &mut OutputWriter<impl Write>,
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

        for bracket_handler in builtin.brachet_handlers.iter().rev() {
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
    let program = "11+r,1+1+r]1]";

    let mut chars = program.chars();

    let mut writer = OutputWriter::new(std::io::stdout());
    transpile_program(&mut chars, &mut writer).unwrap();
    println!();
}

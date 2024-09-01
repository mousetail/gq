use std::{collections::HashMap, io::Write};

use builtin::{BracketHandler, Builtin};
use fragment::dispose_bracket_handler;
use stack::Stack;

mod builtin;
mod fragment;
mod output_writer;
mod stack;
mod varnames;
use output_writer::OutputWriter;
use template_macros::{fragment, half_fragment};
use template_types::{Output, ProgramFragment, TemplateToken};

const BUILTINS: &'static [Builtin] = &[
    Builtin {
        token: ',',
        template: fragment!(
            "
            const {wrapper:local} = () => {{
                {inner}
            "
        ),
        bracket_handlers: &[
            BracketHandler {
                output_handler: Some(half_fragment!(
                    "
                    {output:local}({value:in});
                    "
                )),
                fragment: ProgramFragment {
                    arguments_popped: 0,
                    arguments_pushed: 0,
                    init_tokens: &[],
                    destruct_tokens: &[],
                },
            },
            BracketHandler {
                output_handler: Some(half_fragment!(
                    "
                    {output:local}({value:in});
                    "
                )),
                fragment: fragment!(
                    "
                        //
                    }};
                    const {output:local} = ({out_var:out})=>{{
                        {inner}
                    }}

                    {wrapper:local}();
                "
                ),
            },
        ],
    },
    Builtin {
        token: '[',
        template: fragment!(
            "
            const {arr:local} = [];
            { inner }
        "
        ),
        bracket_handlers: &[BracketHandler {
            output_handler: Some(half_fragment!(
                "
                {arr:local}.push({value:in});
            "
            )),
            fragment: fragment!(
                "
                const {out:out} = {arr:local};
                {inner}
                "
            ),
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
        bracket_handlers: &[],
    },
    Builtin {
        token: '+',
        template: fragment!(
            "
            const {out:out} = {op1:in} + {op2:in};
            { inner }
        "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        token: '1',
        template: fragment!(
            "
            const {out:out} = 1;
            "
        ),
        bracket_handlers: &[],
    },
];

fn transpile_program(
    iter: &mut impl Iterator<Item = char>,
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
    let program = "11+r[,1+1+r]1]]";

    let mut chars = program.chars();

    let mut writer = OutputWriter::new(std::io::stdout());
    println!("(()=>{{");
    transpile_program(&mut chars, &mut writer).unwrap();
    println!();
    println!("}})()");
}

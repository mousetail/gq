use std::{collections::HashMap, io::Write};

use varnames::VarNames;

mod varnames;

enum TemplateToken {
    ArgumentStart,
    ArgumentEnd,
    ArgumentVar(usize),
    String(&'static str),
    ArgumentOut,
}

struct Builtin {
    token: char,
    args: usize,
    template_before: &'static [TemplateToken],
    template_after: &'static [TemplateToken]
}

const BUILTINS: &'static [Builtin] = &[
    Builtin {
        token: ']',
        args: 1,
        template_before: &[
            TemplateToken::String("let "),
            TemplateToken::ArgumentOut,
            TemplateToken::String("= [];"),
            TemplateToken::ArgumentStart,
            TemplateToken::ArgumentOut,
            TemplateToken::String(".push("),
            TemplateToken::ArgumentVar(0),
            TemplateToken::String(");\n"),
            TemplateToken::ArgumentEnd
        ],
        template_after: &[
        ],
    },
    Builtin {
        token: 'r',
        args: 1,
        template_before:&[
            TemplateToken::ArgumentStart,
            TemplateToken::String("for (let "),
            TemplateToken::ArgumentOut,
            TemplateToken::String("=0; "),
            TemplateToken::ArgumentOut,
            TemplateToken::String("<("),
            TemplateToken::ArgumentVar(0),
            TemplateToken::String("); "),
            TemplateToken::ArgumentOut,
            TemplateToken::String("++)\n {"),
        ],
        template_after: &[
            TemplateToken::String("}\n"),
            TemplateToken::ArgumentEnd,
        ],
    },
    Builtin {
        token: '+',
        args: 2,
        template_before: &[
            TemplateToken::ArgumentStart,
            TemplateToken::ArgumentStart,
            TemplateToken::String("let "),
            TemplateToken::ArgumentOut,
            TemplateToken::String(" = ("),
            TemplateToken::ArgumentVar(0),
            TemplateToken::String(") + ("),
            TemplateToken::ArgumentVar(1),
            TemplateToken::String(");\n"),
        ],
        template_after: &[
            TemplateToken::ArgumentEnd,
            TemplateToken::ArgumentEnd
        ],
    },
    Builtin {
        token: '1',
        args: 0,
        template_before: &[
            TemplateToken::String("let "),
            TemplateToken::ArgumentOut,
            TemplateToken::String("= 1;\n"),
        ],
        template_after: &[],
    },
];

struct VarArgs(Vec<(String, &'static Builtin, VarArgs)>);

fn transpile_tokens(tokens: &[TemplateToken], iter: &mut impl Iterator<Item = char>, 
    output: &mut impl Write,
    varnames: &mut impl Iterator<Item = String>,
    arg_vars: &mut VarArgs,
    out_name: &str
) -> std::io::Result<()> {
    for outer_token in tokens {
        match outer_token {
            TemplateToken::ArgumentVar(n) => write!(output, "{}", arg_vars.0.get(*n).expect("var {n} not defined").0)?,
            TemplateToken::ArgumentStart => {

                let name = transpile_program_start(iter, output, varnames)?;
                arg_vars.0.push(name)
            },
            TemplateToken::ArgumentEnd => {
                let mut top = arg_vars.0.pop().unwrap();
                transpile_tokens(
                    top.1.template_after,
                    iter,
                    output,
                    varnames,
                    &mut top.2,
                    &top.0
                )?;
            }
            TemplateToken::String(s) => write!(output, "{}", s)?,
            TemplateToken::ArgumentOut => write!(output, "{}", out_name)?,
            
        }
    }
    return Ok(());
}

fn transpile_program_start(
    iter: &mut impl Iterator<Item = char>,
    output: &mut impl Write,
    varnames: &mut impl Iterator<Item = String>,
) -> std::io::Result<(String, &'static Builtin, VarArgs)> {
    let char = iter.next().unwrap();
    let builtin = BUILTINS.iter().find(|d| d.token == char).unwrap();

    let out_name = varnames.next().unwrap();

    let mut var_args = VarArgs(vec![]);
    transpile_tokens(&builtin.template_before, iter, output, varnames, &mut var_args, &out_name)?;

    Ok((out_name, builtin, var_args))

}

fn main() {
    let program = "11+r1+1+]r";

    let mut chars = program.chars().rev();
    let mut varnames = VarNames::default();
    let mut out = std::io::stdout();

    let (final_name, final_builtin, mut final_args) = transpile_program_start(&mut chars, &mut out, &mut varnames).unwrap();

    write!(out, "console.log({});\n", final_name).unwrap();
    transpile_tokens(&final_builtin.template_after, &mut chars, &mut out, &mut varnames, &mut final_args, &final_name).unwrap();
    println!();
}

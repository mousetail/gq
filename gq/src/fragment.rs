use template_types::{HighestVarNumbers, Output, ProgramFragment, TemplateToken};

use crate::{
    builtin::OutputHandler,
    output_writer::OutputWriter,
    stack::{Stack, StackBracketGroup},
};
use std::{collections::HashMap, io::Write, iter::once};

pub fn write_fragment(
    output: &mut OutputWriter<impl Write>,
    fragment: ProgramFragment<'static>,
    stack: &mut Stack,
    local_vars: &HashMap<String, String>,
) -> std::io::Result<()> {
    let in_vars: Vec<_> = (0..fragment.arguments_popped)
        .map(|_| stack.pop())
        .collect();
    let out_vars: Vec<_> = (0..fragment.arguments_pushed)
        .map(|_| stack.push())
        .collect();

    write_tokens(
        output,
        fragment.init_tokens,
        local_vars,
        &in_vars,
        &out_vars,
    )?;

    stack.current_group.destructors.push(Destructor {
        fragments: fragment.destruct_tokens,
        in_vars,
        out_vars,
        local_vars: local_vars.clone(),
    });

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Destructor {
    pub fragments: &'static [TemplateToken<'static>],
    pub local_vars: HashMap<String, String>,
    pub in_vars: Vec<String>,
    pub out_vars: Vec<String>,
}

pub fn write_output_handler(
    output: &mut OutputWriter<impl Write>,
    handler: &'static [TemplateToken],
    input_vars: &[String],
    local_vars: &HashMap<String, String>,
) -> std::io::Result<()> {
    for token in handler {
        match token {
            TemplateToken::InVar(k) => output.write(Output::String(
                input_vars.get(*k).map(|d| d.as_str()).unwrap_or(""),
            ))?,
            TemplateToken::OutVar(_) => panic!("Output handlers can not produce output {token:?}"),
            TemplateToken::String(s) => output.write(s.clone())?,
            TemplateToken::LocalVar(n) => {
                output.write(Output::String(local_vars.get(*n).unwrap().as_str()))?
            }
        }
    }

    Ok(())
}

pub fn handle_group_output(
    output: &mut OutputWriter<impl Write>,
    handler: OutputHandler,
    input_vars: &[String],
    local_vars: &HashMap<String, String>,
) -> std::io::Result<()> {
    let number_of_input_vars = handler.fragment.get_number_of_input_vars();

    match handler.behavior {
        crate::builtin::MultiOutputBehavior::OnlyFirst => {
            if number_of_input_vars > input_vars.len() {
                panic!(
                    "This group requires at least {number_of_input_vars} values left on the stack"
                )
            }
            write_output_handler(
                output,
                handler.fragment,
                &input_vars[..number_of_input_vars],
                local_vars,
            )?
        }
        crate::builtin::MultiOutputBehavior::FlattenAll => {
            assert_eq!(
                number_of_input_vars, 1,
                "Flatten all can only be used with a single input var"
            );

            for i in 0..input_vars.len() {
                write_output_handler(output, handler.fragment, &input_vars[i..i + 1], local_vars)?;
            }
        } // crate::builtin::MultiOutputBehavior::Array => todo!(),
          // crate::builtin::MultiOutputBehavior::Variadic => todo!(),
    }

    Ok(())
}

pub fn dispose_bracket_handler(
    output: &mut OutputWriter<impl Write>,
    bracket_handler: StackBracketGroup,
    stack: &mut Stack,
) -> std::io::Result<()> {
    handle_group_output(
        output,
        once(bracket_handler.output_handler)
            .chain(stack.output_handlers())
            .flatten()
            .next()
            .unwrap(),
        &bracket_handler.stack,
        &bracket_handler.local_variables,
    )?;

    for destructor in bracket_handler.destructors.iter().rev() {
        write_tokens(
            output,
            destructor.fragments,
            &destructor.local_vars,
            &destructor.in_vars,
            &destructor.out_vars,
        )?;
    }

    write_fragment(
        output,
        bracket_handler.brackent_end_fragment,
        stack,
        &bracket_handler.local_variables,
    )?;

    Ok(())
}

pub fn write_tokens(
    output: &mut OutputWriter<impl Write>,
    tokens: &[TemplateToken],
    local_vars: &HashMap<String, String>,
    in_vars: &[String],
    out_vars: &[String],
) -> std::io::Result<()> {
    for token in tokens
        .into_iter()
        .copied()
        .chain(Some(TemplateToken::String(Output::NewLine)))
    {
        match token {
            TemplateToken::InVar(n) => output.write(Output::String(in_vars[n].as_str()))?,
            TemplateToken::OutVar(n) => output.write(Output::String(out_vars[n].as_str()))?,
            TemplateToken::String(val) => output.write(val.clone())?,
            TemplateToken::LocalVar(n) => {
                output.write(Output::String(local_vars.get(n).unwrap().as_str()))?
            }
        }
    }

    Ok(())
}

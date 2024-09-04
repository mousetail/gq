use template_types::{HighestVarNumbers, Output, ProgramFragment, TemplateToken};

use crate::language::{
    builtin::OutputHandler,
    output_writer::OutputWriter,
    stack::{Stack, StackBracketGroup},
};
use std::{collections::HashMap, io::Write};

use super::{builtin::MultiOutputBehavior, stack::OutputHandlerInstance};

pub fn write_variadic_fragment(
    output: &mut OutputWriter<impl Write>,
    fragment: ProgramFragment<'static>,
    stack: &mut Stack,
    local_vars: &HashMap<String, String>,
    outputs: usize,
) -> std::io::Result<()> {
    let in_vars: Vec<_> = (0..fragment.arguments_popped)
        .map(|_| stack.pop())
        .collect();

    assert_eq!(
        fragment.arguments_pushed, 1,
        "Variadic fragments can only push one argument"
    );
    let out_vars: Vec<_> = (0..outputs).map(|_| stack.push()).collect();

    write_tokens(
        output,
        fragment.init_tokens,
        local_vars,
        &in_vars,
        &[out_vars.join(", ")],
    )?;

    stack.current_group.destructors.push(Destructor {
        fragments: fragment.destruct_tokens,
        in_vars,
        out_vars,
        local_vars: local_vars.clone(),
    });

    Ok(())
}

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

fn write_output_handler(
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
    handler: &OutputHandlerInstance,
    input_vars: &[String],
    local_vars: &HashMap<String, String>,
) -> std::io::Result<()> {
    let number_of_input_vars = handler.inner.fragment.get_number_of_input_vars();

    match handler.inner.behavior {
        crate::language::builtin::MultiOutputBehavior::OnlyFirst => {
            if number_of_input_vars > input_vars.len() {
                panic!(
                    "This group requires at least {number_of_input_vars} values left on the stack"
                )
            }
            write_output_handler(
                output,
                handler.inner.fragment,
                &input_vars[..number_of_input_vars],
                local_vars,
            )?
        }
        crate::language::builtin::MultiOutputBehavior::FlattenAll => {
            assert_eq!(
                number_of_input_vars, 1,
                "Flatten all can only be used with a single input var"
            );

            for i in 0..input_vars.len() {
                write_output_handler(
                    output,
                    handler.inner.fragment,
                    &input_vars[i..i + 1],
                    local_vars,
                )?;
            }
        }
        crate::language::builtin::MultiOutputBehavior::Array
        | crate::language::builtin::MultiOutputBehavior::Variadic => {
            assert_eq!(
                number_of_input_vars, 1,
                "Array all can only be used with a single input var"
            );

            handler.max_variadic_outputs.set(
                handler
                    .max_variadic_outputs
                    .get()
                    .max(Some(input_vars.len())),
            );

            write_output_handler(
                output,
                handler.inner.fragment,
                &[input_vars.join(", ")],
                local_vars,
            )?;
        }
    }

    Ok(())
}

pub fn dispose_bracket_handler(
    output: &mut OutputWriter<impl Write>,
    bracket_handler: StackBracketGroup,
    stack: &mut Stack,
) -> std::io::Result<()> {
    let (output_handler, local_vars) = bracket_handler
        .get_output_handler_context()
        .unwrap_or_else(|| stack.get_output_handler());

    handle_group_output(output, output_handler, &bracket_handler.stack, local_vars)?;

    for destructor in bracket_handler.destructors.iter().rev() {
        write_tokens(
            output,
            destructor.fragments,
            &destructor.local_vars,
            &destructor.in_vars,
            &destructor.out_vars,
        )?;
    }

    if let Some(OutputHandlerInstance {
        inner:
            OutputHandler {
                behavior: MultiOutputBehavior::Variadic,
                ..
            },
        ..
    }) = bracket_handler.output_handler
    {
        write_variadic_fragment(
            output,
            bracket_handler.brackent_end_fragment,
            stack,
            &bracket_handler.local_variables,
            output_handler.max_variadic_outputs.get().unwrap_or(0),
        )?;
    } else {
        write_fragment(
            output,
            bracket_handler.brackent_end_fragment,
            stack,
            &bracket_handler.local_variables,
        )?;
    }

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

pub fn write_comma(
    output: &mut OutputWriter<impl Write>,
    stack: &mut Stack,
) -> std::io::Result<()> {
    let (output_handler, _local_vars) = stack.get_output_handler();
    let number_of_inputs = output_handler.inner.fragment.get_number_of_input_vars();

    let mut stack_values: Vec<_> = (0..number_of_inputs).map(|_| stack.pop()).collect();
    stack_values.reverse();

    // Need to call this twice since I can't borrow accross the two stack values calls
    let (output_handler, local_vars) = stack.get_output_handler();

    write_tokens(
        output,
        output_handler.inner.fragment,
        local_vars,
        &stack_values,
        &[],
    )?;

    for destructor in stack.destruct_unused_vars().iter().rev() {
        write_tokens(
            output,
            destructor.fragments,
            &destructor.local_vars,
            &destructor.in_vars,
            &destructor.out_vars,
        )?;
    }

    Ok(())
}

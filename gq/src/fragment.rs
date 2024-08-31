use crate::{
    builtin::TemplateToken,
    output_writer::{Output, OutputWriter},
    stack::{Stack, StackBracketGroup},
};
use std::{io::Write, iter::once};

#[derive(Default, Clone, Copy, Debug)]
pub struct ProgramFragment {
    pub init_tokens: &'static [TemplateToken],
    pub destruct_tokens: &'static [TemplateToken],
    pub arguments_popped: usize,
    pub arguments_pushed: usize,
}

pub fn write_fragment(
    output: &mut OutputWriter<impl Write>,
    fragment: ProgramFragment,
    stack: &mut Stack,
    local_vars: &[String],
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
        local_vars: local_vars.to_vec(),
    });

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Destructor {
    pub fragments: &'static [TemplateToken],
    pub local_vars: Vec<String>,
    pub in_vars: Vec<String>,
    pub out_vars: Vec<String>,
}

pub fn write_output_handler(
    output: &mut OutputWriter<impl Write>,
    handler: &mut impl Iterator<Item = &'static [TemplateToken]>,
    input_vars: &[String],
    local_vars: &[String],
) -> std::io::Result<()> {
    let value = handler.next().unwrap();
    for token in value {
        match token {
            TemplateToken::InVar(k) => output.write(Output::String(
                input_vars.get(*k).map(|d| d.as_str()).unwrap_or(""),
            ))?,
            TemplateToken::OutVar(_) => panic!("Output handlers can not produce output {value:?}"),
            TemplateToken::String(s) => output.write(s.clone())?,
            TemplateToken::LocalVar(n) => output.write(Output::String(local_vars[*n].as_str()))?,
            TemplateToken::PreviousOuput => {
                write_output_handler(output, handler, input_vars, local_vars)?
            }
        }
    }

    Ok(())
}

pub fn dispose_bracket_handler(
    output: &mut OutputWriter<impl Write>,
    bracket_handler: StackBracketGroup,
    stack: &mut Stack,
) -> std::io::Result<()> {
    write_output_handler(
        output,
        &mut once(bracket_handler.output_handler)
            .chain(stack.output_handlers())
            .flatten(),
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
    local_vars: &[String],
    in_vars: &[String],
    out_vars: &[String],
) -> std::io::Result<()> {
    for token in tokens {
        match token {
            TemplateToken::InVar(n) => output.write(Output::String(in_vars[*n].as_str()))?,
            TemplateToken::OutVar(n) => output.write(Output::String(out_vars[*n].as_str()))?,
            TemplateToken::String(val) => output.write(val.clone())?,
            TemplateToken::LocalVar(n) => output.write(Output::String(local_vars[*n].as_str()))?,
            TemplateToken::PreviousOuput => panic!("Use of previous output outside output handler"),
        }
    }

    Ok(())
}

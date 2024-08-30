use std::io::Write;

#[derive(Clone, Debug)]
pub enum Output<'a> {
    String(&'a str),
    NewLine,
    Indent,
    Dedent,
}

impl Output<'static> {
    pub const fn str(s: &'static str) -> Output {
        Output::String(s)
    }
}

pub struct OutputWriter<W: Write> {
    writer: W,
    indent: usize,
    last_token_was_newline: bool,
}

impl<W: Write> OutputWriter<W> {
    pub fn new(writer: W) -> Self {
        OutputWriter {
            writer,
            indent: 0,
            last_token_was_newline: false,
        }
    }

    pub fn write(&mut self, token: Output) -> std::io::Result<()> {
        match token {
            Output::String(s) => {
                if self.last_token_was_newline {
                    write!(self.writer, "\n{:1$}", "", self.indent * 4)?;
                    self.last_token_was_newline = false;
                }
                write!(self.writer, "{}", s)
            }
            Output::NewLine => {
                self.last_token_was_newline = true;
                Ok(())
            }
            Output::Indent => Ok(self.indent += 1),
            Output::Dedent => Ok(self.indent = self.indent.saturating_sub(1)),
        }
    }
}

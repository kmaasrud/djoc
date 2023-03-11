//! LaTeX renderer for jotdown.
//!
//! The output should mostly match that of Pandoc, though there may be slight differences.

use jotdown::{Container, Event, Render};
use std::fmt;

#[derive(Default)]
pub struct Renderer {
    number_sections: bool,
}

impl Render for Renderer {
    fn push<'s, I: Iterator<Item = Event<'s>>, W: fmt::Write>(
        &self,
        events: I,
        out: W,
    ) -> fmt::Result {
        Writer::from(self).push(events, out)
    }
}

impl From<&Renderer> for Writer {
    fn from(r: &Renderer) -> Self {
        Self {
            number_sections: r.number_sections,
            raw: Raw::None,
            first_line: true,
        }
    }
}

enum Raw {
    None,
    Latex,
    Other,
}

struct Writer {
    number_sections: bool,
    raw: Raw,
    first_line: bool,
}

impl Writer {
    fn push<'s, I: Iterator<Item = jotdown::Event<'s>>, W: fmt::Write>(
        &mut self,
        events: I,
        mut out: W,
    ) -> fmt::Result {
        for e in events {
            match e {
                Event::Str(s) => match self.raw {
                    Raw::None => write_escaped(&mut out, &s)?,
                    Raw::Latex => out.write_str(&s)?,
                    Raw::Other => {}
                },
                Event::Symbol(sym) => write!(out, ":{}:", sym)?,
                Event::LeftSingleQuote => out.write_str("`")?,
                Event::RightSingleQuote => out.write_char('\'')?,
                Event::LeftDoubleQuote => out.write_str("``")?,
                Event::RightDoubleQuote => out.write_char('"')?,
                Event::Ellipsis => out.write_str(r"\ldots")?,
                Event::EnDash => out.write_str(r"\textendash")?,
                Event::EmDash => out.write_str(r"\textemdash")?,
                Event::NonBreakingSpace => out.write_char('~')?,
                Event::Softbreak => out.write_str("\n")?,
                Event::Hardbreak => out.write_str("\\\\\n")?,
                Event::Escape | Event::Blankline => {}
                Event::ThematicBreak(_attrs) => {
                    out.write_str("\n\\begin{center}\\rule{0.5\\linewidth}{0.5pt}\\end{center}")?
                }
                Event::FootnoteReference(_, number) => write!(out, r"\footnotemark[{}]", number)?,
                Event::Start(c, _attrs) => {
                    if self.first_line {
                        self.first_line = false;
                    } else if c.is_block() && !matches!(c, Container::Section { .. }) {
                        out.write_char('\n')?;
                    }
                    match c {
                        Container::Paragraph => {}
                        Container::Blockquote => out.write_str(r"\begin{quote}")?,
                        Container::Strong => out.write_str(r"\textbf{")?,
                        Container::Emphasis => out.write_str(r"\textit{")?,
                        Container::Superscript => out.write_str(r"\textsuperscript{")?,
                        Container::Subscript => out.write_str(r"\textsubscript{")?,
                        Container::Insert => out.write_str(r"\ul{")?,
                        Container::Delete => out.write_str(r"\st{")?,
                        Container::Mark => out.write_str(r"\hl{")?,
                        Container::Link(dest, _) => write!(out, r"\href{{{}}}{{", dest)?,
                        Container::Footnote { number, .. } => {
                            write!(out, r"\footnotetext[{}]{{", number)?
                        }
                        Container::Section { .. } => {}
                        Container::Heading { level, id, .. } => {
                            out.write_str(r"\hypertarget{")?;
                            write_escaped(&mut out, &id)?;
                            out.write_str("}{%\n")?;
                            match level {
                                1 => out.write_str(r"\section")?,
                                2 => out.write_str(r"\subsection")?,
                                3 => out.write_str(r"\subsubsection")?,
                                4 => out.write_str(r"\paragraph")?,
                                5 => out.write_str(r"\subparagraph")?,
                                _ => {}
                            }
                            if self.number_sections {
                                out.write_char('*')?;
                            }
                            out.write_char('{')?;
                        }
                        Container::RawBlock { format } | Container::RawInline { format } => {
                            self.raw = match format {
                                "latex" => Raw::Latex,
                                _ => Raw::Other,
                            }
                        }
                        Container::Math { display } => {
                            self.raw = Raw::Latex;
                            match display {
                                true => out.write_str(r"\[")?,
                                false => out.write_str(r"\(")?,
                            }
                        }
                        _ => {}
                    }
                }
                Event::End(c) => match c {
                    Container::Paragraph => out.write_str("\n")?,
                    Container::Section { .. } => {}
                    Container::Heading { id, .. } => {
                        out.write_str("}")?;
                        out.write_str(r"\label{")?;
                        write_escaped(&mut out, &id)?;
                        out.write_str(r"}}")?;
                        out.write_char('\n')?
                    }
                    Container::Blockquote => out.write_str("\\end{quote}\n")?,
                    Container::Strong
                    | Container::Emphasis
                    | Container::Superscript
                    | Container::Subscript
                    | Container::Insert
                    | Container::Delete
                    | Container::Mark
                    | Container::Link(_, _)
                    | Container::Footnote { .. } => out.write_str("}")?,
                    Container::RawBlock { .. } | Container::RawInline { .. } => {
                        self.raw = Raw::None
                    }
                    Container::Math { display } => {
                        self.raw = Raw::None;
                        match display {
                            true => out.write_str(r"\]")?,
                            false => out.write_str(r"\)")?,
                        }
                    }
                    _ => {}
                },
            }
            self.first_line = false;
        }

        Ok(())
    }
}

fn write_escaped<W: fmt::Write>(mut w: W, mut s: &str) -> fmt::Result {
    let mut escape = "";
    while let Some(i) = s.find(|c| {
        match c {
            '&' => Some(r"\&"),
            '%' => Some(r"\%"),
            '$' => Some(r"\$"),
            '#' => Some(r"\#"),
            '_' => Some(r"\_"),
            '{' => Some(r"\{"),
            '}' => Some(r"\}"),
            '~' => Some(r"\textasciitilde"),
            '^' => Some(r"\textasciicircum"),
            '\\' => Some(r"\textasciibackslash"),
            _ => None,
        }
        .map_or(false, |esc| {
            escape = esc;
            true
        })
    }) {
        w.write_str(&s[..i])?;
        w.write_str(escape)?;
        s = &s[i + 1..];
    }
    w.write_str(s)
}

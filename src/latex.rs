//! LaTeX renderer for jotdown.
//!
//! The output should mostly match that of Pandoc, though there may be slight
//! differences.

use std::fmt;

use jotdown::{Container, Event, ListKind, OrderedListNumbering, OrderedListStyle, Render};

#[derive(Default)]
enum Emit {
    #[default]
    Escaped,
    Raw,
    None,
}

#[derive(Default)]
pub struct Renderer {
    pub number_sections: bool,
    emit: Emit,
    first_line: bool,
}

impl Renderer {
    #[must_use]
    pub fn number_sections(self, number_sections: bool) -> Self {
        Self {
            number_sections,
            ..self
        }
    }
}

impl Render for Renderer {
    fn render_event<W>(&mut self, e: &Event, mut out: W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match e {
            Event::Str(s) => match self.emit {
                Emit::Escaped => write_escaped(&mut out, s)?,
                Emit::Raw => out.write_str(s)?,
                Emit::None => {}
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
            Event::Softbreak => writeln!(out)?,
            Event::Hardbreak => writeln!(out, r"\\")?,
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
                    Container::Paragraph
                    | Container::Section { .. }
                    | Container::DescriptionDetails
                    | Container::Div { .. } => {}
                    Container::Blockquote => out.write_str(r"\begin{quote}")?,
                    Container::DescriptionList => writeln!(out, r"\begin{{description}}")?,
                    Container::Span => out.write_char('{')?,
                    Container::Strong => out.write_str(r"\textbf{")?,
                    Container::Emphasis => out.write_str(r"\textit{")?,
                    Container::Superscript => out.write_str(r"\textsuperscript{")?,
                    Container::Subscript => out.write_str(r"\textsubscript{")?,
                    Container::Insert => out.write_str(r"\ul{")?,
                    Container::Delete => out.write_str(r"\st{")?,
                    Container::Mark => out.write_str(r"\hl{")?,
                    Container::Link(dest, _) => write!(out, r"\href{{{}}}{{", dest)?,
                    Container::DescriptionTerm => write!(out, r"\item[")?,
                    Container::Footnote { number, .. } => {
                        write!(out, r"\footnotetext[{}]{{", number)?
                    }
                    Container::Image(dest, _) => {
                        writeln!(out, r"\begin{{figure}}")?;
                        writeln!(out, r"\centering")?;
                        write!(out, r"\includegraphics[width=\textwidth]{{{dest}}}")?;
                        out.write_str(r"\caption{")?;
                    }
                    Container::Heading { level, id, .. } if *level < 6 => {
                        out.write_str(r"\hypertarget{")?;
                        write_escaped(&mut out, id)?;
                        out.write_str("}{%\n")?;
                        match level {
                            1 => out.write_str(r"\section")?,
                            2 => out.write_str(r"\subsection")?,
                            3 => out.write_str(r"\subsubsection")?,
                            4 => out.write_str(r"\paragraph")?,
                            5 => out.write_str(r"\subparagraph")?,
                            _ => {}
                        }
                        if !self.number_sections {
                            out.write_char('*')?;
                        }
                        out.write_char('{')?;
                    }
                    Container::RawBlock { format } | Container::RawInline { format } => {
                        self.emit = match *format {
                            "latex" | "tex" => Emit::Raw,
                            _ => Emit::None,
                        }
                    }
                    Container::Math { display } => {
                        self.emit = Emit::Raw;
                        match display {
                            true => out.write_str(r"\[")?,
                            false => out.write_str(r"\(")?,
                        }
                    }
                    Container::List { kind, tight } => {
                        out.write_str(r"\begin")?;
                        match kind {
                            ListKind::Unordered => writeln!(out, "{{itemize}}")?,
                            ListKind::Ordered {
                                numbering,
                                style,
                                start,
                            } => {
                                out.write_str("{enumerate}[label=")?;
                                if let OrderedListStyle::ParenParen = style {
                                    out.write_char('(')?;
                                }
                                match numbering {
                                    OrderedListNumbering::Decimal => write!(out, r"\arabic*")?,
                                    OrderedListNumbering::AlphaLower => write!(out, r"\alph*")?,
                                    OrderedListNumbering::AlphaUpper => write!(out, r"\Alph*")?,
                                    OrderedListNumbering::RomanLower => write!(out, r"\roman*")?,
                                    OrderedListNumbering::RomanUpper => write!(out, r"\Roman*")?,
                                }
                                match style {
                                    OrderedListStyle::Period => out.write_char('.')?,
                                    _ => out.write_char(')')?,
                                }
                                writeln!(out, ", start={}]", start)?;
                            }
                            ListKind::Task => writeln!(out, "{{tasklist}}")?,
                        }
                        if *tight {
                            out.write_str(r"\tightlist")?;
                        }
                    }
                    Container::ListItem => out.write_str(r"\item")?,
                    Container::Verbatim => {
                        self.emit = Emit::Raw;
                        write!(out, r"\verb|")?;
                    }
                    Container::CodeBlock { lang: _ } => {
                        self.emit = Emit::Raw;
                        write!(out, r"\begin{{verbatim}}")?;
                        writeln!(out)?;
                    }
                    Container::TaskListItem { checked } => {
                        out.write_str(r"\item")?;
                        if *checked {
                            out.write_str(r"[\done]")?;
                        }
                    }
                    _ => {}
                }
            }
            Event::End(c) => match c {
                Container::Section { .. }
                | Container::ListItem
                | Container::DescriptionDetails
                | Container::Div { .. } => {}
                Container::Paragraph => out.write_str("\n")?,
                Container::Heading { level, id, .. } if *level < 6 => {
                    write!(out, r"}}\label{{{id}}}}}")?;
                    out.write_char('\n')?
                }
                Container::Blockquote => writeln!(out, r"\end{{quote}}")?,
                Container::Image(_, _) => out.write_str("}\n\\end{figure}\n")?,
                Container::DescriptionList => writeln!(out, r"\end{{description}}")?,
                Container::DescriptionTerm => writeln!(out, r"]")?,
                Container::List { kind, .. } => {
                    out.write_str(r"\end{")?;
                    match kind {
                        ListKind::Unordered => out.write_str("itemize")?,
                        ListKind::Ordered { .. } => out.write_str("enumerate")?,
                        ListKind::Task => out.write_str("tasklist")?,
                    }
                    out.write_str("}")?;
                }
                Container::Strong
                | Container::Emphasis
                | Container::Superscript
                | Container::Subscript
                | Container::Insert
                | Container::Delete
                | Container::Mark
                | Container::Span
                | Container::Link(_, _)
                | Container::Footnote { .. } => out.write_char('}')?,
                Container::RawBlock { .. } | Container::RawInline { .. } => {
                    self.emit = Emit::Escaped
                }
                Container::Math { display } => {
                    self.emit = Emit::Escaped;
                    match display {
                        true => out.write_str(r"\]")?,
                        false => out.write_str(r"\)")?,
                    }
                }
                Container::Verbatim => {
                    self.emit = Emit::Escaped;
                    out.write_char('|')?;
                }
                Container::CodeBlock { .. } => {
                    self.emit = Emit::Escaped;
                    writeln!(out, r"\end{{verbatim}}")?;
                }
                _ => {}
            },
        }
        self.first_line = false;
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

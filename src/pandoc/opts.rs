use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

pub enum PandocOption {
    // Boolean opts
    Citeproc,
    NumberSections,
    Standalone,
    LinkCitations,
    Katex,

    // Other opts
    From(PandocFormat),
    To(PandocFormat),
    Csl(PathBuf),
    LuaFilter(PathBuf),
    Bibliography(PathBuf),
    IncludeInHeader(PathBuf),
    Template(PathBuf),

    // Custom opts (through the `metadata`/`variable` flag)
    Title(String),
    TitleScript(String),
    Author(String),
    Date(String),
    DocumentClass(String),
}

impl Display for PandocOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use PandocOption::*;
        match *self {
            Citeproc => write!(f, "-C"),
            NumberSections => write!(f, "--number-sections"),
            Standalone => write!(f, "-s"),
            LinkCitations => write!(f, "--metadata=link-citations"),
            Katex => write!(f, "--katex"),
            From(ref from) => write!(f, "--from={from}"),
            To(ref to) => write!(f, "--to={to}"),
            Csl(ref path) => write!(f, "--csl={}", path.display()),
            LuaFilter(ref path) => write!(f, "--lua-filter={}", path.display()),
            Bibliography(ref path) => write!(f, "--bibliography={}", path.display()),
            IncludeInHeader(ref path) => write!(f, "--include-in-header={}", path.display()),
            Template(ref path) => write!(f, "--template={}", path.display()),
            Title(ref title) => write!(f, "--variable=title:{title}"),
            TitleScript(ref title_script) => write!(f, "--variable=title-script:{title_script}"),
            Author(ref author) => write!(f, "--variable=author:{author}"),
            Date(ref date) => write!(f, "--variable=date:{date}"),
            DocumentClass(ref class) => write!(f, "--variable=documentclass:{class}"),
        }
    }
}

pub enum PandocFormat {
    Html,
    Latex,
    Markdown,
}

impl Display for PandocFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            PandocFormat::Html => write!(f, "html"),
            PandocFormat::Latex => write!(f, "latex"),
            PandocFormat::Markdown => write!(f, "markdown"),
        }
    }
}

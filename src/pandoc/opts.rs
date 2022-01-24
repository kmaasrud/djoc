use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

pub enum PandocOption {
    // Boolean opts
    Citeproc,
    NumberSections,
    Standalone,
    LinkCitations,

    // Other opts
    From(PandocFormat),
    To(PandocFormat),
    Csl(PathBuf),
    LuaFilter(PathBuf),
    Bibliography(PathBuf),
    IncludeInHeader(PathBuf),

    // Custom opts (through the `metadata` flag)
    Title(String),
    Author(String),
    Date(String),
}

impl Display for PandocOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use PandocOption::*;
        match *self {
            Citeproc => write!(f, "-C"),
            NumberSections => write!(f, "--number-sections"),
            Standalone => write!(f, "-s"),
            LinkCitations => write!(f, "--metadata=link-citations"),
            From(ref from) => write!(f, "--from={from}"),
            To(ref to) => write!(f, "--to={to}"),
            Csl(ref path) => write!(f, "--csl={}", path.display()),
            LuaFilter(ref path) => write!(f, "--lua-filter={}", path.display()),
            Bibliography(ref path) => write!(f, "--bibliography={}", path.display()),
            IncludeInHeader(ref path) => write!(f, "--include-in-header={}", path.display()),
            Title(ref title) => write!(f, "--variable=title:{title}"),
            Author(ref author) => write!(f, "--variable=author:{author}"),
            Date(ref date) => write!(f, "--variable=date:{date}"),
        }
    }
}

pub enum PandocFormat {
    Markdown,
    Latex,
}

impl Display for PandocFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            PandocFormat::Markdown => write!(f, "markdown"),
            PandocFormat::Latex => write!(f, "latex"),
        }
    }
}

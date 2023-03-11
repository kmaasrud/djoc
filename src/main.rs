use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cmd;

#[derive(Parser)]
#[command(author, version, about)]
struct Djoc {
    #[command(subcommand)]
    command: Command,

    #[arg(global = true, short, long)]
    /// Make djoc quiet. Only errors will get reported.
    quiet: bool,

    #[arg(global = true, short, long)]
    /// Make djoc's output verbose. Used for debugging.
    debug: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Builds a file or document.
    Build {
        /// File to build into PDF (optional).
        #[arg(default_value = ".")]
        path: PathBuf,

        #[arg(short, long, default_value = "pdf")]
        /// The type of output you want to build to.
        format: String,

        #[arg(short, long)]
        /// The file to write to
        output: Option<String>,
    },

    /// Initializes a new document.
    Init {
        /// Directory to initialize the document in.
        path: Option<PathBuf>,
    },

    Reference {
        path: PathBuf,
    },
}

fn run() -> Result<()> {
    let app = Djoc::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(app.quiet)
        .verbosity(4)
        .init()?;

    match app.command {
        Command::Build {
            path,
            format,
            output,
        } => cmd::build(path, format, output)?,
        Command::Init { path } => cmd::init(path)?,
        Command::Reference { path } => cmd::reference(path)?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}

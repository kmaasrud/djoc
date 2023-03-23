mod build;
mod compile;
mod init;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Build,

    Compile {
        /// The target source file (omit for stdin)
        path: Option<PathBuf>,
        /// The type of output you want to build to.
        #[arg(short, long, default_value = "pdf")]
        format: String,
        /// The file to write to
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Initializes a new document.
    Init {
        /// Directory to initialize the document in.
        path: Option<PathBuf>,
    },
}

pub fn run() -> Result<()> {
    let app = Djoc::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(app.quiet)
        .verbosity(4)
        .init()?;

    match app.command {
        Command::Compile {
            path,
            format,
            output,
        } => compile::compile(path, format, output)?,
        Command::Init { path } => init::init(path)?,
        Command::Build => build::build()?,
    }

    Ok(())
}

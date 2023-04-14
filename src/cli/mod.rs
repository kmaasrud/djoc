mod build;
mod clean;
mod compile;
mod init;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::Logger;

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
    /// Builds the document(s) in the current directory.
    Build,

    /// Removes the build directory.
    Clean,

    /// Compiles a single-file document.
    Compile {
        /// The target source file (omit for stdin)
        path: Option<PathBuf>,
        /// The type of output you want to build to.
        #[arg(short, long, default_value = "pdf")]
        format: String,
        /// The file to write to
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// If set, sections will be numbered.
        #[arg(long, default_value = "false")]
        number_sections: bool,
    },

    /// Initializes a new document project.
    Init {
        /// Directory to initialize the document in.
        path: Option<PathBuf>,
    },
}

pub fn run() -> Result<()> {
    let app = Djoc::parse();

    let logger = match (app.quiet, app.debug) {
        (true, _) => Logger::new(LevelFilter::Error),
        (_, true) => Logger::new(LevelFilter::Off),
        _ => Logger::new(LevelFilter::Info),
    };

    log::set_max_level(logger.filter);
    log::set_boxed_logger(Box::new(logger))?;

    match app.command {
        Command::Build => build::build()?,
        Command::Clean => clean::clean()?,
        Command::Compile {
            path,
            format,
            output,
            number_sections,
        } => compile::compile(path, format, output, number_sections)?,
        Command::Init { path } => init::init(path)?,
    }

    Ok(())
}

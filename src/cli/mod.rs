mod build;
mod clean;
mod compile;
mod init;

use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use log::LevelFilter;

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

    Clean,

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

pub fn run() -> Result<(), Box<dyn Error>> {
    let app = Djoc::parse();

    let logger = match (app.quiet, app.debug) {
        (true, _) => djoc::log::Logger::new(LevelFilter::Error),
        (_, true) => djoc::log::Logger::new(LevelFilter::Off),
        _ => djoc::log::Logger::new(LevelFilter::Info),
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
        } => compile::compile(path, format, output)?,
        Command::Init { path } => init::init(path)?,
    }

    Ok(())
}

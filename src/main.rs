use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;
use structopt::StructOpt;

mod cmd;

#[derive(StructOpt)]
#[structopt(
    name = "MDoc",
    about = "Modern PDF creation through Markdown and LaTeX",
    author = "Knut Magnus Aasrud"
)]
struct App {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(
        short = "q",
        long = "quiet",
        help = "Make MDoc quiet. Only errors will get reported."
    )]
    quiet: bool,

    #[structopt(
        short = "d",
        long = "debug",
        help = "Make MDoc's output verbose. Used for debugging."
    )]
    debug: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Builds a file or document")]
    Build {
        #[structopt(about = "File to build into PDF (optional)", parse(from_os_str))]
        file: Option<PathBuf>,
    },

    #[structopt(about = "Initializes a new document")]
    Init,

    #[structopt(about = "Lists the document structure")]
    List,
}

fn run() -> Result<()> {
    let app = App::from_args_safe()?;

    match (app.debug, app.quiet) {
        (false, false) => mdoc::log::set_max_level(3),
        (true, _) => mdoc::log::set_max_level(4),
        (false, true) => mdoc::log::set_max_level(1),
    }

    match app.command {
        Command::Build { file } => {
            cmd::build(file)?;
        }

        Command::Init => {
            mdoc::info!("Initializing");
        }

        Command::List => {
            mdoc::info!("Listing");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        mdoc::error!("{}{}", e, mdoc::log::format_chain(e.chain()));
        std::process::exit(1);
    }
}

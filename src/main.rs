use anyhow::Result;
use std::path::PathBuf;
use structopt::{StructOpt, clap};

mod cmd;

#[derive(StructOpt)]
#[structopt(
    name = "MDoc",
    author = "Knut Magnus Aasrud",
)]
/// LaTeX for the modern world.
struct App {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(
        short = "q",
        long = "quiet",
    )]
    /// Make MDoc quiet. Only errors will get reported.
    quiet: bool,

    #[structopt(
        short = "d",
        long = "debug",
    )]
    /// Make MDoc's output verbose. Used for debugging.
    debug: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Builds a file or document")]
    Build {
        #[structopt(parse(from_os_str))]
        /// File to build into PDF (optional).
        path: Option<PathBuf>,
    },

    #[structopt(about = "Initializes a new document")]
    Init {
        #[structopt(parse(from_os_str))]
        /// Directory to initialize the document in.
        path: Option<PathBuf>,
    },

    #[structopt()]
    /// Lists the document structure.
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
        Command::Build { path } => {
            cmd::build(path)?;
        }

        Command::Init { path } => {
            cmd::init(path)?;
        }

        Command::List => {
            mdoc::info!("Listing");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        match e.downcast_ref::<clap::Error>() {
            Some(e) if e.kind == clap::ErrorKind::HelpDisplayed => {
                println!("{}", e)
            },
            Some(e) => {
                mdoc::error!("{}", e.to_string().trim_start_matches("\u{1b}[1;31merror:\u{1b}[0m "));
                std::process::exit(1);
            }
            _ => {
                mdoc::error!("{}{}", e, mdoc::log::format_chain(e.chain()));
                std::process::exit(1);
            }
        }
    }
}

use anyhow::Result;
use std::path::PathBuf;
use colored::Colorize;
use structopt::StructOpt;

mod cmd;

#[derive(StructOpt)]
#[structopt(
    name = "doctor",
    about = "Modern PDF creation through Markdown and LaTeX",
    author = "Knut Magnus Aasrud",
)]
struct App {
    #[structopt(subcommand)]
    command: Command, 
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Builds a file or document")]
    Build {
        #[structopt(about = "File to build into PDF (optional)", parse(from_os_str))]
        file: Option<PathBuf>
    },

    #[structopt(about = "Initializes a new document")]
    Init,

    #[structopt(about = "Lists the document structure")]
    List,
}

fn run() -> Result<()> {
    let app = App::from_args();

    match app.command {
        Command::Build{ file } => {
            cmd::build(file)?;
        }

        Command::Init => {
            println!("Initializing");
        }

        Command::List => {
            println!("Listing");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("  {} {}", "E".red(), e);
        let chain = e.chain().skip(1);
        if chain.len() > 0 {
            eprintln!("  {}", "│".bright_black());
            eprintln!("  {} Caused by:", "│".bright_black());
            chain.for_each(|cause| {
                eprintln!("  {}     {}", "│".bright_black(), cause);
            });
            eprintln!("  {}", "╵".bright_black());
        }
        std::process::exit(1);
    }
}

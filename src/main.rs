use structopt::StructOpt;

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
    Build,

    #[structopt(about = "Initializes a new document")]
    Init,

    #[structopt(about = "Lists the document structure")]
    List,
}

fn main() {
    let app = App::from_args();

    match app.command {
        Command::Build => {
            println!("Building");
        }

        Command::Init => {
            println!("Initializing");
        }

        Command::List => {
            println!("Listing");
        }
    }
}

use doctor::Document;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
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
    Build {
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>
    },

    #[structopt(about = "Initializes a new document")]
    Init,

    #[structopt(about = "Lists the document structure")]
    List,
}

fn main() {
    let app = App::from_args();

    match app.command {
        Command::Build{ file } => {
            let content = match file {
                Some(path) => {
                    let mut file = String::new();
                    BufReader::new(File::open(path).unwrap()).read_to_string(&mut file).ok();
                    file
                },
                None => "Didn't find file".to_owned(),
            };

            let doc = Document::from_str(content);
            let pdf_data = tectonic::latex_to_pdf(doc.latex()).expect("PDF creation failed");
            doctor::utils::write_file(&Path::new("main.pdf"), &pdf_data);
        }

        Command::Init => {
            println!("Initializing");
        }

        Command::List => {
            println!("Listing");
        }
    }
}

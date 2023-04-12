mod cli;
mod log;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}

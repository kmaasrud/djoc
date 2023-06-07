#[cfg(feature = "cli")]
mod cli;

use std::io::{self, Stderr, Write};

#[cfg(feature = "cli")]
use anyhow::Result;
use log::{LevelFilter, Log};

pub struct Logger {
    writer: Stderr,
    pub filter: LevelFilter,
}

impl Logger {
    pub fn new(filter: LevelFilter) -> Self {
        Self {
            writer: io::stderr(),
            filter,
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, record: &log::Record) {
        let mut w = io::LineWriter::new(self.writer.lock());
        write!(w, "{} ", record.level()).ok();
        writeln!(w, "{}", record.args()).ok();
    }

    fn flush(&self) {
        let mut w = self.writer.lock();
        w.flush().ok();
    }
}

#[cfg(feature = "cli")]
fn main() -> Result<()> {
    cli::run()
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Build with the `cli` feature to enable the CLI")
}

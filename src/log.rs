use std::io::{self, Stderr, Write};

use anyhow::Error;
use log::{error, info, warn, LevelFilter, Log};

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

pub(crate) struct LoggingStatusBackend;

impl tectonic::status::StatusBackend for LoggingStatusBackend {
    fn report(
        &mut self,
        kind: tectonic::status::MessageKind,
        args: std::fmt::Arguments,
        err: Option<&Error>,
    ) {
        match kind {
            tectonic::status::MessageKind::Error => {
                error!("{}{}", args, err.map(|e| e.to_string()).unwrap_or_default());
            }
            tectonic::status::MessageKind::Warning => warn!("{}", args),
            tectonic::status::MessageKind::Note => info!("{}", args),
        }
    }

    fn dump_error_logs(&mut self, output: &[u8]) {
        error!("{}", String::from_utf8_lossy(output));
    }
}

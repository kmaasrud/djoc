use anyhow::Error;
use log::{error, info, warn};

pub struct LoggingStatusBackend;

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

use anyhow::Error;
use log::{error, info, warn};

pub(crate) struct DjocTectonicStatusBackend {
    pub tidy_logs: bool,
}

impl tectonic::status::StatusBackend for DjocTectonicStatusBackend {
    fn report(
        &mut self,
        kind: tectonic::status::MessageKind,
        args: std::fmt::Arguments,
        err: Option<&Error>,
    ) {
        match kind {
            tectonic::status::MessageKind::Error => {
                let mut msg = args.to_string();

                if self.tidy_logs {
                    msg = msg
                        .trim_end_matches("See the LaTeX manual or LaTeX Companion for explanation.\nType  H <return>  for immediate help")
                        .trim_start_matches('!')
                        .to_owned();
                }

                error!(
                    "{}{}",
                    msg.trim(),
                    err.map(|e| e.to_string()).unwrap_or_default()
                );
            }
            tectonic::status::MessageKind::Warning => {
                let msg = args.to_string();
                // Remove all underfull/overfull vbox/hbox messages.
                if !self.tidy_logs || !(
                    msg.contains(r"Underfull \hbox")
                    || msg.contains(r"Overfull \hbox")
                    || msg.contains(r"Underfull \vbox")
                    || msg.contains(r"Overfull \vbox")
                    || msg.contains(r"warnings were issued by the TeX engine; use --print and/or --keep-logs for details.")
                ) {
                    warn!("{}", args);
                }
            }
            tectonic::status::MessageKind::Note => info!("{}", args),
        }
    }

    fn dump_error_logs(&mut self, output: &[u8]) {
        error!("{}", String::from_utf8_lossy(output));
    }
}

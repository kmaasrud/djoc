use anyhow::{anyhow, Error};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The maximum logging level, stored statically in the memory as an atomic usize.
static MAX_LOG_LEVEL: AtomicUsize = AtomicUsize::new(0);

/// Set the maximum logging level. Accepts a usize.
///
/// The loggin levels use the following nomenclature:
///
/// - 0: Error
/// - 1: Success
/// - 2: Warning
/// - 3: Info and success
/// - >4: Debug
#[inline]
pub fn set_max_level(level: usize) {
    MAX_LOG_LEVEL.store(level, Ordering::SeqCst);
}

#[inline(always)]
pub fn max_level() -> usize {
    MAX_LOG_LEVEL.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        let text = format!($($arg)*);
        let mut lines = text.lines();
        if let Some(line) = lines.next() {
            eprintln!("\x1B[31mE\x1B[0m {}", line);
            for line in lines {
                eprintln!("  {}", line);
            }
        }
    })
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => ({
        if $crate::log::max_level() >= 1 {
            let text = format!($($arg)*);
            let mut lines = text.lines();
            if let Some(line) = lines.next() {
                println!("\x1B[32m✓\x1B[0m {}", line);
                for line in lines {
                    println!("  {}", line);
                }
            }
        }
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        if $crate::log::max_level() >= 2 {
            let text = format!($($arg)*);
            let mut lines = text.lines();
            if let Some(line) = lines.next() {
                eprintln!("\x1B[33mW\x1B[0m {}", line);
                for line in lines {
                    eprintln!("  {}", line);
                }
            }
        }
    })
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        if $crate::log::max_level() >= 3 {
            let text = format!($($arg)*);
            let mut lines = text.lines();
            if let Some(line) = lines.next() {
                println!("  \x1B[90m{}\x1B[0m", line);
                for line in lines {
                    println!("  \x1B[90m{}\x1B[0m", line);
                }
            }
        }
    })
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        if $crate::log::max_level() >= 4 {
            let text = format!($($arg)*);
            let mut lines = text.lines();
            if let Some(line) = lines.next() {
                println!("D {}", line);
                for line in lines {
                    println!("  {}", line);
                }
            }
        }
    })
}

pub fn format_chain(chain: anyhow::Chain) -> String {
    let mut out = String::new();

    fn begin_line_with(i: usize) -> String {
        if i == 0 { "- " } else { "  " }.to_owned()
    }

    for link in chain.skip(1) {
        for (i, line) in link.to_string().lines().enumerate() {
            out.push('\n');
            out.push_str(&begin_line_with(i));
            out.push_str(line);
        }
    }

    out
}

pub struct MdocTectonicStatusBackend;

impl tectonic::status::StatusBackend for MdocTectonicStatusBackend {
    fn report(
        &mut self,
        kind: tectonic::status::MessageKind,
        args: std::fmt::Arguments,
        err: Option<&Error>,
    ) {
        match kind {
            tectonic::status::MessageKind::Error => {
                let msg = args.to_string();

                error!("{}{}",
                    msg
                        .trim_end_matches("See the LaTeX manual or LaTeX Companion for explanation.\nType  H <return>  for immediate help")
                        .trim_start_matches('!')
                        .trim(),
                    format_chain(err.unwrap_or(&anyhow!("")).chain())
                );
            }
            tectonic::status::MessageKind::Warning => {
                let msg = args.to_string();
                // Remove all underfull/overfull hbox messages. TODO: Make this optional
                if !(
                    msg.contains(r"Underfull \hbox")
                    || msg.contains(r"Overfull \hbox")
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

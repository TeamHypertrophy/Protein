/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use std::path::Path;

use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{filter::EnvFilter, fmt, fmt::format::FmtSpan, prelude::*};

use crate::constants::*;

pub fn setup() -> Result<(WorkerGuard, ()), Box<dyn std::error::Error>> {
    // Build Path for Logs Directory
    let path = Path::new(LOG_PATH);

    // This Creates a Log File that Rotates Daily
    let appender: RollingFileAppender = match RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix(LOG_FILE)
        .build(path)
    {
        Ok(appender) => appender,
        Err(_) => panic!("[!] Error Building Rolling Log File"),
    };

    // This Creates a Non-Blocking Appender That Writes to the Log File (So I/O Does
    // Not Interrupt the Main Thread)
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);

    // Get Filter From Environment Variables or Use Default
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| TERMINAL_FILTER.into());

    // Separate Layers for File and Terminal Logging
    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_thread_names(true)
        .with_span_events(FmtSpan::CLOSE);

    let terminal_layer = fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(filter);

    // Register Layers
    tracing_subscriber::registry()
        .with(file_layer)
        .with(terminal_layer)
        .init();

    Ok((guard, ()))
}

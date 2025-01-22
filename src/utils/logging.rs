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
use tracing_subscriber::{fmt, fmt::format::FmtSpan, prelude::*};

pub fn setup_logging() -> Result<(WorkerGuard, ()), Box<dyn std::error::Error>> {
    // Build Path for Logs Directory
    let path = Path::new("./logs/");

    // This Creates a Log File that Rotates Daily
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("[Protein].log")
        .build(path)
        .expect("[!] Error Building Log Files");

    let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);

    // Seperate Layers for File and Terminal Logging
    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_span_events(FmtSpan::CLOSE);

    let terminal_layer = fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_span_events(FmtSpan::CLOSE);

    // Register Layers
    tracing_subscriber::registry()
        .with(file_layer)
        .with(terminal_layer)
        .init();

    Ok((guard, ()))
}

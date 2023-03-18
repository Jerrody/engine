#![feature(panic_info_message)]

mod macros;

pub use macros::*;
pub use tracing;

use std::{panic, path::Path};
use tracing::Level;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, Registry};

#[derive(Debug, Default)]
pub enum LogLevel {
    #[default]
    Dev,
    Editor,
    Shipping,
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Dev => Level::DEBUG,
            LogLevel::Editor => Level::INFO,
            LogLevel::Shipping => Level::ERROR,
        }
    }
}

pub struct Logging {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

impl Logging {
    pub fn new(file_directory: &Path, file_name: &str, log_level: LogLevel) -> Self {
        #[cfg(all(feature = "dev", any(feature = "editor", feature = "shipping")))]
        compile_error!("Cannot be enalbed `dev` feature and the other one.");

        #[cfg(all(feature = "editor", any(feature = "dev", feature = "shipping")))]
        compile_error!("Cannot be enabled `editor` feature anad the other one.");

        #[cfg(all(feature = "shipping", any(feature = "dev", feature = "editor")))]
        compile_error!("Cannot be enabled `snipping` feature and the other one.");

        if let Ok(log_file) = std::fs::File::options().write(true).open(std::format!(
            "{}/{file_name}",
            file_directory.as_os_str().to_string_lossy()
        )) {
            log_file.set_len(Default::default()).unwrap();
        }

        let file_appender = tracing_appender::rolling::never(file_directory, file_name);
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let offset_time = fmt::time::OffsetTime::new(
            time::UtcOffset::current_local_offset().unwrap(),
            time::macros::format_description!("[hour]:[minute]:[second]"),
        );

        let subscriber = Registry::default()
            .with(
                tracing_subscriber::EnvFilter::default()
                    .add_directive(Level::from(log_level).into()),
            )
            .with(
                fmt::Layer::new()
                    .pretty()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_thread_names(true)
                    .with_thread_ids(true)
                    .with_line_number(true)
                    .with_file(true)
                    .with_timer(offset_time.clone()),
            );

        #[cfg(feature = "dev")]
        let subscriber = subscriber.with(
            fmt::Layer::new()
                .pretty()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_thread_names(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_file(true)
                .with_timer(offset_time),
        );

        #[cfg(feature = "profiling")]
        let subscriber = subscriber.with(tracing_tracy::TracyLayer::new());

        tracing::subscriber::set_global_default(subscriber).expect("Failed to init logging.");

        panic::set_hook(Box::new(|args| {
            if let Some(message) = args.message() {
                let backtrace = backtrace::Backtrace::new();
                let location = args.location().unwrap();

                let message = std::format!(
                    "\n  Message:\n  {message}\n  Location: {}:{}:{}\n  Backtrace:\n{:#?}",
                    location.file(),
                    location.line(),
                    location.column(),
                    backtrace,
                );

                tracing::error!(message);
            }
        }));

        Self { _guard: guard }
    }
}

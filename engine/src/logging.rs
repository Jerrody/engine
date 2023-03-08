// TODO: Create different logs for the different features (`dev`, `editor`, `shipping`).

use std::panic;

use tracing::*;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, Registry};

pub struct Logging {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

impl Logging {
    const LOG_FILE_NAME: &str = "engine.log";
    const LOG_FILE_PATH: &str = "engine/";

    pub fn new() -> Self {
        if let Ok(log_file) = std::fs::File::options()
            .write(true)
            .open("engine/engine.log")
        {
            log_file.set_len(Default::default()).unwrap();
        }

        let file_appender =
            tracing_appender::rolling::never(Self::LOG_FILE_PATH, Self::LOG_FILE_NAME);
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let offset_time = fmt::time::OffsetTime::new(
            time::UtcOffset::current_local_offset().unwrap(),
            time::macros::format_description!("[hour]:[minute]:[second]"),
        );

        let subscriber = Registry::default()
            .with(tracing_subscriber::EnvFilter::default().add_directive(
                #[cfg(feature = "dev")]
                Level::DEBUG.into(),
                #[cfg(feature = "editor")]
                Level::INFO.into(),
                #[cfg(feature = "shipping")]
                Level::ERROR.into(),
            ))
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
                let location = args.location().unwrap();
                error!(
                    "{message}\n  Location: {}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                );
            }
        }));

        Self { _guard: guard }
    }
}

#![deny(unsafe_op_in_unsafe_fn)]

mod error;
mod renderer;

use error::*;
use logging::*;

pub struct Engine {
    renderer: renderer::Renderer,
    _logging: Logging,
}

impl Engine {
    const ENGINE_LOG_DIRECTORY: &str = "logs";
    const ENGINE_LOG_NAME: &str = "engine.log";

    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        let logging = Self::init_logging();

        info!("Initializing renderer.");
        let renderer = renderer::Renderer::new(window)?;

        Ok(Self {
            renderer,
            _logging: logging,
        })
    }

    fn init_logging() -> Logging {
        let log_level = match () {
            #[cfg(feature = "dev")]
            _ => LogLevel::Dev,

            #[cfg(feature = "editor")]
            _ => LogLevel::Editor,

            #[cfg(feature = "shipping")]
            _ => LogLevel::Shipping,
        };

        Logging::new(
            &std::path::PathBuf::from(Self::ENGINE_LOG_DIRECTORY),
            Self::ENGINE_LOG_NAME,
            log_level,
        )
    }
}

#![deny(unsafe_op_in_unsafe_fn)]

mod error;
mod renderer;

use error::*;
use logging::*;

pub struct Engine {
    renderer: renderer::Renderer,
    logging: Logging,
}

impl Engine {
    pub const ENGINE_LOG_DIRECTORY: &str = "logs";
    pub const ENGINE_LOG_NAME: &str = "engine.log";

    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        let logging = Self::init_logging();

        info!("Initializing renderer");
        let renderer = renderer::Renderer::new(window)?;

        Ok(Self { renderer, logging })
    }

    fn init_logging() -> Logging {
        let log_level = {
            #[cfg(feature = "dev")]
            {
                LogLevel::Dev
            }

            #[cfg(feature = "editor")]
            {
                LogLevel::Editor
            }

            #[cfg(feature = "shipping")]
            {
                LogLevel::Shipping
            }
        };

        Logging::new(
            &std::path::PathBuf::from(Self::ENGINE_LOG_DIRECTORY),
            Self::ENGINE_LOG_NAME,
            log_level,
        )
    }
}

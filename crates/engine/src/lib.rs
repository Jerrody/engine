#![deny(unsafe_op_in_unsafe_fn)]

mod renderer;

use logging::*;

pub struct Engine {
    renderer: renderer::Renderer,
    logging: logging::Logging,
}

impl Engine {
    pub const ENGINE_LOG_DIRECTORY: &str = "logs";
    pub const ENGINE_LOG_NAME: &str = "engine.log";

    pub fn new(window: &winit::window::Window) -> Self {
        let renderer = renderer::Renderer::new(window);

        let log_level = {
            #[cfg(feature = "dev")]
            {
                logging::LogLevel::Dev
            }

            #[cfg(feature = "editor")]
            {
                logging::LogLevel::Editor
            }

            #[cfg(feature = "shipping")]
            {
                logging::LogLevel::Shipping
            }
        };

        let logging = logging::Logging::new(
            &std::path::PathBuf::from(Self::ENGINE_LOG_DIRECTORY),
            Self::ENGINE_LOG_NAME,
            log_level,
        );

        Self { renderer, logging }
    }
}

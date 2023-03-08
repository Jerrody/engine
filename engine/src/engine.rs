mod logging;
mod renderer;

pub struct Engine {
    renderer: renderer::Renderer,
    logging: logging::Logging,
}

impl Engine {
    pub fn new(window: &winit::window::Window) -> Self {
        let renderer = renderer::Renderer::new(window);
        let logging = logging::Logging::new();

        tracing::debug!("LOL!?");

        Self { renderer, logging }
    }
}

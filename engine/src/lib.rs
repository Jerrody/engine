#![feature(panic_info_message)]
#![deny(unsafe_op_in_unsafe_fn)]

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

        Self { renderer, logging }
    }
}

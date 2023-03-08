mod context;
mod utils;

use context::Context;

pub struct Renderer {
    context: Context,
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> Self {
        let context = Context::new(window);

        Self { context }
    }
}

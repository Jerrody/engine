mod context;
mod utils;

use logging::*;

use crate::error::EngineError;

pub struct Renderer {
    context: context::Context,
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> Result<Self, EngineError> {
        debug!("Initializing Vulkan");
        let context = context::Context::new(window)?;

        Ok(Self { context })
    }
}

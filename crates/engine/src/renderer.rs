mod context;
mod utils;

use logging::*;

use crate::{debug, error::EngineResult};

pub struct Renderer {
    context: context::Context,
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Initializing Vulkan.");
        let context = context::Context::new(window)?;

        Ok(Self { context })
    }
}

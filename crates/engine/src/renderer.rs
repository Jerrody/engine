mod context;
mod utils;

use logging::*;

use crate::{debug, error::EngineResult};

pub struct Renderer<'a> {
    context: context::Context<'a>,
}

impl Renderer<'_> {
    #[inline]
    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Initializing Vulkan.");
        let context = context::Context::new(window)?;

        Ok(Self { context })
    }
}

use std::mem::ManuallyDrop;

mod device;
mod instance;

use instance::Instance;
use logging::*;

use crate::error::EngineResult;

pub struct Context {
    entry: ManuallyDrop<ash::Entry>,
    instance: Instance,
}

impl Context {
    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Loading Vulkan lib");
        let entry = unsafe { ash::Entry::load()? };
        debug!("Creating an Instance of Vulkan Application");
        let instance = Instance::new(&entry, window)?;

        Ok(Self {
            entry: ManuallyDrop::new(entry),
            instance,
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.entry);
        }
    }
}

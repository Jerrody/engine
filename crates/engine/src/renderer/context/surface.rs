use logging::*;

use crate::error::EngineResult;

use ash::extensions::khr::Surface;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct SurfaceHandle {
    pub surface_loader: Surface,
    pub surface: ash::vk::SurfaceKHR,
}

impl SurfaceHandle {
    pub fn new(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &winit::window::Window,
    ) -> EngineResult<Self> {
        debug!("Creating Surface of Vulkan Instance.");

        let surface_loader = Surface::new(entry, instance);
        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
        }?;

        Ok(Self {
            surface_loader,
            surface,
        })
    }
}

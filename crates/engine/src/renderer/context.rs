mod device;
mod instance;
mod surface;

use logging::*;

use crate::{
    error::EngineResult,
    renderer::{context::device::DeviceHandle, utils::*},
};

use instance::InstanceHandle;
use surface::SurfaceHandle;

use std::mem::ManuallyDrop;

pub struct Context {
    entry: ManuallyDrop<ash::Entry>,
    instance_handle: InstanceHandle,
    surface_handle: SurfaceHandle,
    device_handle: DeviceHandle,
}

impl Context {
    #[cfg(feature = "dev")]
    const VALIDATION_LAYER_NAME: *const std::os::raw::c_char = cstr!("VK_LAYER_KHRONOS_validation");

    const REQUIRED_LAYERS: &[*const std::ffi::c_char] = &[
        #[cfg(feature = "dev")]
        Self::VALIDATION_LAYER_NAME,
    ];

    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Loading Vulkan lib.");
        let entry = unsafe { ash::Entry::load()? };

        let instance_handle = InstanceHandle::new(&entry, window)?;

        let surface_handle =
            surface::SurfaceHandle::new(&entry, &instance_handle.instance, window)?;

        debug!("Creating Device.");
        let device_handle = DeviceHandle::new(&instance_handle.instance, &surface_handle)?;

        Ok(Self {
            entry: ManuallyDrop::new(entry),
            instance_handle,
            surface_handle,
            device_handle,
        })
    }

    #[cfg(feature = "dev")]
    fn does_support_layers(
        required_layers: &[*const std::ffi::c_char],
        available_layers: &[ash::vk::LayerProperties],
        unsupported_text_header: &str,
    ) -> bool {
        let mut unsupported_layers = Vec::new();
        required_layers.iter().for_each(|&required_layer| {
            let required_layer_name = to_cstr(required_layer);
            let is_supported = available_layers.iter().any(|available_layer| {
                let available_layer_name = to_cstr(available_layer.layer_name.as_ptr());

                required_layer_name == available_layer_name
            });

            if !is_supported {
                unsupported_layers.push(required_layer_name.to_str().unwrap().to_owned());
            }
        });

        if !unsupported_layers.is_empty() {
            let unsupported_layers_text = format!(
                "{unsupported_text_header}:\n{}",
                unsupported_layers
                    .iter()
                    .map(|unsupported_layer_name| format!(
                        "{TAB_IN_SPACES}- {}\n",
                        unsupported_layer_name
                    ))
                    .collect::<String>()
            );

            debug!(unsupported_layers_text);

            return false;
        }

        true
    }

    fn does_support_extensions(
        required_extensions: &[*const std::ffi::c_char],
        available_extensions: &[ash::vk::ExtensionProperties],
        unsupported_text_header: &str,
    ) -> bool {
        let mut unsupported_extensions = Vec::new();
        required_extensions.iter().for_each(|&required_extension| {
            let required_extension_name = to_cstr(required_extension);
            let is_supported = available_extensions.iter().any(|available_extension| {
                let available_extension_name = to_cstr(available_extension.extension_name.as_ptr());

                required_extension_name == available_extension_name
            });

            if !is_supported {
                unsupported_extensions.push(required_extension_name.to_str().unwrap().to_owned());
            }
        });

        if !unsupported_extensions.is_empty() {
            let unsupported_extensions_text = format!(
                "{unsupported_text_header}:\n{}",
                unsupported_extensions
                    .iter()
                    .map(|unsupported_extension_name| format!(
                        "{TAB_IN_SPACES}- {}\n",
                        unsupported_extension_name
                    ))
                    .collect::<String>()
            );

            debug!(unsupported_extensions_text);

            return false;
        }

        true
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.device_handle.device.device_wait_idle().unwrap();

            self.device_handle.device.destroy_device(None);
            self.instance_handle.instance.destroy_instance(None);
            self.surface_handle
                .surface_loader
                .destroy_surface(self.surface_handle.surface, None);

            ManuallyDrop::drop(&mut self.entry);
        }
    }
}

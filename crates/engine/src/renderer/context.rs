#[cfg(feature = "dev")]
mod debug_messenger;
mod device;
mod instance;
mod shader;
mod surface;
mod sync;
mod resource_manager;

use logging::*;

use crate::{
    error::EngineResult,
    renderer::{context::device::DeviceManager, utils::*},
};

use instance::InstanceHandle;
use surface::SurfaceManager;

use std::mem::ManuallyDrop;

pub struct Context<'a> {
    entry: ManuallyDrop<ash::Entry>,
    instance_manager: InstanceHandle,
    debug_messenger_manager: debug_messenger::DebugMessengerManager,
    surface_manager: SurfaceManager,
    device_manager: DeviceManager,
    shader_manager: shader::ShaderManager<'a>,
    sync_manager: sync::SynchronizationPrimitivesManager,
}

impl Context<'_> {
    #[cfg(feature = "dev")]
    const VALIDATION_LAYER_NAME: *const std::os::raw::c_char = cstr!("VK_LAYER_KHRONOS_validation");

    const REQUIRED_LAYERS: &[*const std::ffi::c_char] = &[
        #[cfg(feature = "dev")]
        Self::VALIDATION_LAYER_NAME,
    ];

    pub fn new(window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Loading Vulkan lib.");
        let entry = unsafe { ash::Entry::load()? };

        let instance_manager = InstanceHandle::new(&entry, window)?;
        let debug_messenger_manager =
            debug_messenger::DebugMessengerManager::new(&entry, &instance_manager.instance)?;

        let surface_manager =
            surface::SurfaceManager::new(&entry, &instance_manager.instance, window)?;

        debug!("Creating Device.");
        let device_manager = DeviceManager::new(&instance_manager.instance, &surface_manager)?;
        let sync_manager = sync::SynchronizationPrimitivesManager::new(&device_manager)?;

        let shader_manager = shader::ShaderManager::new();

        Ok(Self {
            entry: ManuallyDrop::new(entry),
            instance_manager,
            debug_messenger_manager,
            surface_manager,
            device_manager,
            shader_manager,
            sync_manager,
        })
    }

    #[cfg(feature = "dev")]
    fn does_support_layers(
        required_layers: &[*const std::ffi::c_char],
        available_layers: &[ash::vk::LayerProperties],
        unsupported_text_header: &str,
    ) -> bool {
        let unsupported_layers = required_layers
            .iter()
            .filter_map(|&required_layer| {
                let required_layer_name = to_cstr(required_layer);
                if !available_layers.iter().any(|available_layer| {
                    let available_layer_name = to_cstr(available_layer.layer_name.as_ptr());

                    required_layer_name == available_layer_name
                }) {
                    Some(required_layer_name.to_str().unwrap().to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !unsupported_layers.is_empty() {
            let unsupported_layers_text = format!(
                "{}:\n{}",
                unsupported_text_header,
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
        let unsupported_extensions = required_extensions
            .iter()
            .filter_map(|&required_extension| {
                let required_extension_name = to_cstr(required_extension);
                if !available_extensions.iter().any(|available_extension| {
                    let available_extension_name =
                        to_cstr(available_extension.extension_name.as_ptr());

                    required_extension_name == available_extension_name
                }) {
                    Some(required_extension_name.to_str().unwrap().to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !unsupported_extensions.is_empty() {
            let unsupported_extensions_text = format!(
                "{}:\n{}",
                unsupported_text_header,
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

impl Drop for Context<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device_manager.wait_for_idle().unwrap();
            self.shader_manager.unload_shaders(&self.device_manager);
            self.sync_manager.destroy_resources(&self.device_manager);
            self.device_manager.destroy_device();

            #[cfg(feature = "dev")]
            self.debug_messenger_manager
                .debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_messenger_manager.debug_utils, None);

            self.instance_manager.instance.destroy_instance(None);
            self.surface_manager
                .surface_loader
                .destroy_surface(self.surface_manager.surface, None);

            ManuallyDrop::drop(&mut self.entry);
        }
    }
}

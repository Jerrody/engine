use std::{ffi::CStr, os::raw::c_char};

use ash::vk;
use logging::*;
use raw_window_handle::HasRawDisplayHandle;

use crate::{cstr, EngineError, EngineResult};

pub struct Instance {
    pub instance: ash::Instance,
}

impl Instance {
    const ENGINE_NAME: *const c_char = cstr!("No Engine");
    const APPLICATION_NAME: *const c_char = cstr!("Triangle");

    #[cfg(feature = "dev")]
    const VALIDATION_LAYER_NAME: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");

    pub fn new(entry: &ash::Entry, window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Creating Application info");
        let application_info = vk::ApplicationInfo {
            p_application_name: Self::APPLICATION_NAME,
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: Self::ENGINE_NAME,
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        };

        debug!("Checking for availability layers");
        let available_layers = entry.enumerate_instance_layer_properties()?;
        let layers_names = Self::get_layer_names(&available_layers)?;

        debug!("Checking for avalability extensions");
        let available_extensions =
            entry.enumerate_instance_extension_properties(Default::default())?;
        let extension_names = Self::get_extension_names(window, &available_extensions)?;

        debug!("Creating an Instance");
        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_layer_names(&layers_names)
            .enabled_extension_names(&extension_names);
        let instance = unsafe { entry.create_instance(&instance_info, None)? };

        #[cfg(feature = "dev")]
        {
            let mut instance_info = String::from("Created an Instance.\n\n");

            instance_info.push_str("    With Layers:\n");
            layers_names.iter().for_each(|layer_name| {
                instance_info.push_str(&std::format!("    - {}\n", unsafe {
                    CStr::from_ptr(*layer_name).to_str().unwrap()
                }))
            });
            instance_info.push_str("\n");
            instance_info.push_str("    With Extensions:\n");
            extension_names.iter().for_each(|extension_name| {
                instance_info.push_str(&std::format!("    - {}\n", unsafe {
                    CStr::from_ptr(*extension_name).to_str().unwrap()
                }));
            });

            debug!(instance_info);
        }

        Ok(Self { instance })
    }

    fn get_layer_names(available_layers: &[vk::LayerProperties]) -> EngineResult<Vec<*const i8>> {
        #[cfg(feature = "dev")]
        {
            let mut missing_layer_names = vec![];
            let required_layer_raw_names = vec![Self::VALIDATION_LAYER_NAME];

            for required_layer_raw_name in &required_layer_raw_names {
                let required_layer_name = unsafe { CStr::from_ptr(*required_layer_raw_name) };

                if !available_layers.iter().any(|available_layer| unsafe {
                    CStr::from_ptr(available_layer.layer_name.as_ptr()) == required_layer_name
                }) {
                    missing_layer_names.push(required_layer_name.to_str().unwrap().to_owned());
                }
            }

            if !missing_layer_names.is_empty() {
                let missing_layers_text = format!(
                    "Failed to find required Layers:\n{}",
                    missing_layer_names
                        .iter()
                        .map(|missing_layer_name| format!("    - {}\n", missing_layer_name))
                        .collect::<String>()
                );
                error!(missing_layers_text);

                return Err(EngineError::InstanceCreationFailed(
                    "Not presented required Layers for the Instance of Vulkan.".to_owned(),
                ));
            }

            Ok(required_layer_raw_names.to_owned())
        }

        #[cfg(not(feature = "dev"))]
        Ok(Default::default())
    }

    fn get_extension_names(
        window: &winit::window::Window,
        available_extensions: &[vk::ExtensionProperties],
    ) -> EngineResult<Vec<*const i8>> {
        let required_extension_raw_names =
            ash_window::enumerate_required_extensions(window.raw_display_handle())?;
        let required_extension_names: Vec<_> = required_extension_raw_names
            .iter()
            .map(|required_extension_name| unsafe { CStr::from_ptr(*required_extension_name) })
            .collect();

        let mut missing_extension_names = vec![];

        for required_extension_name in required_extension_names {
            if !available_extensions
                .iter()
                .any(|available_extension| unsafe {
                    CStr::from_ptr(available_extension.extension_name.as_ptr())
                        == required_extension_name
                })
            {
                missing_extension_names.push(required_extension_name.to_str().unwrap().to_owned());
            }
        }

        if !missing_extension_names.is_empty() {
            let missing_extensions_text = format!(
                "Failed to find required Extensions:\n{}",
                missing_extension_names
                    .iter()
                    .map(|missing_extension_name| format!("    - {}\n", missing_extension_name))
                    .collect::<String>()
            );

            error!(missing_extensions_text);

            return Err(EngineError::InstanceCreationFailed(
                "Aren't presented required extensions for the Instance of Vulkan.".to_owned(),
            ))?;
        }

        Ok(required_extension_raw_names.to_owned())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

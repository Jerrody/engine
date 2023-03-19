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

        let application_name = unsafe { CStr::from_ptr(Self::APPLICATION_NAME) };
        let application_version = (0, 1, 0);

        let engine_name = unsafe { CStr::from_ptr(Self::ENGINE_NAME) };
        let engine_version = (0, 1, 0);

        let application_info = vk::ApplicationInfo {
            p_application_name: application_name.as_ptr(),
            application_version: vk::make_api_version(
                0,
                application_version.0,
                application_version.1,
                application_version.2,
            ),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(
                0,
                engine_version.0,
                engine_version.1,
                engine_version.0,
            ),
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        };

        debug!("Checking for availability layers");
        let available_layers = entry.enumerate_instance_layer_properties()?;
        let layer_names = Self::get_layer_names(&available_layers)?;

        debug!("Checking for avalability extensions");
        let available_extensions =
            entry.enumerate_instance_extension_properties(Default::default())?;
        let extension_names = Self::get_extension_names(window, &available_extensions)?;

        debug!("Creating an Instance");
        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_layer_names(&layer_names)
            .enabled_extension_names(&extension_names);
        let instance = unsafe { entry.create_instance(&instance_info, None)? };

        Self::print_info(
            &application_name.to_str().unwrap().to_owned(),
            application_version,
            &engine_name.to_str().unwrap().to_owned(),
            engine_version,
            &extension_names,
            &layer_names,
        );

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

    fn print_info(
        application_name: &str,
        application_version: (u32, u32, u32),
        engine_name: &str,
        engine_version: (u32, u32, u32),
        extension_names: &[*const i8],
        layer_names: &[*const i8],
    ) {
        let mut instance_info = String::from("Created an Instance.\n\n");

        instance_info.push_str("    Application Info:\n");

        let application_name = std::format!("    - Application Name: {application_name}\n");
        instance_info.push_str(&application_name);

        let application_version = std::format!(
            "    - Application Version: {}.{}.{}\n",
            application_version.0,
            application_version.1,
            application_version.2
        );
        instance_info.push_str(&application_version);

        let engine_name = std::format!("    - Engine Name: {engine_name}\n");
        instance_info.push_str(&engine_name);

        let engine_version = std::format!(
            "    - Engine Version: {}.{}.{}\n",
            engine_version.0,
            engine_version.1,
            engine_version.2
        );
        instance_info.push_str(&engine_version);

        let vulkan_version = std::format!("    - Vulkan API: 1.3\n\n");
        instance_info.push_str(&vulkan_version);

        #[cfg(feature = "dev")]
        {
            instance_info.push_str("    With Layers:\n");
            layer_names.iter().for_each(|layer_name| {
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
        }

        debug!(instance_info);
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

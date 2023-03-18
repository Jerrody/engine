use std::{ffi::CStr, os::raw::c_char};

use ash::vk;
use logging::*;
use raw_window_handle::HasRawDisplayHandle;

use crate::{cstr, error::EngineError};

pub struct Instance {
    pub instance: ash::Instance,
}

impl Instance {
    const ENGINE_NAME: *const c_char = cstr!("No Engine");
    const APPLICATION_NAME: *const c_char = cstr!("Triangle");

    #[cfg(feature = "dev")]
    const VALIDATION_LAYER_NAME: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");

    pub fn new(entry: &ash::Entry, window: &winit::window::Window) -> Result<Self, EngineError> {
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

    // TODO: Enumerate what layers unable to find.
    fn get_layer_names(
        available_layers: &[vk::LayerProperties],
    ) -> Result<Vec<*const i8>, EngineError> {
        #[cfg(not(feature = "dev"))]
        {
            Ok(Default::default())
        }

        #[cfg(feature = "dev")]
        {
            let mut unavailable_layer_names = vec![];

            let required_layer_raw_names = vec![Self::VALIDATION_LAYER_NAME];
            let required_layer_names: Vec<_> = required_layer_raw_names
                .iter()
                .map(|required_layer_name| unsafe { CStr::from_ptr(*required_layer_name) })
                .collect();

            let are_presented = required_layer_names.into_iter().all(|required_layer_name| {
                let is_found_layer = available_layers.iter().any(|available_layer_property| {
                    let available_layer_name =
                        unsafe { CStr::from_ptr(available_layer_property.layer_name.as_ptr()) };

                    required_layer_name == available_layer_name
                });

                if !is_found_layer {
                    unavailable_layer_names.push(required_layer_name.to_str().unwrap().to_owned());
                }

                is_found_layer
            });

            if unavailable_layer_names.len() > Default::default() {
                let mut unavailable_layers_text = String::from("Failed to find required Layers:\n");

                unavailable_layer_names
                    .iter()
                    .for_each(|unavailable_layer_name| {
                        unavailable_layers_text
                            .push_str(&std::format!("    - {unavailable_layer_name}\n"))
                    });

                error!(unavailable_layers_text);
            }

            match are_presented {
                true => Ok(required_layer_raw_names.to_owned()),
                false => Err("Not presented required Layers for the Instance of Vulkan.")?,
            }
        }
    }

    // TODO: Enumerate what extensions unable to find.
    fn get_extension_names(
        window: &winit::window::Window,
        available_extensions: &[vk::ExtensionProperties],
    ) -> Result<Vec<*const i8>, EngineError> {
        let required_extension_raw_names =
            ash_window::enumerate_required_extensions(window.raw_display_handle())?;
        let required_extension_names: Vec<_> = required_extension_raw_names
            .iter()
            .map(|required_extension_name| unsafe { CStr::from_ptr(*required_extension_name) })
            .collect();

        #[cfg(feature = "dev")]
        let mut unavailable_extension_names = vec![];

        let are_presented = required_extension_names
            .into_iter()
            .all(|required_extension_name| {
                let is_found_extension =
                    available_extensions
                        .iter()
                        .any(|available_extension_property| {
                            let available_extension_name = unsafe {
                                CStr::from_ptr(available_extension_property.extension_name.as_ptr())
                            };

                            available_extension_name == required_extension_name
                        });

                #[cfg(feature = "dev")]
                {
                    if !is_found_extension {
                        unavailable_extension_names
                            .push(required_extension_name.to_str().unwrap().to_owned());
                    }
                }

                is_found_extension
            });

        #[cfg(feature = "dev")]
        {
            if unavailable_extension_names.len() > Default::default() {
                let mut unavailable_extensions_text =
                    String::from("Failed to find required Extensions:\n");

                unavailable_extension_names
                    .iter()
                    .for_each(|unavailable_extenion_name| {
                        unavailable_extensions_text
                            .push_str(&std::format!("    - {unavailable_extenion_name}\n"))
                    });

                error!(unavailable_extensions_text);
            }
        }

        match are_presented {
            true => Ok(required_extension_raw_names.to_owned()),
            false => Err("Aren't presented required extensions for the Instance of Vulkan.")?,
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

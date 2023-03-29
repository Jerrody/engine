use logging::*;

use crate::{cstr, renderer::utils::to_cstr, EngineError, EngineResult};

use std::os::raw::c_char;

use ash::vk;
use raw_window_handle::HasRawDisplayHandle;

pub struct InstanceHandle {
    pub instance: ash::Instance,
}

impl InstanceHandle {
    const ENGINE_NAME: *const c_char = cstr!("No Engine");
    const APPLICATION_NAME: *const c_char = cstr!("Preparation");

    const ENGINE_VERSION: (u32, u32, u32) = (0, 1, 0);
    const APPLICATION_VERSION: (u32, u32, u32) = (0, 1, 0);

    pub fn new(entry: &ash::Entry, window: &winit::window::Window) -> EngineResult<Self> {
        debug!("Creating Application Information.");

        let engine_name = to_cstr(Self::ENGINE_NAME);
        let engine_version = Self::ENGINE_VERSION;

        let application_name = to_cstr(Self::APPLICATION_NAME);
        let application_version = Self::APPLICATION_VERSION;

        let application_info = vk::ApplicationInfo {
            p_application_name: application_name.as_ptr(),
            application_version: vk::make_api_version(
                Default::default(),
                application_version.0,
                application_version.1,
                application_version.2,
            ),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(
                Default::default(),
                engine_version.0,
                engine_version.1,
                engine_version.0,
            ),
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        };

        debug!("Checking for Instance Layers requirement.");

        let required_layer_names = super::Context::REQUIRED_LAYERS;
        #[cfg(feature = "dev")]
        {
            let available_layers = entry.enumerate_instance_layer_properties()?;
            let does_support = super::Context::does_support_layers(
                required_layer_names,
                &available_layers,
                "Failed to find required Layers of Instance",
            );
            if !does_support {
                return Err(EngineError::InstanceCreationFailed(
                    "Not presented required Layers for the Instance of Vulkan.".to_owned(),
                ));
            }
        }

        debug!("Checking for Instance Extensions requirement.");

        let required_extension_names =
            ash_window::enumerate_required_extensions(window.raw_display_handle())?;
        let available_extensions =
            entry.enumerate_instance_extension_properties(Default::default())?;
        let does_support = super::Context::does_support_extensions(
            required_extension_names,
            &available_extensions,
            "Failed to find required Extensions of Instance",
        );
        if !does_support {
            return Err(EngineError::InstanceCreationFailed(
                "Aren't presented required extensions for the Instance of Vulkan.".to_owned(),
            ));
        }

        debug!("Creating an Instance.");

        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_layer_names(required_layer_names)
            .enabled_extension_names(required_extension_names);
        let instance = unsafe { entry.create_instance(&instance_info, None)? };

        let application_name = application_name.to_str().unwrap().to_owned();
        let engine_name = engine_name.to_str().unwrap().to_owned();
        Self::print_metadata(
            &application_name,
            application_version,
            &engine_name,
            engine_version,
            required_layer_names,
            required_extension_names,
        );

        Ok(Self { instance })
    }

    fn print_metadata(
        application_name: &str,
        application_version: (u32, u32, u32),
        engine_name: &str,
        engine_version: (u32, u32, u32),
        extension_names: &[*const i8],
        layer_names: &[*const i8],
    ) {
        let mut instance_info = String::from("Created an Instance.\n\n");

        instance_info.push_str(&std::format!("{TAB_IN_SPACES}Application Info:\n"));

        let application_name =
            std::format!("{TAB_IN_SPACES}- Application Name: {application_name}\n");
        instance_info.push_str(&application_name);

        let application_version = std::format!(
            "{TAB_IN_SPACES}- Application Version: {}.{}.{}\n",
            application_version.0,
            application_version.1,
            application_version.2
        );
        instance_info.push_str(&application_version);

        let engine_name = std::format!("{TAB_IN_SPACES}- Engine Name: {engine_name}\n");
        instance_info.push_str(&engine_name);

        let engine_version = std::format!(
            "{TAB_IN_SPACES}- Engine Version: {}.{}.{}\n",
            engine_version.0,
            engine_version.1,
            engine_version.2
        );
        instance_info.push_str(&engine_version);

        let vulkan_version = std::format!("{TAB_IN_SPACES}- Vulkan API: 1.3\n\n");
        instance_info.push_str(&vulkan_version);

        #[cfg(feature = "dev")]
        {
            instance_info.push_str(&std::format!("{TAB_IN_SPACES}With Layers:\n"));
            layer_names.iter().for_each(|layer_name| {
                instance_info.push_str(&std::format!(
                    "{TAB_IN_SPACES}- {}\n",
                    to_cstr(*layer_name).to_str().unwrap()
                ))
            });
        }

        instance_info.push_str(&std::format!("\n{TAB_IN_SPACES}With Extensions:\n"));
        extension_names.iter().for_each(|extension_name| {
            instance_info.push_str(&std::format!(
                "{TAB_IN_SPACES}- {}\n",
                to_cstr(*extension_name).to_str().unwrap()
            ));
        });

        debug!(instance_info);
    }
}

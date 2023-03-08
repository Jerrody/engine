use ash::vk;
use std::os::raw::c_char;

use crate::cstr;

pub struct Instance {
    pub instance: ash::Instance,
}

impl Instance {
    const ENGINE_NAME: *const c_char = cstr!("No Engine");
    const APPLICATION_NAME: *const c_char = cstr!("Triangle");

    const VALIDATION_LAYER_NAME: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");

    pub fn new(entry: &ash::Entry) -> Self {
        let application_info = vk::ApplicationInfo {
            p_application_name: Self::APPLICATION_NAME,
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: Self::ENGINE_NAME,
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        };

        let layers_names = Self::get_layer_names();
        let extension_names = Self::get_extension_names();

        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_layer_names(&layers_names)
            .enabled_extension_names(&extension_names);
        let instance = unsafe { entry.create_instance(&instance_info, None).unwrap() };

        Self { instance }
    }

    fn get_layer_names() -> Vec<*const i8> {
        #[cfg(debug_assertions)]
        {
            vec![Self::VALIDATION_LAYER_NAME]
        }

        #[cfg(not(debug_assertions))]
        {
            Default::default()
        }
    }

    fn get_extension_names() -> Vec<*const i8> {
        Default::default()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

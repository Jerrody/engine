use ash::extensions::ext::DebugUtils;
use ash::vk;

use crate::error::{EngineError, EngineResult};
use logging::*;

unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message_type = std::format!("{message_type:?}");
    let message = std::format!("\n{TAB_IN_SPACES}[{message_type}] {}", unsafe {
        std::ffi::CStr::from_ptr((*p_callback_data).p_message).to_string_lossy()
    });

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            info!(message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            warning!(message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            error!(message);
        }
        _ => (),
    }

    vk::FALSE
}

pub struct DebugMessengerManager {
    pub debug_utils_loader: DebugUtils,
    pub debug_utils: vk::DebugUtilsMessengerEXT,
}

impl DebugMessengerManager {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> EngineResult<Self> {
        debug!("Initializing Validation Layer of Vulkan Instance");

        let debug_utils_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(debug_callback));

        let debug_utils_loader = DebugUtils::new(entry, instance);
        let debug_utils = match unsafe {
            debug_utils_loader.create_debug_utils_messenger(&debug_utils_info, None)
        } {
            Ok(debug_utils) => debug_utils,
            Err(e) => return EngineResult::Err(EngineError::VulkanApiError(e)),
        };

        Ok(Self {
            debug_utils_loader,
            debug_utils,
        })
    }
}

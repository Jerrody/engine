use logging::*;

use crate::{
    error::{EngineError, EngineResult},
    renderer::utils::to_cstr,
};

use std::ffi::c_char;

use ash::vk;

pub struct PresentModes {
    pub mailbox: Option<vk::PresentModeKHR>,
    pub fifo_relaxed: Option<vk::PresentModeKHR>,
}

impl PresentModes {
    pub fn new(present_modes: &[vk::PresentModeKHR]) -> Option<Self> {
        let does_support_mailbox = present_modes.contains(&vk::PresentModeKHR::MAILBOX);
        let does_support_fifo = present_modes.contains(&vk::PresentModeKHR::FIFO);

        let (mailbox, fifo_relaxed) = match (does_support_mailbox, does_support_fifo) {
            (true, true) => (
                Some(vk::PresentModeKHR::MAILBOX),
                Some(vk::PresentModeKHR::FIFO),
            ),
            (true, false) => (Some(vk::PresentModeKHR::MAILBOX), None),
            (false, true) => (None, Some(vk::PresentModeKHR::FIFO)),
            (false, false) => return None,
        };

        Some(Self {
            mailbox,
            fifo_relaxed,
        })
    }
}

pub struct DeviceManager {
    _physical_device: vk::PhysicalDevice,
    device: ash::Device,
    pub device_properties: vk::PhysicalDeviceProperties,
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_modes: PresentModes,
    pub graphics_queue: vk::Queue,
    pub transfer_queue: vk::Queue,
}

impl DeviceManager {
    const DEFAULT_QUEUE_PRIORITY: f32 = 1.0;

    pub fn new(
        instance: &ash::Instance,
        surface_handle: &super::SurfaceManager,
    ) -> EngineResult<Self> {
        let surface = surface_handle.surface;
        let surface_loader = &surface_handle.surface_loader;

        debug!("Finding suitable device.");

        let required_layer_names = super::Context::REQUIRED_LAYERS;
        let required_extension_names = [ash::extensions::khr::Swapchain::name().as_ptr()];
        let (physical_device, device_properties, queue_family_index, surface_format, present_modes) = unsafe {
            instance
                .enumerate_physical_devices()?
                .into_iter()
                .filter_map(|physical_device| {
                    let device_properties =
                        instance.get_physical_device_properties(physical_device);
                    let device_name = to_cstr(device_properties.device_name.as_ptr())
                        .to_str()
                        .unwrap()
                        .to_owned();

                    debug!(std::format!(
                        "Checking for compatibility GPU: {device_name}."
                    ));

                    // TODO: Make more complicated algorithm of finding suitable family queue in the feature.
                    debug!("Checking for Queue Families requirements.");

                    let queue_family_index = match instance
                        .get_physical_device_queue_family_properties(physical_device)
                        .into_iter()
                        .enumerate()
                        .find(|(i, queue_family_property)| {
                            let queue_flags = queue_family_property.queue_flags;

                            // TODO: This implementation considers that queue family has both GRAPHICS and TRANSFER.
                            // Later need to make it more flexible and general for the GPU of any kind.
                            queue_family_property.queue_count > 1
                                && queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                && queue_flags.contains(vk::QueueFlags::TRANSFER)
                                && surface_loader
                                    .get_physical_device_surface_support(
                                        physical_device,
                                        *i as u32,
                                        surface,
                                    )
                                    .unwrap()
                        }) {
                        Some(queue_family) => queue_family.0,
                        None => {
                            debug!("Unable to find required Queues.");

                            return None;
                        }
                    };

                    debug!("Checking for Surface Format requirements.");

                    let surface_formats = surface_loader
                        .get_physical_device_surface_formats(physical_device, surface)
                        .unwrap();
                    let surface_format = match surface_formats.into_iter().find(|surface_format| {
                        let format = surface_format.format;

                        (format == vk::Format::R8G8B8A8_SRGB || format == vk::Format::B8G8R8A8_SRGB)
                            && surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
                    }) {
                        Some(surface_format) => surface_format,
                        None => {
                            debug!(
                                "Unable to find required Surface Format (RGBA8, SRGB Nonlinear)."
                            );

                            return None;
                        }
                    };

                    debug!("Checking for Present Modes requirements.");

                    let present_modes = surface_loader
                        .get_physical_device_surface_present_modes(physical_device, surface)
                        .unwrap();
                    let present_modes = match PresentModes::new(&present_modes) {
                        Some(present_modes) => present_modes,
                        None => {
                            debug!("Unable to find required Present Modes (MAILBOX, FIFO)");

                            return None;
                        }
                    };

                    debug!("Checking for Device Layers requirement.");

                    #[cfg(feature = "dev")]
                    {
                        let available_layers = instance
                            .enumerate_device_layer_properties(physical_device)
                            .unwrap();
                        let does_support = super::Context::does_support_layers(
                            required_layer_names,
                            &available_layers,
                            &std::format!(
                                "Failed to find required Layers of Device - {device_name}",
                            ),
                        );
                        if !does_support {
                            return None;
                        }
                    }

                    debug!("Checking for Device Extensions requirement.");

                    let available_extensions = instance
                        .enumerate_device_extension_properties(physical_device)
                        .unwrap();
                    let does_support = super::Context::does_support_extensions(
                        &required_extension_names,
                        &available_extensions,
                        &std::format!(
                            "Failed to find required Extensions of Device - {device_name}",
                        ),
                    );
                    if !does_support {
                        return None;
                    }

                    Some((
                        physical_device,
                        device_properties,
                        queue_family_index as u32,
                        surface_format,
                        present_modes,
                    ))
                })
                .max_by_key(
                    |(_, device_properties, _, _, _)| match device_properties.device_type {
                        vk::PhysicalDeviceType::DISCRETE_GPU => 2,
                        vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                        _ => Default::default(),
                    },
                )
                .ok_or(EngineError::DeviceCreationFailed(
                    "Unable to find suitable Device.".to_owned(),
                ))?
        };

        let device_name = to_cstr(device_properties.device_name.as_ptr())
            .to_str()
            .unwrap()
            .to_owned();
        info!(std::format!("Found suitable GPU: {device_name}.",));
        debug!("Creating Vulkan Device.");

        let feature_names = [ash::extensions::khr::DynamicRendering::name()];
        let mut device_features13 =
            vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true);
        let mut device_features2 =
            vk::PhysicalDeviceFeatures2::default().push_next(&mut device_features13);

        let queue_infos = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[Self::DEFAULT_QUEUE_PRIORITY, Self::DEFAULT_QUEUE_PRIORITY])];

        let device_info = vk::DeviceCreateInfo::default()
            .enabled_layer_names(required_layer_names)
            .enabled_extension_names(&required_extension_names)
            .queue_create_infos(&queue_infos)
            .push_next(&mut device_features2);
        let device = unsafe { instance.create_device(physical_device, &device_info, None)? };

        Self::print_info(
            &device_name,
            queue_family_index,
            required_layer_names,
            &required_extension_names,
            &feature_names,
        );

        debug!("Picking Queues of Device.");

        let graphics_queue =
            unsafe { device.get_device_queue(queue_family_index, Default::default()) };
        let transfer_queue =
            unsafe { device.get_device_queue(queue_family_index, Default::default()) };

        Ok(Self {
            _physical_device: physical_device,
            device_properties,
            device,
            surface_format,
            present_modes,
            graphics_queue,
            transfer_queue,
        })
    }

    #[inline(always)]
    pub fn create_shader_module(
        &self,
        spirv_binary: &shaderc::CompilationArtifact,
    ) -> EngineResult<vk::ShaderModule> {
        let spirv_binary = spirv_binary.as_binary();
        let shader_module_info = vk::ShaderModuleCreateInfo::default().code(spirv_binary);

        unsafe {
            Ok(self
                .device
                .create_shader_module(&shader_module_info, None)?)
        }
    }

    #[inline(always)]
    pub fn destroy_shader_module(&self, shader_module: vk::ShaderModule) {
        unsafe { self.device.destroy_shader_module(shader_module, None) }
    }

    #[inline(always)]
    pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        unsafe { self.device.destroy_semaphore(semaphore, None) }
    }

    #[inline(always)]
    pub fn destroy_fence(&self, fence: vk::Fence) {
        unsafe { self.device.destroy_fence(fence, None) }
    }

    #[inline(always)]
    pub fn create_semaphore(
        &self,
        semaphore_info: &vk::SemaphoreCreateInfo,
    ) -> EngineResult<vk::Semaphore> {
        unsafe { Ok(self.device.create_semaphore(semaphore_info, None)?) }
    }

    #[inline(always)]
    pub fn create_fence(&self, fence_info: &vk::FenceCreateInfo) -> EngineResult<vk::Fence> {
        unsafe { Ok(self.device.create_fence(fence_info, None)?) }
    }

    #[inline(always)]
    pub fn wait_for_idle(&self) -> EngineResult<()> {
        unsafe { Ok(self.device.device_wait_idle()?) }
    }

    #[inline(always)]
    pub fn destroy_device(&self) {
        unsafe { self.device.destroy_device(None) }
    }

    fn print_info(
        device_name: &str,
        queue_family_index: u32,
        layer_names: &[*const c_char],
        extension_names: &[*const c_char],
        features_names: &[&std::ffi::CStr],
    ) {
        let mut device_info = String::from("Created a Device.\n\n");

        device_info.push_str(&std::format!("{TAB_IN_SPACES}Device Info:\n"));

        let device_name = std::format!("{TAB_IN_SPACES}- Device Name: {device_name}\n");
        device_info.push_str(&device_name);

        device_info.push_str(&device_name);

        let queue_info =
            std::format!("{TAB_IN_SPACES}- Using Queue Family Index: {queue_family_index}\n\n");
        device_info.push_str(&queue_info);

        #[cfg(feature = "dev")]
        {
            device_info.push_str(&std::format!("{TAB_IN_SPACES}With Layers:\n"));
            layer_names.iter().for_each(|layer_name| {
                device_info.push_str(&std::format!(
                    "{TAB_IN_SPACES}- {}\n",
                    to_cstr(*layer_name).to_str().unwrap()
                ))
            });
        }

        device_info.push_str(&std::format!("\n{TAB_IN_SPACES}With Extensions:\n"));
        extension_names.iter().for_each(|extension_name| {
            device_info.push_str(&std::format!(
                "{TAB_IN_SPACES}- {}\n",
                to_cstr(*extension_name).to_str().unwrap()
            ));
        });

        device_info.push_str(&std::format!("\n{TAB_IN_SPACES}With Features:\n"));
        features_names.iter().for_each(|feature_name| {
            device_info.push_str(&std::format!(
                "{TAB_IN_SPACES}- {}\n",
                feature_name.to_str().unwrap()
            ));
        });

        debug!(device_info);
    }
}

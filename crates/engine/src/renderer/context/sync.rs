use logging::*;

use crate::error::EngineResult;
use ash::vk;

pub struct SynchronizationPrimitivesManager {
    pub available_image_semaphore: vk::Semaphore,
    pub render_semaphore: vk::Semaphore,
    pub render_fence: vk::Fence,
}

impl SynchronizationPrimitivesManager {
    #[inline(always)]
    pub fn new(device: &super::DeviceManager) -> EngineResult<Self> {
        debug!("Initializing sync primitives");

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let available_image_semaphore = device.create_semaphore(&semaphore_info)?;
        let render_semaphore = device.create_semaphore(&semaphore_info)?;

        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let render_fence = device.create_fence(&fence_info)?;

        Ok(Self {
            available_image_semaphore,
            render_semaphore,
            render_fence,
        })
    }

    #[inline(always)]
    pub fn destroy_resources(&self, device: &super::DeviceManager) {
        device.destroy_semaphore(self.available_image_semaphore);
        device.destroy_semaphore(self.render_semaphore);
        device.destroy_fence(self.render_fence);
    }
}

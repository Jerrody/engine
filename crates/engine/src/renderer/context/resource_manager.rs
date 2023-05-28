use once_cell::sync::Lazy;

use super::shader::Shader;
use crate::common::Id;

pub static mut RESOURCE_MANAGER: Lazy<ResourceManager> = Lazy::new(ResourceManager::new);

pub struct ResourceManager {
    device: *const super::DeviceManager,
    shaders: Vec<*const Shader>,
    // pub textures: HashMap<Id, Texture>,
    // pub meshes: HashMap<Id, Mesh>,
    // pub materials: HashMap<Id, Material>,
    // pub pipelines: HashMap<Id, Pipeline>,
    // pub render_passes: HashMap<Id, RenderPass>,
    // pub framebuffers: HashMap<Id, Framebuffer>,
    // pub samplers: HashMap<Id, Sampler>,
    // pub descriptor_sets: HashMap<Id, DescriptorSet>,
    // pub descriptor_set_layouts: HashMap<Id, DescriptorSetLayout>,
    // pub descriptor_pools: HashMap<Id, DescriptorPool>,
    // pub command_buffers: HashMap<Id, CommandBuffer>,
    // pub command_pools: HashMap<Id, CommandPool>,
    // pub fences: HashMap<Id, Fence>,
    // pub semaphores: HashMap<Id, Semaphore>,
    // pub events: HashMap<Id, Event>,
    // pub query_pools: HashMap<Id, QueryPool>,
    // pub buffers: HashMap<Id, Buffer>,
    // pub images: HashMap<Id, Image>,
    // pub image_views: HashMap<Id, ImageView>,
    // pub framebuffers: HashMap<Id, Framebuffer>,
    // pub render_passes: HashMap<Id, RenderPass>,
    // pub pipelines: HashMap<Id, Pipeline>,
    // pub pipeline_layouts: HashMap<Id, PipelineLayout>,
    // pub pipeline_caches: HashMap<Id, PipelineCache>,
    // pub pipeline_cache: HashMap<I
}

impl ResourceManager {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            device: std::ptr::null(),
            shaders: Default::default(),
        }
    }

    pub fn set_device(&mut self, device: &super::DeviceManager) {
        self.device = device;
    }

    #[inline]
    pub fn register_shader(&mut self, shader: &Shader) {
        self.shaders.push(shader);
    }

    pub unsafe fn delete_shader(&mut self, id: Id) {
        let Some((index, shader)) = (unsafe {
            self.shaders
                .iter()
                .enumerate()
                .find(|(_, &shader)| (*shader).id == id)
        }) else {
            return;
        };

        unsafe {
            (*self.device).destroy_shader_module(shader.as_ref().unwrap_unchecked().module);
        }

        self.shaders.remove(index);
    }
}

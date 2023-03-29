use ash::vk::Result;
use thiserror::Error;

pub type EngineResult<T> = std::result::Result<T, EngineError>;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Failed to load Vulkan Lib: {0}.")]
    VulkanLoadingError(#[from] ash::LoadingError),
    #[error("Failed to create Vulkan Instance: {0}.")]
    InstanceCreationFailed(String),
    #[error("Failed to create Vulkan Device: {0}.")]
    DeviceCreationFailed(String),
    #[error("Vulkan API Error: {0}.")]
    VulkanApiError(#[from] Result),
    #[error("{0}.")]
    UnknownError(String),
}

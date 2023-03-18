use ash::vk::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Vulkan Error: {0}.")]
    VulkanResult(ash::vk::Result),
    #[error("{0}")]
    Error(String),
}

impl From<ash::vk::Result> for EngineError {
    fn from(error: Result) -> Self {
        Self::VulkanResult(error)
    }
}

impl From<&str> for EngineError {
    fn from(error: &str) -> Self {
        Self::Error(error.to_owned())
    }
}

use std::path::Path;

use crate::{
    common::Id,
    error::{EngineError, EngineResult},
};

use ash::vk;

use super::resource_manager::RESOURCE_MANAGER;

pub struct Shader {
    pub id: Id,
    pub module: vk::ShaderModule,
    pub stage: vk::ShaderStageFlags,
}

impl Shader {
    #[inline(always)]
    fn new(id: Id, module: vk::ShaderModule, stage: vk::ShaderStageFlags) -> Self {
        Self { id, module, stage }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            RESOURCE_MANAGER.delete_shader(self.id);
        }
    }
}

pub struct ShaderManager<'a> {
    compiler_options: shaderc::CompileOptions<'a>,
    compiler: shaderc::Compiler,
    shaders: Vec<Shader>,
}

impl ShaderManager<'_> {
    pub const DEFAULT_ENTRY_POINT: &'static str = "main";

    pub fn new() -> Self {
        let mut compiler_options = shaderc::CompileOptions::new().unwrap();
        compiler_options.set_target_env(
            shaderc::TargetEnv::Vulkan,
            shaderc::EnvVersion::Vulkan1_3 as _,
        );
        compiler_options.set_optimization_level(shaderc::OptimizationLevel::Performance);

        let compiler = shaderc::Compiler::new().unwrap();

        Self {
            compiler_options,
            compiler,
            shaders: Default::default(),
        }
    }

    #[inline(always)]
    pub fn load_shaders<T: AsRef<Path>>(
        &mut self,
        _device: &super::DeviceManager,
        shaders_paths: &[T],
    ) -> EngineResult<()> {
        for _shader_path in shaders_paths {}

        Ok(())
    }

    pub fn load_shader<T: AsRef<Path>>(
        &mut self,
        device: &super::DeviceManager,
        path: &T,
    ) -> EngineResult<()> {
        let path_as_str: &str = path.as_ref().to_str().unwrap();
        let id = Id::new(path_as_str);

        if self.shaders.iter().any(|shader| shader.id == id) {
            return Err(EngineError::ShaderError(format!(
                "Found duplicated shader: {path_as_str}."
            )));
        }

        let splitted_path = path_as_str.split('.').collect::<Vec<_>>();
        let Some(kind_name) = splitted_path.get(2) else {
            return Err(EngineError::ShaderError("Invalid shader file extension.".to_owned()));
        };

        let kind = Self::map_shader_kind_from_str(kind_name).ok_or_else(|| {
            EngineError::ShaderError(format!("Invalid shader type: {}", kind_name))
        })?;

        let source = std::fs::read_to_string(path_as_str).map_err(|e| {
            EngineError::ShaderError(format!("Failed to read shader file {path_as_str}: {e}"))
        })?;

        let filename = path.as_ref().file_name().unwrap().to_str().unwrap();
        let spirv = self
            .compiler
            .compile_into_spirv(
                &source,
                kind,
                filename,
                Self::DEFAULT_ENTRY_POINT,
                Some(&self.compiler_options),
            )
            .map_err(|e| {
                EngineError::ShaderError(format!("Failed to compile shader {path_as_str}: {e}"))
            })?;

        let shader_module = device.create_shader_module(&spirv)?;

        let shader = Shader::new(id, shader_module, Self::map_shader_stage_from_kind(kind));

        unsafe {
            RESOURCE_MANAGER.register_shader(&shader);
        }
        self.shaders.push(shader);

        Ok(())
    }

    #[inline(always)]
    pub fn unload_shader(&mut self, id: Id, _device: &super::DeviceManager) {
        unsafe {
            RESOURCE_MANAGER.delete_shader(id);
        }
    }

    #[inline(always)]
    pub fn unload_shaders(&mut self, _device: &super::DeviceManager) {
        for shader in self.shaders.drain(..) {
            unsafe {
                RESOURCE_MANAGER.delete_shader(shader.id);
            }
        }
    }

    #[inline(always)]
    fn map_shader_kind_from_str(kind_name: &str) -> Option<shaderc::ShaderKind> {
        let kind = match kind_name {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            "geom" => shaderc::ShaderKind::Geometry,
            "tesc" => shaderc::ShaderKind::TessControl,
            "tese" => shaderc::ShaderKind::TessEvaluation,
            _ => return None,
        };

        Some(kind)
    }

    #[inline(always)]
    fn map_shader_stage_from_kind(kind: shaderc::ShaderKind) -> vk::ShaderStageFlags {
        match kind {
            shaderc::ShaderKind::Vertex => vk::ShaderStageFlags::VERTEX,
            shaderc::ShaderKind::Fragment => vk::ShaderStageFlags::FRAGMENT,
            shaderc::ShaderKind::Compute => vk::ShaderStageFlags::COMPUTE,
            shaderc::ShaderKind::Geometry => vk::ShaderStageFlags::GEOMETRY,
            shaderc::ShaderKind::TessControl => vk::ShaderStageFlags::TESSELLATION_CONTROL,
            shaderc::ShaderKind::TessEvaluation => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
            _ => unreachable!(),
        }
    }
}

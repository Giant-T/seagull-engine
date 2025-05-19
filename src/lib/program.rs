use std::{ffi::{c_char, CString}, rc::Rc};

use anyhow::Result;
use log::info;

use crate::extensions::Extensions;

use super::gl::{
    self, FALSE, FRAGMENT_SHADER_BIT, LINK_STATUS, VALIDATE_STATUS, VERTEX_SHADER_BIT,
    types::GLenum,
};

pub struct Shader {
    pub id: u32,
    extensions: Rc<Extensions>,
}

impl Shader {
    pub fn new(extensions: Rc<Extensions>, shader_type: u32, source: &str) -> Result<Self> {
        let id;
        unsafe {
            let c_str = CString::new(source)?;
            let source: [*const c_char; 1] = [c_str.as_ptr()];
            id = gl::CreateShaderProgramv(shader_type, 1, source.as_ptr());

            let mut is_valid: i32 = FALSE.into();
            gl::GetProgramiv(id, LINK_STATUS, &mut is_valid as *mut i32);

            if is_valid == FALSE.into() {
                return Err(anyhow::anyhow!("Could not compile shader"));
            }
        }

        info!("Initialized shader {id}");

        Ok(Self { id, extensions })
    }

    pub fn get_loc(&self, name: &str) -> Result<i32> {
        return unsafe {
            let loc = gl::GetUniformLocation(self.id, CString::new(name)?.as_ptr());
            if loc == -1 {
                return Err(anyhow::anyhow!(
                    "Uniform '{name}' not found in shader {}",
                    self.id
                ));
            }
            Ok(loc)
        };
    }

    pub fn uniform_1ui_arb(&self, location: i32, val: u64) {
        unsafe {
            (self.extensions.gl_program_uniform_1ui_arb)(
                self.id,
                location,
                val,
            );
        }
    }

    pub fn uniform_1f(&self, location: i32, val: f32) {
        unsafe {
            gl::ProgramUniform1f(
                self.id,
                location,
                val,
            );
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Pipeline {
    pub id: u32,
}

impl Pipeline {
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self> {
        let mut id = 0;
        unsafe {
            gl::CreateProgramPipelines(1, &mut id);
            gl::UseProgramStages(id, VERTEX_SHADER_BIT, vertex_shader.id);
            gl::UseProgramStages(id, FRAGMENT_SHADER_BIT, fragment_shader.id);
            gl::ValidateProgramPipeline(id);

            let mut is_valid: i32 = FALSE.into();
            gl::GetProgramPipelineiv(id, VALIDATE_STATUS, &mut is_valid as *mut i32);
            if is_valid == FALSE.into() {
                return Err(anyhow::anyhow!("Could not link program pipeline"));
            }
        }

        info!("Pipeline {id} was created successfully");

        Ok(Self { id })
    }

    pub fn use_shader(&self, stage: GLenum, shader_id: u32) {
        unsafe {
            gl::UseProgramStages(self.id, stage, shader_id);
            gl::ValidateProgramPipeline(self.id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindProgramPipeline(self.id);
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgramPipelines(1, &self.id);
        }
    }
}

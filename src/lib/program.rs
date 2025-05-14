use std::ffi::{CString, c_char};

use anyhow::Result;

use super::gl::{
    self, FALSE, FRAGMENT_SHADER_BIT, LINK_STATUS, VALIDATE_STATUS, VERTEX_SHADER_BIT,
    types::GLenum,
};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(shader_type: u32, source: &str) -> Result<Self> {
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

        println!("Shader {id} was created successfully");

        Ok(Self { id })
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

        println!("Pipeline {id} was created successfully");

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

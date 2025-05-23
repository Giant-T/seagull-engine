use std::sync::Arc;

use anyhow::Result;
use glow::{HasContext, UniformLocation};
use log::info;

pub struct Shader {
    pub id: glow::Program,
    gl: Arc<glow::Context>,
}

impl Shader {
    pub fn new(gl: Arc<glow::Context>, vertex_source: &str, fragment_source: &str) -> Result<Self> {
        unsafe {
            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .or_else(|s| Err(anyhow::anyhow!(s)))?;
            gl.shader_source(vertex_shader, vertex_source);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                let log = gl.get_shader_info_log(vertex_shader);
                return Err(anyhow::anyhow!("Vertex shader compile error: {}", log));
            }

            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .or_else(|s| Err(anyhow::anyhow!(s)))?;
            gl.shader_source(fragment_shader, fragment_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                let log = gl.get_shader_info_log(fragment_shader);
                return Err(anyhow::anyhow!("Fragment shader compile error: {}", log));
            }

            let program = gl.create_program().or_else(|s| Err(anyhow::anyhow!(s)))?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                return Err(anyhow::anyhow!("Program link error: {}", log));
            }

            info!(
                "Compile shaders {vertex_shader:?} and {fragment_shader:?}, and linked program {program:?}"
            );

            // We can delete shaders after linking
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self { id: program, gl })
        }
    }

    pub fn get_loc(&self, name: &str) -> Result<UniformLocation> {
        unsafe {
            let loc = self.gl.get_uniform_location(self.id, name);
            let Some(loc) = loc else {
                return Err(anyhow::anyhow!(
                    "Uniform '{name}' not found in program {:?}",
                    self.id
                ));
            };

            Ok(loc)
        }
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl.use_program(Some(self.id));
        }
    }

    pub fn uniform_1i(&self, location: &UniformLocation, v0: i32) {
        unsafe {
            self.gl.uniform_1_i32(Some(location), v0);
        }
    }

    pub fn uniform_1f(&self, location: &UniformLocation, v0: f32) {
        unsafe {
            self.gl.uniform_1_f32(Some(location), v0);
        }
    }
}

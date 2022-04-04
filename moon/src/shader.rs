//! The [`Shader`] struct, and [`ShaderType`] enum.

use std::fmt;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlUniformLocation;

use crate::{gl, GL};

/// Type of Shader
#[repr(u32)]
pub enum ShaderType {
    /// Vertex Shader.
    VERTEX = GL::VERTEX_SHADER,
    /// Fragment Shader.
    FRAGMENT = GL::FRAGMENT_SHADER,
}

/// A program that is run on the GPU.
///
/// A [Shader] contains a [`Program`](WebGlProgram), that can be bound and run on the GPU using [WebGL](GL).
#[derive(Debug)]
pub struct Shader {
    /// A name to refer to the shader with, and for debugging purposes.
    pub name: &'static str,
    program: Option<WebGlProgram>,
}

impl Default for Shader {
    fn default() -> Self {
        Self {
            name: "Uninitialized Shader",
            program: None,
        }
    }
}

impl fmt::Display for Shader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.program.is_some() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} (SHADER PROGRAM MISSING)", self.name)
        }
    }
}

impl gl::Bind for Shader {
    /// Bind the `Shader`
    fn bind(&self, gl: &GL) {
        gl.use_program(self.program.as_ref());
    }
    fn unbind(&self, gl: &GL) {
        gl.use_program(None);
    }
}

impl Shader {
    /// Create a new Shader Program with default Vertex and Fragment shaders.
    pub fn new(gl: &GL) -> Self {
        let name = "Default Shader";
        let vertex_shader =
            Shader::create_vertex(gl, include_str!("../res/shader/default.vert.glsl"))
                .expect("Could not create Vertex Shader!");

        let fragment_shader =
            Shader::create_fragment(gl, include_str!("../res/shader/default.frag.glsl"))
                .expect("Could not create Fragment Shader!");

        let program =
            Shader::program_with_vertex_and_fragment(gl, &vertex_shader, &fragment_shader).ok();

        Self { name, program }
    }

    /// Create a new Shader with default Fragment Shader and a custom Vertex Shader.
    pub fn new_with_vertex(
        gl: &GL,
        vertex_shader: WebGlShader,
        name: Option<&'static str>,
    ) -> Self {
        let name = name.unwrap_or("Custom Vertex Shader");

        let fragment_shader =
            Shader::create_fragment(gl, include_str!("../res/shader/default.frag.glsl"))
                .expect("Could not create Fragment Shader!");

        let program =
            Shader::program_with_vertex_and_fragment(gl, &vertex_shader, &fragment_shader).ok();

        Self { name, program }
    }

    /// Create a fragment `WebGlShader`.
    pub fn create_fragment(gl: &GL, source: &str) -> Result<WebGlShader, String> {
        Self::create_with_type(gl, ShaderType::FRAGMENT, source)
    }

    /// Create a vertex `WebGlShader`.
    pub fn create_vertex(gl: &GL, source: &str) -> Result<WebGlShader, String> {
        Self::create_with_type(gl, ShaderType::VERTEX, source)
    }

    /// Create a new `WebGlShader` with a given `ShaderType`.
    pub fn create_with_type(
        gl: &GL,
        shader_type: ShaderType,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type as u32)
            .ok_or_else(|| String::from("Unable to create Shader object."))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Could not compile shader.")))
        }
    }

    /// Create a new [`WebGlProgram`] with the given vertex and fragment [`shaders`](WebGlShader).
    pub fn program_with_vertex_and_fragment(
        gl: &GL,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create Program object."))?;
        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            gl.delete_shader(Some(vertex_shader));
            gl.delete_shader(Some(fragment_shader));
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Could not link program.")))
        }
    }

    /// Get the location of a uniform on the `Shader`.
    pub fn get_uniform_location(&self, gl: &GL, name: &str) -> Option<WebGlUniformLocation> {
        self.program
            .as_ref()
            .map(|program| gl.get_uniform_location(program, name))
            .unwrap()
    }

    /// Get the location of an attribute on the `Shader`.
    pub fn get_attrib_location(&self, gl: &GL, name: &str) -> Option<i32> {
        self.program
            .as_ref()
            .map(|program| gl.get_attrib_location(program, name))
    }
}

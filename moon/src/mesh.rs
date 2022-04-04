//! The definitions of [`Vertex`], [`Mesh`] and their implementations.

use web_sys::{WebGlBuffer, WebGlVertexArrayObject};

use crate::{gl, GL};

/// The `Vertex` struct holds the data that will be later sent to WebGL in a `GL::ARRAY_BUFFER`.
/// It consists of position and color vectors, and UV co-ordinates.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    /// A two component array of [`f32`], representing the position of the [`Vertex`].
    pub position: [f32; 2],
    /// A two component array of [`f32`], representing the UV co-ordinates of the [`Vertex`].
    pub uv: [f32; 2],
    /// A four component array of [`f32`], representing the color of the [`Vertex`].
    pub color: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            uv: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// An indiced [`Mesh`], stored along with it's vertex array, index array and vertex buffer.
#[derive(Debug)]
pub struct Mesh {
    /// Vertices of the Mesh.
    ///
    /// Represented as a [`Vec`] of [`Vertex`]s.
    pub vertices: Vec<Vertex>,
    /// Indices of the Mesh.
    ///
    /// Stored as a [`Vec`] of [`u32`].
    pub indices: Vec<u32>,
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    ibo: WebGlBuffer,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        use gl::Bind;
        let gl = gl::get_context();
        self.unbind(&gl);

        gl.delete_buffer(Some(&self.vbo));
        gl.delete_buffer(Some(&self.ibo));
        self.vertices.clear();
        self.indices.clear();
        gl.delete_vertex_array(Some(&self.vao));
    }
}

impl gl::Bind for Mesh {
    /// Bind the `WebGlVertexArrayObject` of the `Mesh`.
    fn bind(&self, gl: &GL) {
        gl.bind_vertex_array(Some(&self.vao));
    }
    fn unbind(&self, gl: &GL) {
        gl.bind_vertex_array(None);
    }
}

impl Mesh {
    /// Create a new [`Mesh`] with the given [`vertices`](Vertex) and indices.
    pub fn new(gl: &GL, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vao: {
                let vao = gl
                    .create_vertex_array()
                    .expect("Could not create Vertex Array Object.");
                gl.bind_vertex_array(Some(&vao));
                vao
            },
            vbo: gl.create_buffer().expect("Could not create Buffer."),
            ibo: gl.create_buffer().expect("Could not create Buffer."),
        }
    }
    /// Create a new Quad mesh with a side length of 1m
    pub fn quad(gl: &GL) -> Self {
        Self::quad_with_side(gl, 1.0)
    }
    /// Create a new Quad mesh with a given side length
    pub fn quad_with_side(gl: &GL, side: f32) -> Self {
        let half = side / 2.0;
        let vertices = vec![
            Vertex {
                position: [-half, half],
                uv: [0.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [-half, -half],
                uv: [0.0, 1.0],
                ..Default::default()
            },
            Vertex {
                position: [half, -half],
                uv: [1.0, 1.0],
                ..Default::default()
            },
            Vertex {
                position: [half, half],
                uv: [1.0, 0.0],
                ..Default::default()
            },
        ];
        let indices: Vec<u32> = vec![0, 2, 1, 0, 3, 2];
        Self::new(gl, vertices, indices)
    }

    /// Set up the vertex (vbo) and index (ibo) `WebGlBuffer` and send their data to the GPU.
    pub fn setup(&self, gl: &GL) {
        use gl::Bind;
        self.bind(gl);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vbo));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.ibo));

        let vertex_slice = unsafe {
            std::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8,
                self.vertices.len() * std::mem::size_of::<Vertex>(),
            )
        };
        let index_slice = unsafe {
            std::slice::from_raw_parts(
                self.indices.as_ptr() as *const u8,
                self.indices.len() * std::mem::size_of::<u32>(),
            )
        };

        gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, vertex_slice, GL::DYNAMIC_DRAW);
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, index_slice, GL::DYNAMIC_DRAW);

        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 8 * 4, 0);
        gl.vertex_attrib_pointer_with_i32(1, 2, GL::FLOAT, false, 8 * 4, 8);
        gl.vertex_attrib_pointer_with_i32(2, 4, GL::FLOAT, false, 8 * 4, 16);

        gl.enable_vertex_attrib_array(0);
        gl.enable_vertex_attrib_array(1);
        gl.enable_vertex_attrib_array(2);
    }
}

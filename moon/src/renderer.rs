//! The [`Renderer`] annd [`Quad`] structs. Used for rendering.

use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;
use web_sys::WebGlUniformLocation;

use crate::component::Component;
use crate::{gl, mesh, texture, Color32};
use crate::{Camera, Shader, Transform, GL};

use gl::Bind;
use mesh::{Mesh, Vertex};
use texture::{SubTexture, Texture};

/// Maximum [`Quad`]s in a single batch.
pub const MAX_BATCH_QUADS: i32 = 1000;
const MAX_BATCH_VERTICES: i32 = MAX_BATCH_QUADS * 4;
const MAX_BATCH_INDICES: i32 = MAX_BATCH_QUADS * 6;

/// A [`Quad`] is a simple mesh definition with four [`Vertices`](Vertex).
#[derive(Debug)]
pub struct Quad([Vertex; 4]);

impl Default for Quad {
    fn default() -> Self {
        Self([
            Vertex {
                position: [-0.5, 0.5],
                uv: [0.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [-0.5, -0.5],
                uv: [0.0, 1.0],
                ..Default::default()
            },
            Vertex {
                position: [0.5, -0.5],
                uv: [1.0, 1.0],
                ..Default::default()
            },
            Vertex {
                position: [0.5, 0.5],
                uv: [1.0, 0.0],
                ..Default::default()
            },
        ])
    }
}

impl Quad {
    /// Create a new [`Quad`] from a given position, rotation, size and color.
    pub fn new_from_position_and_rotation_and_size_and_color(
        pos_x: f32,
        pos_y: f32,
        rotation: f32,
        size_x: f32,
        size_y: f32,
        color: Color32,
    ) -> Self {
        let size_x = size_x / 2.0;
        let size_y = size_y / 2.0;
        let color = <[f32; 4]>::from(color);
        let (sin_theta, cos_theta) = rotation.sin_cos();
        let mut points = [
            [-size_x, -size_y],
            [-size_x, size_y],
            [size_x, size_y],
            [size_x, -size_y],
        ];
        for point in points.iter_mut() {
            let (x, y) = (point[0], point[1]);
            point[0] = pos_x + x * cos_theta - y * sin_theta;
            point[1] = pos_y + x * sin_theta + y * cos_theta;
        }
        Self([
            Vertex {
                position: points[0],
                uv: [0.0, 0.0],
                color,
            },
            Vertex {
                position: points[1],
                uv: [0.0, 1.0],
                color,
            },
            Vertex {
                position: points[2],
                uv: [1.0, 1.0],
                color,
            },
            Vertex {
                position: points[3],
                uv: [1.0, 0.0],
                color,
            },
        ])
    }

    /// Create a new [`Quad`] from a given position, size and color.
    pub fn new_from_position_and_size_and_color(
        pos_x: f32,
        pos_y: f32,
        size_x: f32,
        size_y: f32,
        color: Color32,
    ) -> Self {
        let size_x = size_x / 2.0;
        let size_y = size_y / 2.0;
        let color = <[f32; 4]>::from(color);
        Self([
            Vertex {
                position: [pos_x - size_x, pos_y + size_y],
                uv: [0.0, 0.0],
                color,
            },
            Vertex {
                position: [pos_x - size_x, pos_y - size_y],
                uv: [0.0, 1.0],
                color,
            },
            Vertex {
                position: [pos_x + size_x, pos_y - size_y],
                uv: [1.0, 1.0],
                color,
            },
            Vertex {
                position: [pos_x + size_x, pos_y + size_y],
                uv: [1.0, 0.0],
                color,
            },
        ])
    }

    /// Create a new [`Quad`] from a given position and size.
    pub fn new_from_position_and_size(pos_x: f32, pos_y: f32, size_x: f32, size_y: f32) -> Self {
        Self::new_from_position_and_size_and_color(pos_x, pos_y, size_x, size_y, Color32::WHITE)
    }

    /// Create a new [`Quad`] from a given position, size, and a reference to a [`SubTexture`].
    pub fn new_from_position_and_size_and_sprite(
        pos_x: f32,
        pos_y: f32,
        size_x: f32,
        size_y: f32,
        sprite: &SubTexture,
    ) -> Self {
        let uv = sprite.get_uv_coords();
        let size_x = size_x / 2.0;
        let size_y = size_y / 2.0;
        Self([
            Vertex {
                position: [pos_x - size_x, pos_y + size_y],
                uv: uv[0],
                ..Default::default()
            },
            Vertex {
                position: [pos_x - size_x, pos_y - size_y],
                uv: uv[1],
                ..Default::default()
            },
            Vertex {
                position: [pos_x + size_x, pos_y - size_y],
                uv: uv[2],
                ..Default::default()
            },
            Vertex {
                position: [pos_x + size_x, pos_y + size_y],
                uv: uv[3],
                ..Default::default()
            },
        ])
    }

    /// Create a new [`Quad`] using a given [`Transform`] for its position and scale.
    pub fn new_from_transform(transform: Transform) -> Self {
        Self::new_from_position_and_size(
            transform.position.x,
            transform.position.x,
            transform.scale.x,
            transform.scale.y,
        )
    }

    /// Create a new [`Quad`] using a given [`Transform`] for its position and scale, and a reference to [`SubTexture`].
    pub fn new_from_transform_and_sprite(transform: Transform, sprite: &SubTexture) -> Self {
        Self::new_from_position_and_size_and_sprite(
            transform.position.x,
            transform.position.x,
            transform.scale.x,
            transform.scale.y,
            sprite,
        )
    }

    /// Get the [`Vertices`](Vertex) of the [`Quad`] as a [`Vec`].
    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.0.to_vec()
    }
}

/// The [`Renderer`] is responsible for drawing on the screen. It handles the [`Camera`] and [`Shader`]s.
pub struct Renderer {
    /// The [`WebGl2RenderingContext`](web_sys::WebGl2RenderingContext) used by the [`Renderer`].
    pub gl: GL,
    /// The [`Shader`] used by the [`Renderer`].
    pub program: Shader,
    /// The [`Camera`] used by the [`Renderer`].
    pub camera: Camera,
    batches: Vec<Mesh>,
    /// [`Components`](Component) that can be added to the [`Renderer`].
    pub components: BTreeMap<&'static str, Box<dyn Component>>,
    textures: BTreeMap<&'static str, Rc<Texture>>,
    u_time: Option<WebGlUniformLocation>,
    u_color: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_view_matrix: Option<WebGlUniformLocation>,
    u_projection_matrix: Option<WebGlUniformLocation>,
}

impl Default for Renderer {
    fn default() -> Self {
        let gl = gl::get_context();
        let program = Shader::new(&gl);
        Self {
            camera: Camera::default(),
            batches: Vec::new(),
            components: BTreeMap::new(),
            u_time: program.get_uniform_location(&gl, "uTime"),
            u_color: program.get_uniform_location(&gl, "uColor"),
            u_model_matrix: program.get_uniform_location(&gl, "uModel"),
            u_view_matrix: program.get_uniform_location(&gl, "uView"),
            u_projection_matrix: program.get_uniform_location(&gl, "uProj"),
            program,
            textures: {
                let mut textues = BTreeMap::<&str, Rc<Texture>>::new();
                textues.insert("WHITE", Rc::new(Texture::white(&gl)));
                textues.insert("MAGENTA", Rc::new(Texture::colored(&gl, Color32::MAGENTA)));
                textues.insert("CHECKERBOARD", Rc::new(Texture::checkerboard(&gl)));
                textues
            },
            gl,
        }
    }
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renderer")
            .field("gl", &self.gl)
            .field("program", &self.program)
            .field("camera", &self.camera)
            .field("batches", &self.batches)
            .field("textures", &self.textures)
            .field("u_time", &self.u_time)
            .field("u_color", &self.u_color)
            .field("u_model_matrix", &self.u_model_matrix)
            .field("u_view_matrix", &self.u_view_matrix)
            .field("u_projection_matrix", &self.u_projection_matrix)
            .finish()
    }
}

impl Renderer {
    /// Create a new [`Renderer`] with a given [`Camera`].
    pub fn new_with_camera(camera: Camera) -> Self {
        Self {
            camera,
            ..Default::default()
        }
    }

    /// Create a new [`Renderer`] with a given [`Camera`] and [`Shader`].
    pub fn new_with_camera_and_program(camera: Camera, program: Shader) -> Self {
        let gl = gl::get_context();
        program.bind(&gl);

        Self {
            camera,
            u_time: program.get_uniform_location(&gl, "uTime"),
            u_color: program.get_uniform_location(&gl, "uColor"),
            u_model_matrix: program.get_uniform_location(&gl, "uModel"),
            u_view_matrix: program.get_uniform_location(&gl, "uView"),
            u_projection_matrix: program.get_uniform_location(&gl, "uProj"),
            program,
            gl,
            ..Default::default()
        }
    }

    /// Set the [`Camera`] that the [`Renderer`] will use.
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    /// Set the [`Shader`] that the [`Renderer`] will use.
    pub fn set_shader(&mut self, program: Shader) {
        let gl = &self.gl;

        self.u_time = program.get_uniform_location(gl, "uTime");
        self.u_color = program.get_uniform_location(gl, "uColor");
        self.u_model_matrix = program.get_uniform_location(gl, "uModel");
        self.u_view_matrix = program.get_uniform_location(gl, "uView");
        self.u_projection_matrix = program.get_uniform_location(gl, "uProj");
        self.program = program;
    }

    /// Handle screen resizes.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.set_width_and_height(width, height);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.uniform_matrix4fv_with_f32_array(
            self.u_projection_matrix.as_ref(),
            false,
            self.camera.projection(),
        );
    }

    /// Initialise the uniforms for the current [`Shader`].
    pub fn init_shader(&mut self) {
        let gl = &self.gl;
        self.program.bind(gl);

        gl.uniform1f(self.u_time.as_ref(), 0.0);
        gl.uniform4f(self.u_color.as_ref(), 1.0, 1.0, 1.0, 1.0);
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            crate::Mat4::identity().as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            self.camera.transform.matrix_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.u_projection_matrix.as_ref(),
            false,
            self.camera.projection(),
        );
    }

    /// Add a [`Texture`] to the [`Renderer`].
    ///
    /// The renderer stores [`Texture`]s that can be retreived later, via a string slice.
    pub fn add_texture(&mut self, key: &'static str, texture: Texture) {
        self.textures.insert(key, Rc::new(texture));
    }

    /// Use the requested [`Texture`].
    ///
    /// Sets the currently bound [`Texture`] to the one that matches the key. If no such texture is found, a default MAGENTA one is found.
    pub fn use_texture(&self, key: &str) {
        let gl = &self.gl;
        if let Some(texture) = self.textures.get(key) {
            texture.bind(gl);
        } else {
            self.textures.get("MAGENTA").unwrap().bind(gl);
        }
    }

    /// Get the requested [`Texture`], or MAGENTA if none is found.
    pub fn get_texture(&mut self, key: &str) -> Rc<Texture> {
        Rc::clone(
            self.textures
                .get(key)
                .unwrap_or_else(|| self.textures.get("MAGENTA").unwrap()),
        )
    }

    /// Clear the batch queue and start a new batch.
    pub fn begin_draw(&mut self) {
        let gl = &self.gl;

        self.batches.clear();

        let mesh = Mesh::new(
            gl,
            Vec::with_capacity(MAX_BATCH_VERTICES as usize),
            Vec::with_capacity(MAX_BATCH_INDICES as usize),
        );

        self.batches.push(mesh);
    }

    /// Add a [`Quad`] to the batching queue.
    pub fn add_quad(&mut self, quad: &Quad) {
        let gl = &self.gl;

        // Get last batch. This should never be empty becase begin_draw should have been called before.
        let mut batch = self
            .batches
            .last_mut()
            .expect("Batch list empty. Check if begin_draw was called before.");
        if batch.vertices.len() + 4 > MAX_BATCH_VERTICES as usize {
            let mesh = Mesh::new(
                gl,
                Vec::with_capacity(MAX_BATCH_VERTICES as usize),
                Vec::with_capacity(MAX_BATCH_INDICES as usize),
            );
            self.batches.push(mesh);

            batch = self.batches.last_mut().unwrap();
        }

        let last = batch.vertices.len() as u32;
        batch.vertices.append(&mut quad.get_vertices());
        let mut indices = vec![last, last + 2, last + 1, last, last + 3, last + 2];
        batch.indices.append(&mut indices);
    }

    /// Begin a new layer.
    ///
    /// A new mesh is added to the batches and subsequent calls are made on this layer.
    pub fn begin_layer(&mut self) {
        let gl = &self.gl;
        let mesh = Mesh::new(
            gl,
            Vec::with_capacity(MAX_BATCH_VERTICES as usize),
            Vec::with_capacity(MAX_BATCH_INDICES as usize),
        );

        self.batches.push(mesh);
    }

    /// Remove the last layer.
    pub fn delete_layer(&mut self) -> Option<Mesh> {
        self.batches.pop()
    }

    /// Draw the current layer.
    pub fn draw_layer(&self) {
        let gl = &self.gl;
        if let Some(batch) = self.batches.last() {
            batch.setup(gl);
            gl.draw_elements_with_i32(
                GL::TRIANGLES,
                batch.indices.len() as i32,
                GL::UNSIGNED_INT,
                0,
            );
        }
    }

    /// Draw all batched geometry.
    pub fn end_draw(&mut self) {
        let gl = &self.gl;
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            self.camera.transform.matrix_slice(),
        );
        self.program.bind(gl);
        for batch in self.batches.iter() {
            batch.setup(gl);
            gl.draw_elements_with_i32(
                GL::TRIANGLES,
                batch.indices.len() as i32,
                GL::UNSIGNED_INT,
                0,
            );
        }
    }

    /// Clear the screen with a given Color.
    pub fn clear(&mut self, color: [f32; 4]) {
        let gl = &self.gl;
        gl.clear_color(color[0], color[1], color[2], color[3]);
        gl.clear(GL::COLOR_BUFFER_BIT);
    }

    /// Add a [`Component`] to the [`Renderer`].
    pub fn add_component(&mut self, name: &'static str, component: Box<dyn Component>) {
        self.components.insert(name, component);
    }

    /// (Re)Initialize the [`Components`](Component) of the [`Renderer`].
    pub fn init_components(&mut self) {
        for component in self.components.values_mut() {
            component.init()
        }
    }

    /// Update the [`Components`](Component) of the [`Renderer`].
    pub fn update_components(&mut self, delta_time: f32) {
        for component in self.components.values_mut() {
            component.update(delta_time)
        }
    }

    /// Get a [`Components`](Component) using a key, and ty to cast it to a given type.
    pub fn get_component<T: 'static + Component>(&self, key: &'static str) -> Result<&T, String> {
        self.components
            .get(key)
            .unwrap()
            .as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| String::from("Could not cast to type."))
    }

    /// Get a mutable [`Components`](Component) using a key, and ty to cast it to a given type.
    pub fn get_mut_component<T: 'static + Component>(
        &mut self,
        key: &'static str,
    ) -> Result<&mut T, String> {
        self.components
            .get_mut(key)
            .unwrap()
            .as_mut_any()
            .downcast_mut::<T>()
            .ok_or_else(|| String::from("Could not cast to type."))
    }

    /// Draw the [`Components`](Component) of the [`Renderer`].
    pub fn draw_components(&mut self) {
        let gl = &self.gl;
        let mut layers: Vec<Vec<Quad>> = self
            .components
            .values()
            .filter_map(|component| component.get_quads())
            .collect();
        for layer in layers.iter_mut() {
            let mut mesh = Mesh::new(
                gl,
                Vec::with_capacity(layer.len() * 4),
                Vec::with_capacity(layer.len() * 6),
            );
            for (id, quad) in layer.iter_mut().enumerate() {
                let last: u32 = id as u32 * 4;
                mesh.vertices.append(&mut quad.get_vertices());
                let mut indices = vec![last, last + 2, last + 1, last, last + 3, last + 2];
                mesh.indices.append(&mut indices);
            }
            mesh.setup(gl);
            gl.draw_elements_with_i32(
                GL::TRIANGLES,
                mesh.indices.len() as i32,
                GL::UNSIGNED_INT,
                0,
            );
        }
    }
}

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Moon Engine

pub mod camera;
pub mod collider;
pub mod component;
pub mod gl;
pub mod input;
pub mod math;
pub mod mesh;
pub mod particle;
pub mod renderer;
pub mod shader;
pub mod texture;
pub mod transform;
pub mod ui;
pub mod utils;
pub mod web;

use wasm_bindgen::prelude::*;

use camera::Camera;
use gl::GL;
use input::InputManager;
pub use math::*;
use particle::ParticleSystem;
use renderer::Renderer;
use shader::Shader;
use texture::Texture;
use transform::Transform;
use utils::set_panic_hook;
use web::Canvas;

use crate::input::Key;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The [`Application`] struct acts as the communicator between the browser and the game logic. It consists of calls made from JavaScript.
#[wasm_bindgen]
pub struct Application {
    renderer: Renderer,
    input: InputManager,
}

impl Default for Application {
    fn default() -> Self {
        // Initialize the JS panic hook
        set_panic_hook();
        Self {
            renderer: Renderer::default(),
            input: InputManager::new(),
        }
    }
}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Application {
    /// Initilize a default [`Application`].
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set up data before render loop.
    #[wasm_bindgen]
    pub fn init(&mut self) {
        let renderer = &mut self.renderer;

        renderer
            .gl
            .blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        renderer.gl.enable(GL::BLEND);
        // Initialize the default Shader
        renderer.init_shader();

        let u_tex0 = renderer.program.get_uniform_location(&renderer.gl, "uTex0");
        renderer.gl.uniform1i(u_tex0.as_ref(), 0);

        renderer.add_texture("TILEMAP", Texture::new_with_texture_id(&renderer.gl, 0));
        renderer.add_texture("SHREK", Texture::new_with_texture_id(&renderer.gl, 1));

        renderer.use_texture("WHITE");

        let simple = ParticleSystem::new_from_emission_and_position(
            particle::ParticleProps::default(),
            0.0,
            0.0,
        );
        renderer.add_component("DEFAULT", Box::new(simple));

        let fire = ParticleSystem::new_from_emission_and_position(
            particle::ParticleProps::fire(),
            -5.0,
            0.0,
        );
        renderer.add_component("FIRE", Box::new(fire));

        let smoke = ParticleSystem::new_from_emission_and_position(
            particle::ParticleProps::smoke(),
            5.0,
            0.0,
        );
        renderer.add_component("SMOKE", Box::new(smoke));

        renderer.init_components();
    }

    /// Called when window gets resized.
    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.renderer.resize(width, height);
    }

    /// Called when a keyboard input event is generated.
    #[wasm_bindgen]
    pub fn input(&mut self, key: &str, is_down: bool) {
        self.input.set_key_state(key, is_down);
    }

    /// Handles Mouse movement.
    #[wasm_bindgen]
    pub fn mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        let (x, y) = self
            .renderer
            .camera
            .screen_to_world_coordinates(mouse_x as f32, mouse_y as f32);
        self.input.mouse_position = Vec2::new(x, y);
    }

    /// Renders a new frame.
    ///
    /// Called every frame, and draws its output onto the [Canvas](web_sys::HtmlCanvasElement).
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: u32) {
        let renderer = &mut self.renderer;
        let delta_time = delta_time as f32 / 1000.0;

        renderer.clear([0.5, 0.2, 0.3, 1.0]);

        if self.input.consume_key_state(Key::R) {
            renderer.init_components();
        }
        if self.input.consume_key_state(Key::Num1) {
            renderer
                .get_mut_component::<ParticleSystem>("FIRE")
                .unwrap()
                .toggle_alive();
        }
        if self.input.consume_key_state(Key::Num2) {
            renderer
                .get_mut_component::<ParticleSystem>("DEFAULT")
                .unwrap()
                .toggle_alive();
        }
        if self.input.consume_key_state(Key::Num3) {
            renderer
                .get_mut_component::<ParticleSystem>("SMOKE")
                .unwrap()
                .toggle_alive();
        }
        let horizontal =
            self.input.get_key_state(Key::D) as i32 - self.input.get_key_state(Key::A) as i32;
        let vertical =
            self.input.get_key_state(Key::S) as i32 - self.input.get_key_state(Key::W) as i32;

        let simple = renderer
            .get_mut_component::<ParticleSystem>("DEFAULT")
            .unwrap();

        simple.transform.position = self.input.mouse_position;

        let smoke = renderer
            .get_mut_component::<ParticleSystem>("SMOKE")
            .unwrap();

        if smoke.alive {
            smoke.transform.position +=
                Vec2::new(horizontal as f32 * delta_time, vertical as f32 * delta_time);
        }

        renderer.update_components(delta_time);

        renderer.draw_components();

        // self.renderer.begin_layer();
        // self.renderer.add_quad(Quad::default());
        // self.renderer.use_texture("MAGENTA");
        // self.renderer.draw_layer();
        // self.renderer.delete_layer();
    }
}

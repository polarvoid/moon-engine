//! The [`Texture`] and [`SubTexture`] structs.

use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGlTexture;

use crate::gl::Bind;
use crate::Color32;
use crate::{gl, GL};

/// A [`Texture`] stores an Image that can be used while rendering, or to store data.
#[derive(Debug)]
pub struct Texture {
    texture: Option<WebGlTexture>,
    /// Width of the [`Texture`].
    pub width: u32,
    /// Height of the [`Texture`].
    pub height: u32,
    /// Slot the [`Texture`] will occupy.
    pub slot: u32,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            texture: None,
            width: 1,
            height: 1,
            slot: 0,
        }
    }
}

impl Bind for Texture {
    fn bind(&self, gl: &GL) {
        gl.active_texture(GL::TEXTURE0 + self.slot);
        gl.bind_texture(GL::TEXTURE_2D, self.texture.as_ref());
    }
    fn unbind(&self, gl: &GL) {
        gl.active_texture(GL::TEXTURE0 + self.slot);
        gl.bind_texture(GL::TEXTURE_2D, None);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let gl = gl::get_context();

        gl.delete_texture(self.texture.as_ref());
    }
}

impl Texture {
    /// Create a new [`Texture`] using an [`HtmlImageElement`].
    pub fn new(gl: &GL, image: &HtmlImageElement) -> Self {
        let (width, height) = (image.width(), image.height());

        let texture = gl.create_texture();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
        // Flip the Y-axis so the image displays the right way up
        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            image,
        )
        .expect("Failed to load texture");
        gl.generate_mipmap(GL::TEXTURE_2D);

        Self {
            width,
            height,
            texture,
            ..Default::default()
        }
    }

    /// Create a new [`Texture`] from an [`HtmlImageElement`] with an given element ID.
    pub fn new_with_element_id(gl: &GL, image_src: &str) -> Self {
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let image = document
            .get_element_by_id(image_src)
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        Self::new(gl, &image)
    }

    /// Create a new [`Texture`] from an [`HtmlImageElement`] with an element ID in the format **textureXX** where *XX* is a number.
    pub fn new_with_texture_id(gl: &GL, count: u32) -> Self {
        Self::new_with_element_id(gl, &format!("texture{}", count))
    }

    /// Create a new [`Texture`] using a slice of [`u8`]s.
    pub fn new_from_pixels(gl: &GL, width: u32, height: u32, pixels: &[u8]) -> Self {
        assert!(pixels.len() == (width * height * 4) as usize);
        let texture = gl.create_texture();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            width as i32,
            height as i32,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            Some(pixels),
        )
        .expect("Failed to generate texture");
        gl.generate_mipmap(GL::TEXTURE_2D);

        Self {
            width,
            height,
            texture,
            ..Default::default()
        }
    }

    /// A colored [`Texture`].
    ///
    /// Create a single pixel sized [`Texture`] with the specified [`Color32`].
    pub fn colored(gl: &GL, color: Color32) -> Self {
        Self::new_from_pixels(gl, 1, 1, &<[u8; 4]>::from(color))
    }

    /// A fully-white [`Texture`].
    pub fn white(gl: &GL) -> Self {
        Self::colored(gl, Color32::WHITE)
    }

    /// A black and white checkerboard [`Texture`].
    pub fn checkerboard(gl: &GL) -> Self {
        Self::checkerboard_colored(gl, Color32::WHITE, Color32::BLACK)
    }

    /// A checkerboard [`Texture`] with two [`Color32`]s.
    pub fn checkerboard_colored(gl: &GL, color1: Color32, color2: Color32) -> Self {
        let size = 8;
        let mut pixels = Vec::<u8>::with_capacity(size * size);
        for x_offset in 0..size {
            for y_offset in 0..size {
                let color = if (x_offset + y_offset) % 2 == 0 {
                    color1
                } else {
                    color2
                };
                pixels.append(&mut Vec::from(color));
            }
        }
        Self::new_from_pixels(gl, size as u32, size as u32, &pixels)
    }
}

/// A [`SubTexture`] is a part of a full [`Texture`].
///
/// It stores the UV co-ordinates of the part of the Texture it occupies.
#[derive(Debug, Clone)]
pub struct SubTexture {
    texture: Option<Rc<Texture>>,
    min: [f32; 2],
    max: [f32; 2],
}

impl Default for SubTexture {
    fn default() -> Self {
        Self {
            texture: None,
            min: [0.0, 0.0],
            max: [1.0, 1.0],
        }
    }
}

impl SubTexture {
    /// Create a new [`SubTexture`] with the full UV co-ordinates.
    pub fn new(texture: Rc<Texture>) -> Self {
        Self {
            texture: Some(texture),
            ..Default::default()
        }
    }

    /// Create a new [`SubTexture`] with the given UV co-ordinates.
    pub fn new_with_coords(texture: Rc<Texture>, uv: Color32) -> Self {
        Self {
            texture: Some(texture),
            min: [uv.x(), uv.z()],
            max: [uv.y(), uv.w()],
        }
    }

    /// Get the UV co-ordinates as an array of four two-component [`f32`].
    pub fn get_uv_coords(&self) -> [[f32; 2]; 4] {
        let (min, max) = (self.min, self.max);
        [
            [min[0], min[1]],
            [min[0], max[1]],
            [max[0], max[1]],
            [max[0], min[1]],
        ]
    }

    /// Create a [`Vec`] of [`SubTexture`]s from a sprite sheet.
    ///
    /// Uses the number of cells horizontally and vertically to devide the cells.
    pub fn create_tiles_from_spritesheet(
        texture: Rc<Texture>,
        horizontal_cells: u32,
        vertical_cells: u32,
    ) -> Vec<SubTexture> {
        assert!(horizontal_cells > 0 && vertical_cells > 0);
        let mut tiles: Vec<SubTexture> = Vec::new();
        for cell_x in 0..horizontal_cells {
            for cell_y in 0..vertical_cells {
                let uv = Color32(
                    cell_x as f32 / horizontal_cells as f32,
                    (cell_x + 1) as f32 / horizontal_cells as f32,
                    cell_y as f32 / vertical_cells as f32,
                    (cell_y + 1) as f32 / vertical_cells as f32,
                );
                let tile = SubTexture::new_with_coords(Rc::clone(&texture), uv);
                tiles.push(tile);
            }
        }
        tiles
    }
}

impl Bind for SubTexture {
    fn bind(&self, gl: &GL) {
        if let Some(texture) = &self.texture {
            texture.bind(gl);
        }
    }
    fn unbind(&self, gl: &GL) {
        gl.bind_texture(GL::TEXTURE_2D, None);
    }
}

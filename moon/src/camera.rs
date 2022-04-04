//! The [`Camera`] struct.

use crate::transform::Transform;
use crate::Mat4;
use crate::Ortho;
use crate::Vec3;

/// The 'X' component at the left and right edges of the screen
pub const FIXED_WIDTH: f32 = 20.0;
/// Calculate the height from the `FIXED_WIDTH` to maintain 16:9 Aspect ratio
pub const FIXED_HEIGHT: f32 = FIXED_WIDTH / 1.77;

/// A [`Camera`] represents a Virtual Camera, that has a view and Orthographic projection matrices
#[derive(Debug)]
pub struct Camera {
    /// [`Transform`] for the Camera
    pub transform: Transform,
    orthographic: Ortho,
    width: f32,
    height: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::new(),
            width: FIXED_WIDTH,
            height: FIXED_HEIGHT,
            orthographic: Ortho::new(
                -FIXED_WIDTH / 2.0,
                FIXED_WIDTH / 2.0,
                FIXED_HEIGHT / 2.0,
                -FIXED_HEIGHT / 2.0,
                0f32,
                1000.0f32,
            ),
        }
    }
}

impl Camera {
    /// Create a new `Camera` with default values.
    pub fn new() -> Self {
        Default::default()
    }
    /// Create a new `Camera` with an initial position.
    pub fn with_position(position: Vec3) -> Self {
        Self {
            transform: Transform::new_with_position(position),
            ..Default::default()
        }
    }
    /// Create a new `Camera` with an initial transform.
    pub fn with_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..Default::default()
        }
    }
    /// Create a new `Camera` with an initial width and height.
    pub fn with_width_and_height(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            orthographic: Ortho::new(
                -width / 2.0,
                width / 2.0,
                height / 2.0,
                -height / 2.0,
                0f32,
                1000.0f32,
            ),
            ..Default::default()
        }
    }

    /// Set the width and height of the camera plane, and update the Projection Matrix to match.
    pub fn set_width_and_height(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Return the Projection Matrix of the `Camera` as a slice of `f32` so it can be used by WebGL.
    pub fn projection(&self) -> &[f32] {
        self.orthographic.as_matrix().as_slice()
    }

    /// Return the calculated and combined view-projection matrix as a [`Mat4`].
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.transform.matrix() * self.orthographic.as_matrix()
    }

    /// Get a position in screen co-ordinates to a range within the world.
    ///
    /// This works by first converting it into a `-1.0 to 1.0` range, and then multiplying its components by the [`FIXED_WIDTH`] and [`FIXED_HEIGHT`].
    pub fn screen_to_world_coordinates(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let clipped_x = screen_x / self.width - 0.5;
        let clipped_y = screen_y / self.height - 0.5;

        (clipped_x * FIXED_WIDTH, clipped_y * FIXED_HEIGHT)
    }
}

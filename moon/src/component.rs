//! The [`Component`] trait.

use std::any::Any;

use crate::renderer::Quad;

/// The [`Component`] trait
pub trait Component {
    /// Initializes a [`Component`].
    fn init(&mut self) {}
    /// Called once each frame.
    ///
    /// The time between frames is also provided.
    fn update(&mut self, _delta_time: f32) {}

    /// Get the [`Component`]'s drawable [`Quad`].
    fn get_quads(&self) -> Option<Vec<Quad>> {
        None
    }

    /// Borrow the [`Component`] as a [`&mut dyn Any`](Any).
    fn as_any(&self) -> &dyn Any;

    /// Borrow the [`Component`] as a [`&dyn Any`](Any).
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

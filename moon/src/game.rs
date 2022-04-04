//! Game-specific code for the example game.
use std::{collections::BTreeMap, rc::Rc};

use crate::{Vec2, renderer::Quad, texture::{SubTexture, Texture}, input::{InputManager, Key}, Color32, collider::{Collider, AABB}};

/// A [`Tile`] is a single cell in a [`TileMap`].
pub struct Tile {
    /// Name for the [`Tile`].
    pub name: &'static str,
    pub image: Rc<SubTexture>,
    pub collider: Option<AABB>
}

impl Default for Tile {
    fn default() -> Self {
        Self { 
            name: "Unknown Tile", 
            image: Default::default(),
            collider: None,
        }
    }
}

impl Tile {
    /// Create a new [`Tile`] using the given co-ordinates.
    pub fn new_with_coords(name: &'static str, uv: Color32) -> Self {
        Self {
            name,
            image: Rc::new(SubTexture::new_with_coords(Rc::new(Texture::default()), uv)),
            collider: None,
        }
    }

    /// Create a new [`Tile`] using the given [`SubTexture`].
    pub fn new_from_subtexture(name: &'static str, image: Rc<SubTexture>) -> Self {
        Self {
            name,
            image,
            collider: None,
        }
    }
}
pub struct TileMap {
    width: u32,
    height: u32,
    tiles: BTreeMap<u32, Tile>
}

/// States for the [`Player`].
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerState {
    /// Right-facing Idle state.
    Idle,
    /// Left-facing Idle state.
    IdleLeft,
    /// [`Player`] is Jumping.
    Jumping,
    /// [`Player`] is Jumping, while looking left.
    JumpingLeft,
}

/// The [`Player`] is the main character in the game.
pub struct Player {
    state: PlayerState,
    position: Vec2,
    velocity: Vec2,
    is_grounded: bool,
    is_backward: bool,
    is_jumping: bool,
    sprites: BTreeMap<PlayerState, SubTexture>
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Idle,
            position: Default::default(), 
            is_backward: false,
            is_jumping: false,
            sprites: {
                use std::rc::Rc;
                let mut sprites = BTreeMap::new();
                let texture = Rc::new(Texture::default());

                let uv = crate::Color32(0.01, 0.1, 1.0/6.0, 2.0/6.0);
                let sprite = SubTexture::new_with_coords(Rc::clone(&texture), uv);
                sprites.insert(PlayerState::Idle, sprite);

                let uv = crate::Color32(0.1, 0.01, 1.0/6.0, 2.0/6.0);
                let sprite = SubTexture::new_with_coords(Rc::clone(&texture), uv);
                sprites.insert(PlayerState::IdleLeft, sprite);

                sprites
            },
            velocity: Default::default(),
            is_grounded: false,
        }
    }
}

impl Player {
    /// Handle [`InputManager`] states.
    pub fn handle_input(&mut self, input: &InputManager) {
        let horizontal_movement = input.get_key_state(Key::D) as i32 - input.get_key_state(Key::A) as i32;
        self.position.x += horizontal_movement as f32 / 100.0;
        self.calculate_state();
    }

    /// Update the [`Player`].
    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }

    /// Calculate the [`PlayerState`] of the [`Player`].
    fn calculate_state(&mut self) {
        if self.is_jumping {
            self.state = if self.is_backward {
                PlayerState::JumpingLeft
            } else {
                PlayerState::Jumping
            }
        } else {
            self.state = if self.is_backward {
                PlayerState::IdleLeft
            } else {
                PlayerState::Idle
            }
        }
    }

    /// Get the current [`SubTexture`] based on the [`PlayerState`] of the [`Player`].
    pub fn get_sprite(&self) -> &SubTexture {
        self.sprites.get(&self.state).unwrap_or(self.sprites.get(&PlayerState::Idle).unwrap())
    }

    /// Get a [`Quad`] that can be submitted to the [`Renderer`](crate::renderer::Renderer).
    pub fn quad(&self) -> Quad {
        Quad::new_from_position_and_size_and_sprite(self.position.x, self.position.y, 1.0, 1.0, self.get_sprite())
    }
}
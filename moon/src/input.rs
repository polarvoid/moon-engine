//! The [`InputManager`] struct.

use std::collections::BTreeSet;

use crate::Vec2;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
/// A [`Key`] is used to identify the keys on a keyboard.
pub enum Key {
    /// The `Space` key.
    Space,
    /// The `A` key.
    A,
    /// The `D` key.
    D,
    /// The `S` key.
    S,
    /// The `W` key.
    W,
    /// An Unknown Key.
    Unknown,
}

impl From<&str> for Key {
    fn from(key: &str) -> Self {
        match key {
            "Space" => Key::Space,
            "a" | "A" => Key::A,
            "d" | "D" => Key::D,
            "s" | "S" => Key::S,
            "w" | "W" => Key::W,
            _ => Key::Unknown
        }
    }
}

/// A store for Input-related data.
///
/// The [`InputManager`] stores and handles the current input states.
///
/// # Examples
/// ```
/// use moon::input::InputManager;
///
/// let mut input = InputManager::new();
///
/// input.key_down(b'w');
///
/// assert!(input.get_key_state(b'w'));
/// ```
#[derive(Default)]
pub struct InputManager {
    /// Set of Keyboard key states.
    ///
    /// If a key is present, then it is being pressed, and otherwise it is not.
    keyboard_states: BTreeSet<Key>,
    /// Position of the Mouse.
    ///
    /// The Screen-Space position of the Mouse as a [`Vec2`].
    pub mouse_position: Vec2,
}

impl InputManager {
    /// Default [`InputManager`] instance.
    ///
    /// Creates a new [`InputManager`] with default keyboard and mouse input states.
    pub fn new() -> Self {
        Default::default()
    }

    /// Key Down State.
    ///
    /// Sets the key in the [`BTreeSet`].
    pub fn key_down(&mut self, key: Key) {
        self.keyboard_states.insert(key);
    }

    /// Key Up State.
    ///
    /// Resets the key in the [`BTreeSet`].
    pub fn key_up(&mut self, key: Key) {
        self.keyboard_states.remove(&key);
    }

    /// Set's key state using a string
    pub fn set_key_state(&mut self, key: &str, is_down: bool) {
        let key = Key::from(key);
        if is_down {
            self.key_down(key);
        } else {
            self.key_up(key);
        }
    }

    /// Get the state of a key as a [`bool`].
    ///
    /// Returns true if the key is currently pressed, or false.
    pub fn get_key_state(&self, key: Key) -> bool {
        self.keyboard_states.contains(&key)
    }

    /// Set the mouse position.
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }
}

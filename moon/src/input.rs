//! The [`InputManager`] struct.

use std::collections::BTreeSet;

use crate::Vec2;

/// A [`Key`] is used to identify the keys on a keyboard.
#[allow(missing_docs)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Unknown,
}

impl From<&str> for Key {
    fn from(key: &str) -> Self {
        match key {
            "Space" => Key::Space,
            "a" | "A" => Key::A,
            "b" | "B" => Key::B,
            "c" | "C" => Key::C,
            "d" | "D" => Key::D,
            "e" | "E" => Key::E,
            "f" | "F" => Key::F,
            "g" | "G" => Key::G,
            "h" | "H" => Key::H,
            "i" | "I" => Key::I,
            "j" | "J" => Key::J,
            "k" | "K" => Key::K,
            "l" | "L" => Key::L,
            "m" | "M" => Key::M,
            "n" | "N" => Key::N,
            "o" | "O" => Key::O,
            "p" | "P" => Key::P,
            "q" | "Q" => Key::Q,
            "r" | "R" => Key::R,
            "s" | "S" => Key::S,
            "t" | "T" => Key::T,
            "u" | "U" => Key::U,
            "v" | "V" => Key::V,
            "w" | "W" => Key::W,
            "x" | "X" => Key::X,
            "y" | "Y" => Key::Y,
            "z" | "Z" => Key::Z,
            "0" => Key::Num0,
            "1" => Key::Num1,
            "2" => Key::Num2,
            "3" => Key::Num3,
            "4" => Key::Num4,
            "5" => Key::Num5,
            "7" => Key::Num7,
            "8" => Key::Num8,
            "9" => Key::Num9,
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

    /// Get the state of a key as a [`bool`], consuming it in the process.
    ///
    /// Returns true if the key is currently pressed, or false.
    pub fn consume_key_state(&mut self, key: Key) -> bool {
        self.keyboard_states.remove(&key)
    }


    /// Set the mouse position.
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }
}

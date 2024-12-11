#![allow(dead_code)]

use crate::primitives::{Point2D, Vector2D};
use minifb::{Key, Window};
use std::collections::HashSet;

#[derive(Debug)]

pub struct InputHandler {
    current_mouse_button_states: [bool; 3],
    previous_mouse_button_states: [bool; 3],

    mouse_position: Point2D,
    previous_mouse_position: Point2D,

    current_keys: HashSet<Key>,
    previous_keys: HashSet<Key>,
}

// is_* for boolean state checks
// get_* for retrieving values
// was_* for checking previous state
// has_* for checking if something occurred since last frame

impl InputHandler {
    pub fn new() -> Self {
        let current_mouse_button_states = [false, false, false];
        let previous_mouse_button_states = [false, false, false];

        let mouse_position: Point2D = Point2D::new(0.0, 0.0);
        let previous_mouse_position: Point2D = Point2D::new(0.0, 0.0);

        let current_keys: HashSet<Key> = HashSet::new();
        let previous_keys: HashSet<Key> = HashSet::new();

        Self {
            current_mouse_button_states,
            previous_mouse_button_states,
            mouse_position,
            previous_mouse_position,
            current_keys,
            previous_keys,
        }
    }
    pub fn update(&mut self, window: &Window) {
        // safe previous keys
        self.previous_keys = self.current_keys.clone();
        // Update current keys
        self.current_keys = window.get_keys().into_iter().collect();

        // Update mouse position if available
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            self.previous_mouse_position = self.mouse_position;
            self.mouse_position = Point2D::new(x, y);
        }

        // safe previous mouse_button_states
        self.previous_mouse_button_states = self.current_mouse_button_states;

        // Update mouse button states
        self.current_mouse_button_states = [
            window.get_mouse_down(minifb::MouseButton::Left),
            window.get_mouse_down(minifb::MouseButton::Middle),
            window.get_mouse_down(minifb::MouseButton::Right),
        ];
    }

    // For instant checks (single frame)
    pub fn is_mouse_button_pressed(&self, button_number: usize) -> bool {
        self.current_mouse_button_states[button_number.clamp(0, 2)]&& !self.previous_mouse_button_states[button_number.clamp(0, 2)]
        // Returns true only on the frame the key is first pressed
    }

    // For continuous state checks
    pub fn is_mouse_button_down(&self, button_number: usize) -> bool {
        self.current_mouse_button_states[button_number.clamp(0, 2)] && self.previous_mouse_button_states[button_number.clamp(0, 2)]

        // Returns true while key is held down
    }

    // For release checks
    pub fn is_mouse_button_released(&self, button_number: usize) -> bool {
        !self.current_mouse_button_states[button_number.clamp(0, 2)] && self.previous_mouse_button_states[button_number.clamp(0, 2)]
        // Returns true only on the frame the key is released
    }

    // For mouse movement
    // current position
    pub fn get_mouse_position(&self) -> Point2D {
        self.mouse_position
    }

    // Movement since last frame
    pub fn get_mouse_delta(&self) -> Vector2D {
        self.get_mouse_position().sub_p(self.get_mouse_position())
    }

    // For instant checks (single frame)
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.current_keys.contains(&key) && !self.previous_keys.contains(&key)
        // Returns true only on the frame the key is first pressed
    }

    // For continuous state checks
    pub fn is_key_down(&self, key: Key) -> bool {
        self.current_keys.contains(&key) && self.previous_keys.contains(&key)

        // Returns true while key is held down
    }

    // For release checks
    pub fn is_key_released(&self, key: Key) -> bool {
        !self.current_keys.contains(&key) && self.previous_keys.contains(&key)
        // Returns true only on the frame the key is released
    }
}

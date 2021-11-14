use std::collections::HashSet;
use cgmath::{Point2, Vector2};
use log::info;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::mouse::{MouseButton, MouseState};

pub struct InputState {
    pressed_keys : HashSet<Keycode>,
    previous_mouse_position : Point2<i32>,
    mouse_position : Point2<i32>,
    mouse_left_button_pressed : bool,
    mouse_right_button_pressed : bool,
}

impl InputState {
    pub fn new() -> Box<Self> {
        info!("Initializing input manager...");
        return Box::new(Self {
            pressed_keys: HashSet::new(),
            previous_mouse_position: Point2::new(0, 0),
            mouse_position: Point2::new(0, 0),
            mouse_left_button_pressed: false,
            mouse_right_button_pressed: false
        });
    }
}

//kbd
impl InputState {
    pub fn handle_keyboard(&mut self, kbd_state: KeyboardState) {
        self.pressed_keys = kbd_state.pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
    }


    pub fn is_key_pressed(&self, key_code: Keycode) -> bool {
        return self.pressed_keys.contains(&key_code);
    }
    pub fn is_key_down(&self) -> bool {
        return self.is_key_pressed(Keycode::Down);
    }
    pub fn is_key_up(&self) -> bool {
        return self.is_key_pressed(Keycode::Up);
    }
    pub fn is_key_left(&self) -> bool {
        return self.is_key_pressed(Keycode::Left);
    }
    pub fn is_key_right(&self) -> bool {
        return self.is_key_pressed(Keycode::Right);
    }
}

//mouse
impl InputState {
    pub fn handle_mouse(&mut self, mouse_state: MouseState) {
        self.mouse_right_button_pressed = mouse_state.is_mouse_button_pressed(MouseButton::Right);
        self.mouse_left_button_pressed = mouse_state.is_mouse_button_pressed(MouseButton::Left);
        self.previous_mouse_position = self.mouse_position;
        self.mouse_position = Point2 { x: mouse_state.x(), y: mouse_state.y() };
    }

    pub fn get_mouse_position(&self) -> Point2<i32> {
        return self.mouse_position;
    }
    pub fn get_mouse_movemen_vector(&self) -> Vector2<i32> {
        return Vector2 {
            x: self.mouse_position.x - self.previous_mouse_position.x,
            y: self.mouse_position.y - self.previous_mouse_position.y,
        };
    }

    pub fn is_mouse_left_button_pressed(&self) -> bool {
        return self.mouse_left_button_pressed;
    }
    pub fn is_mouse_right_button_pressed(&self) -> bool {
        return self.mouse_right_button_pressed;
    }

}


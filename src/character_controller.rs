use cgmath::{InnerSpace, Matrix3, Point3, Vector3, Zero};
use std::collections::HashSet;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

const SPEED: f32 = 1.0;
const MOUSE_SENSITIVITY: f32 = 0.01;
const MAX_Y_DEGREES: f32 = 80.0;

pub struct CharacterController {
    position: Point3<f32>,
    rotation_x_degrees: f32,
    rotation_y_degrees: f32,
    pressed_keys: HashSet<KeyCode>,
}

impl CharacterController {
    pub fn new() -> Self {
        CharacterController {
            position: Point3::new(0.0, 0.0, 2.0),
            rotation_x_degrees: 0.0,
            rotation_y_degrees: 0.0,
            pressed_keys: HashSet::new(),
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => self.update_pressed_keys(code, is_pressed),
        }
    }

    fn update_pressed_keys(&mut self, code: KeyCode, is_pressed: bool) {
        if is_pressed {
            self.pressed_keys.insert(code);
        } else {
            self.pressed_keys.remove(&code);
        }
    }

    pub fn update_and_get_position(&mut self, delta: f32) -> Point3<f32> {
        use KeyCode::*;

        let mut horizontal_direction = Vector3::zero();
        let mut vertical_direction = Vector3::zero();
        for key in &self.pressed_keys {
            horizontal_direction += match key {
                KeyW => -Vector3::unit_z(),
                KeyS => Vector3::unit_z(),
                KeyA => -Vector3::unit_x(),
                KeyD => Vector3::unit_x(),
                _ => Vector3::zero(),
            };
            vertical_direction += match key {
                Space => Vector3::unit_y(),
                ControlLeft => -Vector3::unit_y(),
                _ => Vector3::zero(),
            };
        }

        let speed = delta
            * if self.pressed_keys.contains(&ShiftLeft) {
                SPEED * 2.0
            } else {
                SPEED
            };

        if horizontal_direction != Vector3::zero() {
            self.position += self.get_rotation_matrix() * horizontal_direction.normalize() * speed;
        }

        self.position += vertical_direction * speed;
        self.position
    }

    pub fn handle_cursor(&mut self, delta: (f64, f64)) {
        self.rotation_x_degrees =
            (self.rotation_x_degrees + delta.0 as f32 * MOUSE_SENSITIVITY) % 360.0;
        self.rotation_y_degrees = (self.rotation_y_degrees + delta.1 as f32 * MOUSE_SENSITIVITY)
            .clamp(-MAX_Y_DEGREES, MAX_Y_DEGREES);
    }

    pub fn get_target_position(&mut self) -> Point3<f32> {
        let starting_direction = Vector3::new(0.0, 0.0, -1.0);
        self.position + self.get_rotation_matrix() * starting_direction
    }

    fn get_rotation_matrix(&self) -> Matrix3<f32> {
        let x_rotation = Matrix3::from_angle_y(-cgmath::Deg(self.rotation_x_degrees));
        let y_rotation = Matrix3::from_angle_x(-cgmath::Deg(self.rotation_y_degrees));
        x_rotation * y_rotation
    }
}

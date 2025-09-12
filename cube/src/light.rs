use raylib::prelude::*;

pub struct Light {
    pub position: Vector3,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vector3, color: Color, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }
}
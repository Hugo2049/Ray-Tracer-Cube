use raylib::prelude::Color;

pub trait ColorOps {
    fn mult_f32(&self, factor: f32) -> Color;
}

impl ColorOps for Color {
    fn mult_f32(&self, factor: f32) -> Color {
        Color::new(
            (self.r as f32 * factor).min(255.0) as u8,
            (self.g as f32 * factor).min(255.0) as u8,
            (self.b as f32 * factor).min(255.0) as u8,
            self.a,
        )
    }
}

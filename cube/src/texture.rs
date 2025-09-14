use raylib::prelude::*;
use std::path::Path;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vector3>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Texture {
            width,
            height,
            pixels: vec![Vector3::zero(); (width * height) as usize],
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let image = Image::load_image(Path::new(path))
            .map_err(|e| format!("Failed to load texture: {}", e))?;
        
        let width = image.width as u32;
        let height = image.height as u32;
        let mut pixels = Vec::with_capacity((width * height) as usize);
        
        for y in 0..height {
            for x in 0..width {
                let color = image.get_pixel(x as i32, y as i32);
                let color_vec = Vector3::new(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0
                );
                pixels.push(color_vec);
            }
        }
        
        Ok(Texture {
            width,
            height,
            pixels,
        })
    }

    pub fn sample(&self, u: f32, v: f32) -> Vector3 {
        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);
        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }
}
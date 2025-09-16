use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use raylib::prelude::*;

pub struct Cube {
    pub center: Vector3,
    pub size: f32,
    pub material: Material,
    pub texture: Option<Image>,
}

impl Cube {
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        Self {
            center,
            size,
            material,
            texture: None,
        }
    }

    pub fn with_texture(center: Vector3, size: f32, material: Material, texture: Image) -> Self {
        Self {
            center,
            size,
            material,
            texture: Some(texture),
        }
    }

    /// Calculate UV coordinates for a point on the cube face
    fn calculate_uv(&self, point: Vector3, normal: Vector3) -> (f32, f32) {
        let local_point = point - self.center;
        let half_size = self.size / 2.0;
        
        // Calculate UV based on which face we hit
        let (u, v) = if normal.x.abs() > 0.9 {
            // X face (left/right)
            if normal.x > 0.0 {
                // Right face (+X): looking from outside, Z maps to U, Y maps to V
                ((-local_point.z + half_size) / self.size, (local_point.y + half_size) / self.size)
            } else {
                // Left face (-X): looking from outside, Z maps to U (flipped), Y maps to V
                ((local_point.z + half_size) / self.size, (local_point.y + half_size) / self.size)
            }
        } else if normal.y.abs() > 0.9 {
            // Y face (top/bottom)
            if normal.y > 0.0 {
                // Top face (+Y): looking from above, X maps to U, Z maps to V (flipped)
                ((local_point.x + half_size) / self.size, (-local_point.z + half_size) / self.size)
            } else {
                // Bottom face (-Y): looking from below, X maps to U, Z maps to V
                ((local_point.x + half_size) / self.size, (local_point.z + half_size) / self.size)
            }
        } else {
            // Z face (front/back)
            if normal.z > 0.0 {
                // Front face (+Z): looking from front, X maps to U, Y maps to V
                ((local_point.x + half_size) / self.size, (local_point.y + half_size) / self.size)
            } else {
                // Back face (-Z): looking from back, X maps to U (flipped), Y maps to V
                ((-local_point.x + half_size) / self.size, (local_point.y + half_size) / self.size)
            }
        };
        
        // Clamp UV coordinates to [0, 1] range
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    /// Sample color from texture at UV coordinates
    fn sample_texture(&mut self, u: f32, v: f32) -> Vector3 {
        if let Some(ref mut texture) = self.texture {
            // Clamp UV coordinates to [0, 1] range
            let u = u.clamp(0.0, 1.0);
            let v = v.clamp(0.0, 1.0);
            
            // Convert UV to pixel coordinates
            let x = ((u * (texture.width - 1) as f32).round() as i32).clamp(0, texture.width - 1);
            let y = ((v * (texture.height - 1) as f32).round() as i32).clamp(0, texture.height - 1);
            
            // Sample the pixel color
            let color = texture.get_color(x, y);
            
            // Convert Color to Vector3 (normalize to [0, 1] range)
            Vector3::new(
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
            )
        } else {
            // Return white if no texture (no modulation)
            Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&mut self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        let half_size = self.size / 2.0;
        let min_bounds = self.center - Vector3::new(half_size, half_size, half_size);
        let max_bounds = self.center + Vector3::new(half_size, half_size, half_size);
        
        // Calculate intersection distances for each axis
        let inv_dir = Vector3::new(
            if ray_direction.x.abs() < 1e-8 { 1e8 } else { 1.0 / ray_direction.x },
            if ray_direction.y.abs() < 1e-8 { 1e8 } else { 1.0 / ray_direction.y },
            if ray_direction.z.abs() < 1e-8 { 1e8 } else { 1.0 / ray_direction.z }
        );
        
        let t1 = (min_bounds.x - ray_origin.x) * inv_dir.x;
        let t2 = (max_bounds.x - ray_origin.x) * inv_dir.x;
        let t3 = (min_bounds.y - ray_origin.y) * inv_dir.y;
        let t4 = (max_bounds.y - ray_origin.y) * inv_dir.y;
        let t5 = (min_bounds.z - ray_origin.z) * inv_dir.z;
        let t6 = (max_bounds.z - ray_origin.z) * inv_dir.z;
        
        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
        
        // No intersection if tmax < 0 (cube is behind ray) or tmin > tmax
        if tmax < 0.0 || tmin > tmax {
            return Intersect::empty();
        }
        
        // Choose the closest positive intersection
        let t = if tmin > 0.0 { tmin } else { tmax };
        
        if t <= 0.0 {
            return Intersect::empty();
        }
        
        let point = *ray_origin + *ray_direction * t;
        
        // Calculate normal based on which face was hit
        let local_point = point - self.center;
        let epsilon = 1e-6;
        
        let normal = if (local_point.x - half_size).abs() < epsilon {
            Vector3::new(1.0, 0.0, 0.0)
        } else if (local_point.x + half_size).abs() < epsilon {
            Vector3::new(-1.0, 0.0, 0.0)
        } else if (local_point.y - half_size).abs() < epsilon {
            Vector3::new(0.0, 1.0, 0.0)
        } else if (local_point.y + half_size).abs() < epsilon {
            Vector3::new(0.0, -1.0, 0.0)
        } else if (local_point.z - half_size).abs() < epsilon {
            Vector3::new(0.0, 0.0, 1.0)
        } else {
            Vector3::new(0.0, 0.0, -1.0)
        };
        
        // Calculate UV coordinates and sample texture
        let (u, v) = self.calculate_uv(point, normal);
        let texture_color = self.sample_texture(u, v);
        
        // Create material with texture color modulating the diffuse color
        let mut textured_material = self.material;
        textured_material.diffuse = Vector3::new(
            textured_material.diffuse.x * texture_color.x,
            textured_material.diffuse.y * texture_color.y,
            textured_material.diffuse.z * texture_color.z,
        );
        
        Intersect::new(point, normal, t, textured_material)
    }
}
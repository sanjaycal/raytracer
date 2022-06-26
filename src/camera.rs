use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera{
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera{
    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray{origin: self.origin, direction: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin}
    }
}

pub fn new_camera(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera{
    let origin = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    let viewport_width = aspect_ratio*viewport_height;
    let horizontal = Vec3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3 {x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 -vertical/2.0 - Vec3{x:0.0, y: 0.0, z: focal_length};
    Camera {aspect_ratio: aspect_ratio, viewport_height: viewport_height, viewport_width:viewport_width, focal_length: focal_length, origin: origin, horizontal: horizontal, vertical: vertical, lower_left_corner: lower_left_corner} 
}


use super::vec::Vec3;
use super::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub vertical: Vec3,
    pub horizontal: Vec3,
    pub left_bottom: Vec3,
}

impl Camera {
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        return Ray {
            origin: self.origin.clone(),
            direction: self
                .left_bottom
                .add(&self.horizontal.muln(x))
                .add(&self.vertical.muln(y))
                .sub(&self.origin),
        };
    }
}
use super::base::ray::Ray;
use super::base::vec::Vec3;


pub mod sphere;
pub mod hitList;

#[derive(Debug)]
// struct HitResult {
//     field: Type
// }


pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
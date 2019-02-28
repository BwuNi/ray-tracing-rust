use crate::base::ray::Ray;
use crate::base::vec::Vec3;
use crate::material::Material;

pub mod hit_list;
pub mod sphere;

pub use hit_list::HitList;
pub use sphere::Sphere;

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord,&Box<Material>)>;
}



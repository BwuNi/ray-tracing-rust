use super::base::ray::Ray;
use super::base::vec::Vec3;

pub mod hitList;
pub mod sphere;

#[derive(Debug)]
// struct HitResult {
//     field: Type
// }

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub fn hit() {}

// impl Hitable for HitObj {
//     // add code here
// }

pub type HitFunc = Fn(&Ray, f64, f64) -> Option<HitRecord>;

pub struct HitObj {
    pub hit: HitFunc
}


pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn toObj(self) -> Box<HitFunc> ;
}

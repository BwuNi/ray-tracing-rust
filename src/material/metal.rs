use super::Material;
use crate::base::{Ray, Vec3};
use crate::hit::HitRecord;

extern crate rand;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

#[allow(unused_variables)]
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: HitRecord) -> (Option<Ray>, &Vec3) {

        return (Some(ray_in.reflect(&hit,self.fuzz)), &self.albedo);
    }
}

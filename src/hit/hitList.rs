use super::super::base::ray::Ray;
use super::super::base::vec::Vec3;
use super::Hitable;
use super::HitRecord;



pub struct HitList {
    list:Vec<Box<Hitable>>,
}

impl Hitable for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        return None;
    }
}

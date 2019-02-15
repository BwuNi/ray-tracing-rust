use super::super::base::ray::Ray;
use super::super::base::vec::Vec3;
use super::HitFunc;
use super::HitRecord;
use super::Hitable;
use super::HitObj;

pub struct HitList {
    pub list: Vec<Box<HitFunc>>,
}

impl HitList {
       fn hitn(self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list.into_iter().map(|x| {
            let hit = Box::into_raw(x);

            return "";
        });

        return None;
    }
}

impl Hitable for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self.list).into_iter().map(|x| {
            // let hit = Box::into_raw(x);
            return "";
        });

        return None;
    }

    
 

    fn toObj(self) -> Box<HitFunc> {
        let hit = move |ray: &Ray, t_min: f64, t_max: f64| self.hit(ray, t_min, t_max);
        return Box::new(hit);
    }
}

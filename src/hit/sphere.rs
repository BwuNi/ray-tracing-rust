use super::super::base::ray::Ray;
use super::super::base::vec::Vec3;
use super::Hitable;
use super::HitRecord;

use super::HitFunc;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.sub(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction) * 2.0;
        let c = oc.dot(&oc) - self.radius.powf(2.);

        let discriminate = b.powf(2.) - 4. * a * c;

        if discriminate > 0. {
            let mut temp = (-b - discriminate.powf(0.5)) / (2. * a);

            if temp > t_min && temp < t_max {
                let t = temp;
                let p = ray.get_point(temp);
                let normal = p.sub(&self.center).divn(self.radius);

                return Some(HitRecord { t, p, normal });
            }

            temp = (-b + discriminate.powf(0.5)) / (2. * a);

            if temp > t_min && temp < t_max {
                let t = temp;
                let p = ray.get_point(temp);
                let normal = p.sub(&self.center).divn(self.radius);

                return Some(HitRecord { t, p, normal });
            }
        }

        return None;
    }


    fn toObj(self)-> Box<HitFunc>{
        let hit = move |ray: &Ray, t_min: f64, t_max: f64|{
            return self.hit(ray,t_min,t_max)
        };
        return Box::new(hit)
    }
}

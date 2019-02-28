use crate::base::vec::Vec3;
use crate::hit::HitRecord;
use rand::Rng;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn get_point(&self, t: f64) -> Vec3 {
        return self.origin.add(&self.direction.muln(t));
    }
    #[allow(dead_code)]
    pub fn reflect(&self, hit: &HitRecord,fuzz:f64) -> Ray {
        return Ray {
            origin: hit.p.clone(),
            direction: 
            reflect(&self.direction.unit_vec(), &hit.normal).add(&random_in_unit_sphere(fuzz)),
        };
    }
}
#[allow(dead_code)]
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v.sub(&n.muln(v.dot(&n) * 2.));
}



fn random_in_unit_sphere(fuzz:f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            .muln(2.0)
            .sub(&Vec3(1., 1., 1.));

        if p.squared_length() > 1. {
            break p;
        }
    }.muln(0.)
}

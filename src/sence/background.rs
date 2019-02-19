use super::super::base::ray::Ray;
use super::super::base::vec::Vec3;

pub type BackGround = Box<Fn(Ray) -> Vec3>;

pub fn normal() -> BackGround {
    return Box::new(|ray: Ray| {
        let u = ray.direction.unit_vec();
        let t = (u.1 + 1.0) * 0.5;

        return Vec3(1., 1., 1.)
            .muln(1.0 - t)
            .add(&Vec3(0.3, 0.5, 1.0).muln(t));
    });
}

pub fn black()  -> BackGround {
    return Box::new(|ray: Ray| {
        return Vec3(0., 0., 0.)
    });
}

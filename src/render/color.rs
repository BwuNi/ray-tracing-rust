use super::super::base::vec::Vec3;
use super::super::hit::Hitable;
use super::super::sence;

pub fn gen()->Box<Fn(f64,f64)->Vec3> {
    let (camera, background, world) = sence::gen();

    return Box::new(move|x: f64, y: f64| -> Vec3 {
        let ray = camera.get_ray(x, y);

        match world.hit(&ray, 0., std::f64::INFINITY) {
            Some(hit) => return hit.normal.unit_vec().addn(1.0).muln(0.5),
            None => return background(ray),
        };
    })
}
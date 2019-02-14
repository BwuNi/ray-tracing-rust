mod base;
mod hit;

use base::ray::Ray;
use base::vec::Vec3;
use hit::Sphere;
use hit::Hitable;

struct Camera {
    origin: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    left_bottom: Vec3,
}

impl Camera {
    fn get_ray(&self, x: f64, y: f64) -> Ray {
        return Ray {
            origin: self.origin.clone(),
            direction: self.left_bottom
                .add(&self.horizontal.muln(x))
                .add(&self.vertical.muln(y)),
        };
    }
}

#[no_mangle]
pub extern "C" fn fib(_w: i32, _h: i32, _x: i32, _y: i32) -> i32 {
    return color(
        (_x as f64) / (_w as f64),
        ((_h - 1 - _y) as f64) / (_h as f64),
    );
}

fn color(x: f64, y: f64) -> i32 {
    let camera = Camera {
        origin: Vec3(0., 0., 1.),
        left_bottom: Vec3(-2., -1., -1.),
        horizontal: Vec3(4., 0., 0.),
        vertical: Vec3(0., 2., 0.),
    };

    let ball = Sphere{
        center:Vec3(0., 0., -1.),
        radius:0.5
    };


    let ray = camera.get_ray(x, y);


    let u = ray.direction.unit_vec();
    let t = (u.1 + 1.0) * 0.5;

    let res = Vec3(1., 1., 1.)
        .muln(1.0 - t)
        .add(&Vec3(0.3, 0.5, 1.0).muln(t));



    match ball.hit(&ray,0.,std::f64::INFINITY) {
        Some(hit) => return hit.normal.unit_vec().addn(1.0).muln(0.5).color(),
        None => return res.color(),
    };

}

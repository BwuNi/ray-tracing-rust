mod base;
mod hit;
mod sence;

extern crate rand;

use base::vec::Vec3;
use hit::Hitable;
use rand::Rng;

mod render;


#[no_mangle]
pub extern "C" fn fib(_w: i32, _h: i32, _x: i32, _y: i32) -> i32 {
    let mut rng = rand::thread_rng();

    return color(
        (_x as f64 + rng.gen::<f64>()) / (_w as f64),
        ((_h - 1 - _y) as f64 + rng.gen::<f64>()) / (_h as f64),
    );
}

fn color(x: f64, y: f64) -> i32 {
    if x < 0. || x > 1. {
        return Vec3(0.3, 0.5, 1.0).color();
    }
    if y < 0. || y > 1. {
        return Vec3(0.3, 0.5, 1.0).color();
    }

    let (camera,background,world) = sence::gen();

    let ray = camera.get_ray(x, y);

    match world.hit(&ray, 0., std::f64::INFINITY) {
        Some(hit) => return hit.normal.unit_vec().addn(1.0).muln(0.5).color(),
        None => return background(ray).color(),
    };
}

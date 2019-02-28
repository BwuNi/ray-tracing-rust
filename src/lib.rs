mod base;
mod hit;
mod material;
mod render;
mod sence;

extern crate rand;

use base::{Ray, Vec3};
use rand::Rng;
use crate::hit::Hitable;

#[no_mangle]
pub extern "C" fn fib(_w: i32, _h: i32, _x: i32, _y: i32) -> i32 {
    let mut rng = rand::thread_rng();

    return color(
        (_x as f64 + rng.gen::<f64>()) / (_w as f64),
        ((_h - 1 - _y) as f64 + rng.gen::<f64>()) / (_h as f64),
    )
    .color();
}

fn color(x: f64, y: f64) -> Vec3 {
    let (camera, background, world) = sence::gen();

    let mut some_ray: Option<Ray> = Some(camera.get_ray(x, y));
    let mut step: u32 = 0;
    let mut vec = Vec3(1., 1., 1.);

    return loop {
        match some_ray {
            // 存在下一步光线
            Some(ref ray) => match world.hit(ray, 0., std::f64::INFINITY) {
                Some((h, m)) => {
                    // step 大于 50 ，直接返回黑色
                    step = step + 1;
                    if step > 51 {
                        break Vec3(0., 0., 0.);
                    };
                    // step 小于 50 ，计算下一步 ray，vec 乘上反射系数
                    let (s_r, v) = m.scatter(ray, h);
                    some_ray = s_r;
                    vec = vec.mul(v);
                }
                None => break vec.mul(&background(ray)),
            },
            // 不存在下一步光线
            None => break vec,
        };
    };
}

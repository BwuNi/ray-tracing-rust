mod base;
mod hit;

extern crate rand;

use base::ray::Ray;
use base::vec::Vec3;
use base::camera::Camera;

use hit::hitList::HitList;
use hit::sphere::Sphere;
use hit::Hitable;
use rand::Rng;


fn main() {
    println!("Hello, world!");
}

fn color(x: f64, y: f64) -> i32 {
    if x < 0. || x > 1. {
        return Vec3(0.3, 0.5, 1.0).color();
    }
    if y < 0. || y > 1. {
        return Vec3(0.3, 0.5, 1.0).color();
    }

    let camera = Camera {
        origin: Vec3(0., 0., 1.),
        left_bottom: Vec3(-2., -1., -1.),
        horizontal: Vec3(4., 0., 0.),
        vertical: Vec3(0., 2., 0.),
    };

    let ball0 = Sphere {
        center: Vec3(0., 0., -1.),
        radius: 0.5,
    };

    let ball1 = Sphere {
        center: Vec3(1., 0., -1.),
        radius: 0.5,
    };

    let ball2 = Sphere {
        center: Vec3(-1., 0., -1.),
        radius: 0.5,
    };

    let earth = Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
    };

    let world = HitList {
        list: vec![
            Box::new(ball0),
            Box::new(ball1),
            Box::new(ball2),
            Box::new(earth),
        ],
    };

    let ray = camera.get_ray(x, y);

    let u = ray.direction.unit_vec();
    let t = (u.1 + 1.0) * 0.5;

    let res = Vec3(1., 1., 1.)
        .muln(1.0 - t)
        .add(&Vec3(0.3, 0.5, 1.0).muln(t));

    match world.hit(&ray, 0., std::f64::INFINITY) {
        Some(hit) => return hit.normal.unit_vec().addn(1.0).muln(0.5).color(),
        None => return res.color(),
    };
}

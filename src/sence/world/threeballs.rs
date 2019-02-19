use super::super::super::base::Vec3;
use super::super::super::hit::HitList;
use super::super::super::hit::Sphere;

pub fn defalut ()->HitList{
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

    return  HitList {
        list: vec![
            Box::new(ball0),
            Box::new(ball1),
            Box::new(ball2),
            Box::new(earth),
        ],
    };
}
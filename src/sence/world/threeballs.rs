use crate::base::Vec3;
use crate::hit::HitList;
use crate::hit::Sphere;
use crate::material::Metal;

pub fn defalut ()->HitList{
    let ball0 = Sphere {
        center: Vec3(0., 0., -1.),
        radius: 0.5,
        material:Box::new(Metal{
            albedo:Vec3(0.8,0.4,0.),
            fuzz:0.5
        })
    };

    let ball1 = Sphere {
        center: Vec3(1., 0., -1.),
        radius: 0.5,
        material:Box::new(Metal{
            albedo:Vec3(0.8,0.,0.4),
            fuzz:1.
        })
    };

    let ball2 = Sphere {
        center: Vec3(-1., 0., -1.),
        radius: 0.5,
        material:Box::new(Metal{
            albedo:Vec3(0.4,0.8,0.),
            fuzz:0.
        })
    };

    let earth = Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
        material:Box::new(Metal{
            albedo:Vec3(0.2,0.2,0.2),
            fuzz:0.2
        })
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
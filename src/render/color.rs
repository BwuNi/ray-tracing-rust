// use super::super::base::vec::Vec3;
// use super::super::hit::Hitable;
// use super::super::sence;

// pub fn gen()->Box<Fn(f64,f64)->Vec3> {
//     let (camera, background, world) = sence::gen();

//     return Box::new(move|x: f64, y: f64| -> Vec3 {
//         let ray = camera.get_ray(x, y);

//         match world.hit(&ray, 0., std::f64::INFINITY) {
//             Some(hit) => return hit.normal.unit_vec().addn(1.0).muln(0.5),
//             None => return background(ray),
//         };
//     })
// }

use crate::hit::hit_list::HitList;
use super::super::sence;
use crate::base::{Ray, Vec3};
use crate::hit::Hitable;

#[allow(dead_code)]
// pub fn gen() -> Box<Fn(f64, f64) -> Vec3> {
//     let (camera, background, world) = sence::gen();

//     return Box::new(move |x: f64, y: f64| -> Vec3 {

//         fn aaaa(
//             world: &HitList,
//             background: &Box<Fn(&Ray) -> Vec3>,
//             ray: &Ray,
//             step: u32,
//             v: Vec3,
//         ) -> Vec3 {
//             if step > 50 {
//                 return Vec3(0., 0., 0.);
//             };

//             let a = world.hit(ray, 0., std::f64::INFINITY);

//             match a {
//                 None => v.mul(&background(ray)),
//                 Some((h, m)) => {
//                     let (s_r, v0) = m.scatter(ray, h);
//                     match s_r {
//                         None => v.mul(&v0),
//                         Some(rray)=>{
//                             aaaa(&world,background,&rray,step+1,v.mul(&v0))
//                         }
//                     }
//                 }
//             }
//         }

//         aaaa(&world,&background,&camera.get_ray(x, y),0,Vec3(1.,1.,1.,))
//     });
// }

pub fn gen() -> Box<Fn(f64, f64) -> Vec3> {
    let (camera, background, world) = sence::gen();

    return Box::new(move |x: f64, y: f64| -> Vec3 {
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
                        if step > 50 {
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
        }
    });
}

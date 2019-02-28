mod metal;

use crate::base::{Ray,Vec3};
use crate::hit::HitRecord;

pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: HitRecord) ->( Option<Ray>,&Vec3);
}

// export default interface Material {
//     scatter: (rayIn: Ray, hit: HitRecord) => [Ray,Attenuation]
// }
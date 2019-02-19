use super::super::base::Camera;
use super::super::base::Vec3;

pub fn c0() -> Camera {
    return Camera {
        origin: Vec3(0., 0., 1.),
        left_bottom: Vec3(-2., -1., -1.),
        horizontal: Vec3(4., 0., 0.),
        vertical: Vec3(0., 2., 0.),
    };
}

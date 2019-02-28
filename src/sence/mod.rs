use super::base::Camera;
use super::hit::HitList;


pub mod background;

pub mod world;

pub mod camera;

type Sence  = (Camera,background::BackGround,HitList);

pub fn gen()->Sence{
    return (camera::c0(),background::normal(),world::three_balls())
}
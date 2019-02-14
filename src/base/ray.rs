use super::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn get_point(&self, t: f64) -> Vec3 {
        return self.origin.add(&self.direction.muln(t));
    }
}

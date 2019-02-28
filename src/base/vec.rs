use std::convert::Into;

#[derive(Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub fn v<T: Into<f64>>(a: T, b: T, c: T) -> Vec3 {
    return Vec3(a.into(), b.into(), c.into());
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        return self.squared_length().powf(0.5);
    }

    pub fn squared_length(&self) -> f64 {
        return self.dot(self);
    }

    pub fn unit_vec(&self) -> Vec3 {
        let l = self.length();
        return v(self.0 / l, self.1 / l, self.2 / l);
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
    }

    #[allow(dead_code)]
    pub fn color(&self) -> i32 {
        return (self.0 * 255.999999999999f64).floor() as i32 * 256 * 256
            + (self.1 * 255.999999999999f64).floor() as i32 * 256
            + (self.2 * 255.999999999999f64).floor() as i32;
    }

    #[allow(dead_code)]
    pub fn to_color_string(&self) -> String {
        let r = (self.0 * 255.999999999999f64).floor() as i32;
        let g = (self.1 * 255.999999999999f64).floor() as i32;
        let b = (self.2 * 255.999999999999f64).floor() as i32;

        return format!("{}.{}.{}", r, g, b);
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    pub fn mul(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }

    pub fn div(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }

    #[allow(dead_code)]
    pub fn addn(&self, other: f64) -> Vec3 {
        self.add(&Vec3(other, other, other))
    }

    #[allow(dead_code)]
    pub fn subn(&self, other: f64) -> Vec3 {
        self.sub(&Vec3(other, other, other))
    }

    pub fn muln(&self, other: f64) -> Vec3 {
        self.mul(&Vec3(other, other, other))
    }

    pub fn divn(&self, other: f64) -> Vec3 {
        self.div(&Vec3(other, other, other))
    }

    pub fn clone(&self) -> Vec3 {
        Vec3(self.0, self.1, self.2)
    }
}

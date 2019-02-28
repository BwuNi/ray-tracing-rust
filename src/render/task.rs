extern crate rand;

use crate::base::vec::Vec3;
use rand::Rng;

use super::color;

#[allow(dead_code)]
pub struct Task {
    px: Vec<Vec<(i32, i32)>>,
    width: i32,
    height: i32,
    length: usize,
    color: Box<Fn(f64, f64) -> Vec3>,
    callback: Box<Fn(Vec<(i32, i32, Vec3)>) -> bool>,
}

impl Task {
    #[allow(dead_code)]
    pub fn start(&mut self) {
        self.px = generate_px(self.width, self.height, self.length);

        for list in &self.px {
            let res: Vec<(i32, i32, Vec3)> = list
                .into_iter()
                .map(|item| -> (i32, i32, Vec3) { 
                    let mut r = Vec3(0.,0.,0.,);
                    let length = 5;
                    for _i in 0..length {
                        r = r.add(&self.render(item.0, item.1))
                    }
                    (item.0, item.1, r.divn(length as f64)) })
                .collect();

            if !(&self.callback)(res) {
                return;
            }
        }
    }
    #[allow(dead_code)]
    fn render(&self, x: i32, y: i32) -> Vec3 {
        let mut rng = rand::thread_rng();

        let (_x, _y) = (
            (x as f64 + rng.gen::<f64>()) / (self.width as f64),
            ((self.height - 1 - y) as f64 + rng.gen::<f64>()) / (self.height as f64),
        );
        return (&self.color)(_x, _y);
    }
}

#[allow(dead_code)]
pub fn new(
    width: i32,
    height: i32,
    length: usize,
    callback: Box<Fn(Vec<(i32, i32, Vec3)>) -> bool>,
) -> Task {
    return Task {
        px: generate_px(width, height, length),
        width,
        height,
        length,
        color: color::gen(),
        callback,
    };
}

fn generate_px(width: i32, height: i32, length: usize) -> Vec<Vec<(i32, i32)>> {
    let mut all: Vec<(i32, i32)> = Vec::new();

    for w in 0..width {
        for h in 0..height {
            all.push((w, h))
        }
    }

    let mut res: Vec<Vec<(i32, i32)>> = vec![vec![]];
    let mut list: Vec<(i32, i32)> = vec![];
    for v in all {
        if list.len() >= length {
            res.push(list);
            list = vec![];
        }
        list.push(v)
    }

    res.push(list);

    return res;
}

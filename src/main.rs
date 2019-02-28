mod base;
mod hit;
mod material;
mod render;
mod sence;

extern crate rand;

use base::vec::Vec3;
// use hit::Hitable;
// use rand::Rng;

use render::task;

fn main() {
    let width = 800;
    let heigth = 400;
    let callback = Box::new(|res: Vec<(i32, i32, Vec3)>| {
        let mut all = String::new();
        for i in res {
            let s = format!(
                "/{}-{}-{}",
                i.0.to_string(),
                i.1.to_string(),
                i.2.to_color_string()
            );

            all = all + &s;
        }

        print!("{}",all);
        return true;
    });

    let mut t = task::new(width, heigth,20000, callback);

    loop {
        t.start();
    }    
}

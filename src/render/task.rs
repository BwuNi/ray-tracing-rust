extern crate rand;

use crate::base::vec::Vec3;
use rand::Rng;

use super::color;

struct Task {
    px: Vec<Vec<(i32, i32)>>,
    width: i32,
    height: i32,
    color:Box<Fn(f64,f64)->Vec3>
}


impl Task {
    pub fn do_task(&self) {
        for list in &self.px {
            let res:Vec<(i32,i32,Vec3)> = list.into_iter().map(|item|->(i32,i32,Vec3) {(item.0,item.1,self.render(item.0,item.1))}).collect();
        }
    }

    fn render(&self, x: i32, y: i32) ->Vec3 {
        let mut rng = rand::thread_rng();

        let (_x,_y) =  ( 
            (x as f64 + rng.gen::<f64>()) / (self.width as f64),
            ((self.height - 1 - y) as f64 + rng.gen::<f64>()) / (self.height as f64)
        );
        return (&self.color)(_x,_y)
    }
}

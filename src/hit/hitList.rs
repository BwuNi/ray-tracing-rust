use super::super::base::ray::Ray;
use super::HitRecord;
use super::Hitable;

pub struct HitList {
    pub list: Vec<Box<Hitable>>,
}


impl Hitable for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

           
        let mut res:Option<HitRecord> = None;

        for obj in &self.list{
            let hit = obj.hit(ray,t_min,t_max);

            res = match res {
                Some(old_record) => match hit {
                    Some(record) => {
                        if record.t < old_record.t{
                            Some(record)
                        }else{
                            Some(old_record)
                        }
                    },
                    None => {Some(old_record)},
                },
                None => {hit},
            }

        } 

        return res;
    }

}



// struct B {a:Box<Fn(i32)->i32>}

// fn main(){
//     let a = |x:i32|->i32 {x};
//     let e = B{a:Box::new(a)};

//     let d = Box::into_raw(e.a);

//     let dd = d(1);
//     print!("{}",dd)
// }
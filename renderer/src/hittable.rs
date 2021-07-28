use super::{HitRecord, Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

//impl<'a> Hittable for Vec<&'a dyn Hittable> {
//    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//        self.iter().fold(None, |hr, object| {
//            if let Some(prev_hit) = &hr {
//                match object.hit(ray, t_min, prev_hit.t) {
//                    Some(new_hit) => Some(new_hit),
//                    None => hr,
//                }
//            } else {
//                object.hit(ray, t_min, t_max)
//            }
//        })
//    }
//}

impl Hittable for &[Box<dyn Hittable>] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter().fold(None, |hr, object| {
            if let Some(prev_hit) = &hr {
                match object.hit(ray, t_min, prev_hit.t) {
                    Some(new_hit) => Some(new_hit),
                    None => hr,
                }
            } else {
                object.hit(ray, t_min, t_max)
            }
        })
    }
}

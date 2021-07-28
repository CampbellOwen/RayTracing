use super::{HitRecord, Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl<'a> Hittable for Vec<&'a dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

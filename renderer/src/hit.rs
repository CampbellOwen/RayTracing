use super::{dot, Ray, Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: &Vec3, normal: &Vec3, t: f32) -> HitRecord {
        let mut hr = HitRecord {
            point: *point,
            normal: *normal,
            t,
            front_face: true,
        };
        hr.set_face_normal(ray, normal);
        return hr;
    }
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

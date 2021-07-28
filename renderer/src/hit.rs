use std::rc::Rc;

use super::{dot, Material, Ray, Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        point: &Vec3,
        normal: &Vec3,
        material: Rc<dyn Material>,
        t: f64,
    ) -> HitRecord {
        let mut hr = HitRecord {
            point: *point,
            normal: *normal,
            material,
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

use std::rc::Rc;

use super::{dot, HitRecord, Hittable, Material, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - &self.center;

        let a = ray.dir.length_squared();
        let half_b = dot(&oc, &ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let hr = HitRecord::new(ray, &point, &normal, self.material.clone(), t);

        Some(hr)
    }
}

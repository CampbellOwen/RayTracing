use super::{dot, HitRecord, Hittable, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - &self.center;
        //let a = dot(&ray.dir, &ray.dir);
        //let b = 2.0 * dot(&oc, &ray.dir);
        //let c = dot(&oc, &oc) - radius * radius;
        //let discriminant = b * b - (4.0 * a * c);

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
        let hr = HitRecord::new(ray, &point, &normal, t);

        Some(hr)
    }
}

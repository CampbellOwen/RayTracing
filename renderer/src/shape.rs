use std::rc::Rc;

use super::{dot, HitRecord, Hittable, Material, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

pub trait Spherical {
    fn center(&self, time: f64) -> Vec3;
    fn radius(&self, time: f64) -> f64;
    fn material(&self) -> Rc<dyn Material>;
}

impl Spherical for Sphere {
    fn center(&self, _: f64) -> Vec3 {
        self.center
    }
    fn radius(&self, _: f64) -> f64 {
        self.radius
    }
    fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

fn spherical_hit<T: Spherical>(sphere: &T, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let center = sphere.center(ray.time);
    let radius = sphere.radius(ray.time);

    let oc = ray.origin - center;

    let a = ray.dir.length_squared();
    let half_b = dot(&oc, &ray.dir);
    let c = oc.length_squared() - radius * radius;
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
    let normal = (point - center) / radius;
    let hr = HitRecord::new(ray, &point, &normal, sphere.material(), t);

    Some(hr)
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        spherical_hit(self, ray, t_min, t_max)
    }
}

pub struct MovingSphere {
    pub center_0: Vec3,
    pub center_1: Vec3,

    pub time_0: f64,
    pub time_1: f64,

    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Spherical for MovingSphere {
    fn center(&self, time: f64) -> Vec3 {
        self.center_0
            + (self.center_1 - self.center_0) * ((time - self.time_0) / (self.time_1 - self.time_0))
    }
    fn radius(&self, _: f64) -> f64 {
        self.radius
    }

    fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        spherical_hit(self, ray, t_min, t_max)
    }
}

use std::rc::Rc;

use super::{dot, HitRecord, Hittable, Material, Ray, Vec3, AABB};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

pub trait Spherical {
    fn center(&self, time: f64) -> Vec3;
    fn radius(&self, time: f64) -> f64;
    fn material(&self) -> &Rc<dyn Material>;
}

impl Spherical for Sphere {
    fn center(&self, _: f64) -> Vec3 {
        self.center
    }
    fn radius(&self, _: f64) -> f64 {
        self.radius
    }
    fn material(&self) -> &Rc<dyn Material> {
        &self.material
    }
}

fn spherical_uv(p: &Vec3) -> (f64, f64) {
    // u in [0, 1] for angle around the Y axis, from x = -1
    // v in [0, 1] for angle around the Z axis, from y = -1 to y = 1
    //
    // u = phi / 2pi
    // v = theta / pi
    //
    // x = -cos(phi)sin(theta)
    // y = -cos(theta)
    // z = sin(phi)sin(theta)

    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;

    (
        phi / (2.0 * std::f64::consts::PI),
        theta / std::f64::consts::PI,
    )
}

fn spherical_hit<'material, T: Spherical>(
    sphere: &'material T,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord<'material>> {
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
    let (u, v) = spherical_uv(&normal);
    let hr = HitRecord::new(ray, &point, &normal, sphere.material(), t, u, v);

    Some(hr)
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        spherical_hit(self, ray, t_min, t_max)
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<crate::AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
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

    fn material(&self) -> &Rc<dyn Material> {
        &self.material
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        spherical_hit(self, ray, t_min, t_max)
    }
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        let center_0 = self.center(time_0);
        let center_1 = self.center(time_1);

        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);

        Some(AABB::surrounding_box(
            &AABB {
                min: center_0 - radius_vec,
                max: center_0 + radius_vec,
            },
            &AABB {
                min: center_1 - radius_vec,
                max: center_1 + radius_vec,
            },
        ))
    }
}

pub struct AARect {
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    pub z: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for AARect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / ray.dir.z;
        if t < t_min || t > t_max {
            return None;
        }

        let ray_hit = ray.at(t);
        let (x, y) = (ray_hit.x, ray_hit.y);
        let (min_x, max_x) = self.x_range;
        let (min_y, max_y) = self.y_range;
        if x < min_x || x > max_x || y < min_y || y > max_y {
            return None;
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        let u = (x - min_x) / width;
        let v = (y - min_y) / height;

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        Some(HitRecord::new(
            ray,
            &ray_hit,
            &outward_normal,
            &self.material,
            t,
            u,
            v,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let (min_x, max_x) = self.x_range;
        let (min_y, max_y) = self.y_range;
        Some(AABB {
            min: Vec3::new(min_x, min_y, self.z - 0.0001),
            max: Vec3::new(max_x, max_y, self.z + 0.0001),
        })
    }
}

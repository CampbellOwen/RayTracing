use std::{ops::Mul, sync::Arc};

use crate::{
    rand_in_unit_sphere, spherical_direction, OrthoNormalBasis, SampleableLight, UniformConePDF,
    UniformSpherePDF, PDF,
};

use super::{HitRecord, Hittable, Material, Ray, AABB};

use glam::{DMat4, DVec3, DVec4, Vec4Swizzles};
use rand::Rng;

pub trait Transformable {
    fn transform(&self, transform: &DMat4) -> Self;
}

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Transformable for Sphere {
    fn transform(&self, transform: &DMat4) -> Sphere {
        let o: DVec4 = DVec4::from((self.center, 1.0));
        let r: DVec4 = DVec4::from((o.xy(), o.z + self.radius, 0.0));

        Sphere {
            center: transform.mul(o).xyz(),
            radius: transform.mul(r).length(),
            material: self.material.clone(),
        }
    }
}

impl SampleableLight for Sphere {
    fn pdf_for_point(&self, point: DVec3) -> Box<dyn PDF> {
        if (point - self.center).length_squared() <= self.radius * self.radius {
            return Box::new(UniformSpherePDF {
                center: self.center,
                radius: self.radius,
            });
        }

        let sin_theta_max_2 = (self.radius * self.radius) / (point.distance_squared(self.center));
        let cos_theta_max = f64::max(0.0, 1.0 - sin_theta_max_2).sqrt();

        Box::new(UniformConePDF::new(
            (self.center - point).normalize(),
            cos_theta_max,
        ))
    }
}

pub trait Spherical {
    fn center(&self, time: f64) -> DVec3;
    fn radius(&self, time: f64) -> f64;
    fn material(&self) -> &Arc<dyn Material>;
}

impl Spherical for Sphere {
    fn center(&self, _: f64) -> DVec3 {
        self.center
    }
    fn radius(&self, _: f64) -> f64 {
        self.radius
    }
    fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }
}

fn spherical_uv(p: &DVec3) -> (f64, f64) {
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
    let half_b = oc.dot(ray.dir);
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
    let hr = HitRecord::new(ray, &point, normal, sphere.material().as_ref(), t, u, v);

    Some(hr)
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        spherical_hit(self, ray, t_min, t_max)
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<crate::AABB> {
        Some(AABB {
            min: self.center - DVec3::new(self.radius, self.radius, self.radius),
            max: self.center + DVec3::new(self.radius, self.radius, self.radius),
        })
    }

    fn sample_uniform(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        (rand_in_unit_sphere(rng) * self.radius) + self.center
    }

    fn pdf_uniform(&self, point: DVec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI * (self.radius * self.radius))
    }
    fn sample_from_ref(&self, rng: &mut dyn rand::RngCore, reference_point: DVec3) -> DVec3 {
        if (reference_point - self.center).length_squared() <= self.radius * self.radius {
            return self.sample_uniform(rng);
        }

        let ref_to_sphere = (self.center - reference_point).normalize();
        let local_coordinates = OrthoNormalBasis::from_w(&ref_to_sphere);

        // Sample sphere uniformly inside subtended cone
        // theta and phi for values in sample in cone
        let sin_theta_max2 =
            (self.radius * self.radius) / reference_point.distance_squared(self.center);

        let cos_theta_max = (f64::max(0.0, 1.0 - sin_theta_max2)).sqrt();

        let pdf = UniformConePDF::new(ref_to_sphere, cos_theta_max);
        pdf.generate(rng)
        //let rand_1 = rng.gen::<f64>();
        //let rand_2 = rng.gen::<f64>();
        //let cos_theta = (1.0 - rand_1) + rand_1 * cos_theta_max;
        //let sin_theta = f64::max(0.0, 1.0 - (cos_theta * cos_theta)).sqrt();
        //let phi = rand_2 * 2.0 * std::f64::consts::PI;

        //let dc = reference_point.distance(self.center);
        //let ds = dc * cos_theta
        //    - f64::max(
        //        0.0,
        //        (self.radius * self.radius) - (dc * dc) - (sin_theta * sin_theta),
        //    )
        //    .sqrt();

        //let cos_alpha =
        //    ((dc * dc) + (self.radius * self.radius) - (ds * ds)) / (2.0 * dc * self.radius);
        //let sin_alpha = f64::max(0.0, 1.0 - (cos_alpha * cos_alpha)).sqrt();

        //let sampled_point = local_coordinates
        //    .local(&(spherical_direction(sin_alpha, cos_alpha, phi) * self.radius));

        //sampled_point
    }

    fn pdf_from_ref(&self, reference_point: DVec3, pt: DVec3) -> f64 {
        if (reference_point - self.center).length_squared() <= self.radius * self.radius {
            return self.pdf_uniform(pt);
        }

        let sin_theta_max_2 = (self.radius * self.radius) / (pt.distance_squared(self.center));
        let cos_theta_max = f64::max(0.0, 1.0 - sin_theta_max_2).sqrt();

        let pdf = UniformConePDF::new((pt - reference_point).normalize(), cos_theta_max);
        pdf.value(pt)
    }
}

pub struct MovingSphere {
    pub center_0: DVec3,
    pub center_1: DVec3,

    pub time_0: f64,
    pub time_1: f64,

    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Spherical for MovingSphere {
    fn center(&self, time: f64) -> DVec3 {
        self.center_0
            + (self.center_1 - self.center_0) * ((time - self.time_0) / (self.time_1 - self.time_0))
    }
    fn radius(&self, _: f64) -> f64 {
        self.radius
    }

    fn material(&self) -> &Arc<dyn Material> {
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

        let radius_vec = DVec3::new(self.radius, self.radius, self.radius);

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

    fn sample_uniform(&self, _: &mut dyn rand::RngCore) -> DVec3 {
        todo!()
    }

    fn pdf_uniform(&self, point: DVec3) -> f64 {
        todo!()
    }
}

pub struct AARect {
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    pub z: f64,
    pub material: Arc<dyn Material>,
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

        let outward_normal = DVec3::new(0.0, 0.0, 1.0);
        Some(HitRecord::new(
            ray,
            &ray_hit,
            outward_normal,
            self.material.as_ref(),
            t,
            u,
            v,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let (min_x, max_x) = self.x_range;
        let (min_y, max_y) = self.y_range;
        Some(AABB {
            min: DVec3::new(min_x, min_y, self.z - 0.0001),
            max: DVec3::new(max_x, max_y, self.z + 0.0001),
        })
    }

    fn sample_uniform(&self, _: &mut dyn rand::RngCore) -> DVec3 {
        todo!()
    }

    fn pdf_uniform(&self, point: DVec3) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Lambertian;

    use super::*;

    #[test]
    fn sample_sphere_test() {
        let lit_point = DVec3::ZERO;
        let sphere_light = Sphere {
            center: DVec3::new(12.0, 12.0, 12.0),
            radius: 6.0,
            material: Arc::new(Lambertian::new(DVec3::ZERO)),
        };

        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let sample = sphere_light
                .sample_from_ref(&mut rng, lit_point)
                .normalize();
            println!("{}", sample);
        }

        assert!(false)
    }
}

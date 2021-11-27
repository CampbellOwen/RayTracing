use std::sync::Arc;

use glam::{DMat4, DVec4, Vec4Swizzles};

use crate::{HitRecord, Hittable, Ray, AABB};

pub struct Transformed {
    t: DMat4,
    t_inv: DMat4,
    hittable: Arc<dyn Hittable>,
}

impl Transformed {
    pub fn new(transformation: DMat4, hittable: Arc<dyn Hittable>) -> Transformed {
        Transformed {
            t: transformation,
            t_inv: transformation.inverse(),
            hittable: hittable,
        }
    }
}

impl Hittable for Transformed {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let new_origin = self.t_inv * DVec4::from((ray.origin, 1.0));
        let new_dir = self.t_inv * DVec4::from((ray.dir, 0.0));

        let transformed = Ray {
            origin: new_origin.xyz(),
            dir: new_dir.xyz(),
            time: ray.time,
        };

        let inverse_transpose = self.t_inv.transpose();

        match self.hittable.hit(&transformed, t_min, t_max) {
            Some(hr) => Some(HitRecord {
                //pub point: DVec3,
                //pub normal: DVec3,
                //pub material: &'material Arc<dyn Material>,
                //pub t: f64,
                //pub u: f64,
                //pub v: f64,
                //pub front_face: bool,
                point: ray.at(hr.t),
                normal: (inverse_transpose * DVec4::from((hr.normal, 0.0)))
                    .xyz()
                    .normalize(),
                material: hr.material,
                t: hr.t,
                u: hr.u,
                v: hr.v,
                front_face: hr.front_face,
            }),
            None => None,
        }
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        self.hittable.bounding_box(time_0, time_1)
    }
}

#[cfg(test)]
mod tests {

    use glam::DVec3;

    use crate::{Lambertian, Sphere};

    use super::*;

    #[test]
    fn transformed_sphere() {
        let sphere = Sphere {
            center: DVec3::ZERO,
            radius: 1.0,
            material: Arc::new(Lambertian::new(DVec3::splat(0.5))),
        };

        let ray = Ray {
            origin: DVec3::new(0.0, 2.0, 1.0),
            dir: DVec3::new(0.0, 0.0, -1.0),
            time: 0.0,
        };

        assert!(sphere.hit(&ray, 0.0, 100.0).is_none());

        let transformed_sphere =
            Transformed::new(DMat4::from_scale(DVec3::splat(2.0)), Arc::new(sphere));

        assert!(transformed_sphere.hit(&ray, 0.0, 100.0).is_some());
    }
}

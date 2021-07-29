use super::{HitRecord, Ray, AABB};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB>;
}

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
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        if self.len() <= 0 {
            return None;
        }

        let first_box = self[0].bounding_box(time_0, time_1);
        if first_box.is_none() {
            return None;
        }

        self.iter().skip(1).fold(first_box, |bbox, hittable| {
            return Some(AABB::surrounding_box(
                &bbox?,
                &hittable.bounding_box(time_0, time_1)?,
            ));
        })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{Lambertian, Sphere, Vec3};

    use super::*;

    #[test]
    fn test_single_bounding_box() {
        let material = Rc::new(Lambertian {
            albedo: Vec3::new(0.0, 0.0, 0.0),
        });
        let world: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.5,
            material: material.clone(),
        })];

        let bbox = world.as_slice().bounding_box(0.0, 0.0);

        assert!(bbox.is_some());

        let bbox = bbox.unwrap();

        assert_eq!(
            bbox,
            AABB {
                min: Vec3::new(-0.5, -0.5, -0.5),
                max: Vec3::new(0.5, 0.5, 0.5)
            }
        )
    }

    #[test]
    fn test_overlap_bounding_box() {
        let material = Rc::new(Lambertian {
            albedo: Vec3::new(0.0, 0.0, 0.0),
        });
        let world: Vec<Box<dyn Hittable>> = vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.5,
                material: material.clone(),
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.2,
                material: material.clone(),
            }),
        ];

        let bbox = world.as_slice().bounding_box(0.0, 0.0);

        assert!(bbox.is_some());

        let bbox = bbox.unwrap();

        assert_eq!(
            bbox,
            AABB {
                min: Vec3::new(-0.5, -0.5, -0.5),
                max: Vec3::new(0.5, 0.5, 0.5)
            }
        )
    }

    #[test]
    fn test_two_bounding_box() {
        let material = Rc::new(Lambertian {
            albedo: Vec3::new(0.0, 0.0, 0.0),
        });
        let world: Vec<Box<dyn Hittable>> = vec![
            Box::new(Sphere {
                center: Vec3::new(-0.5, -0.5, -0.5),
                radius: 0.5,
                material: material.clone(),
            }),
            Box::new(Sphere {
                center: Vec3::new(0.5, 0.5, 0.5),
                radius: 0.5,
                material: material.clone(),
            }),
        ];

        let bbox = world.as_slice().bounding_box(0.0, 0.0);

        assert!(bbox.is_some());

        let bbox = bbox.unwrap();

        assert_eq!(
            bbox,
            AABB {
                min: Vec3::new(-1.0, -1.0, -1.0),
                max: Vec3::new(1.0, 1.0, 1.0)
            }
        )
    }
}

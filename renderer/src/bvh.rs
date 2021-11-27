use std::{cmp::Ordering, sync::Arc};

use super::{HitRecord, Hittable, Triangle, AABB};
use rand::{self, Rng};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_mesh(triangles: Vec<Triangle>, time_0: f64, time_1: f64) -> BVHNode {
        let hittables = triangles
            .into_iter()
            .map(|triangle| Arc::new(triangle) as Arc<dyn Hittable>)
            .collect::<Vec<Arc<dyn Hittable>>>();

        BVHNode::new(&hittables.as_slice(), time_0, time_1)
    }

    pub fn new(hittables: &[Arc<dyn Hittable>], time_0: f64, time_1: f64) -> BVHNode {
        if hittables.len() == 1 {
            return BVHNode {
                left: hittables[0].clone(),
                right: hittables[0].clone(),
                bbox: hittables.bounding_box(time_0, time_1).unwrap(),
            };
        }

        if hittables.len() == 2 {
            return BVHNode {
                left: hittables[0].clone(),
                right: hittables[1].clone(),
                bbox: hittables.bounding_box(time_0, time_1).unwrap(),
            };
        }

        let sort_axis: usize = rand::thread_rng().gen_range(0..=2);

        let mut objects = hittables.to_vec();
        objects.sort_by(|h1, h2| {
            let bbox1 = h1.bounding_box(time_0, time_1);
            let bbox2 = h2.bounding_box(time_0, time_1);
            if bbox1.is_none() {
                return Ordering::Less;
            }

            if bbox2.is_none() {
                return Ordering::Greater;
            }

            let bbox1 = bbox1.unwrap();
            let bbox2 = bbox2.unwrap();

            return bbox1.min[sort_axis]
                .partial_cmp(&bbox2.min[sort_axis])
                .unwrap_or(Ordering::Less);
        });

        let midpoint = objects.len() / 2;
        let left = Arc::new(BVHNode::new(&objects[0..midpoint], time_0, time_1));
        let right = Arc::new(BVHNode::new(&objects[midpoint..], time_0, time_1));

        let left_bbox = left.bounding_box(time_0, time_1);
        let right_bbox = right.bounding_box(time_0, time_1);

        BVHNode {
            left,
            right,
            bbox: AABB::surrounding_box(&left_bbox.unwrap(), &right_bbox.unwrap()),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);

        let t_max = match &left_hit {
            Some(hr) => hr.t,
            None => t_max,
        };

        let right_hit = self.right.hit(ray, t_min, t_max);

        return right_hit.or(left_hit);
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

#[cfg(test)]
mod tests {
    use glam::{DVec2, DVec3};

    use crate::{create_mesh, Lambertian, Ray, SolidColour};

    use super::*;

    #[test]
    fn intersect() {
        let vertices = vec![
            DVec3::new(-1., -1., 0.),
            DVec3::new(1., 1., 0.),
            DVec3::new(-1., 1., 0.),
        ];
        let normals = vec![
            DVec3::new(0., 0., 1.),
            DVec3::new(0., 0., 1.),
            DVec3::new(0., 0., 1.),
        ];
        let uv = vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(1.0, 1.0),
            DVec2::new(0.0, 1.0),
        ];
        let indices = vec![[0, 1, 2]];

        let material = Arc::new(Lambertian {
            albedo: Arc::new(SolidColour {
                colour: DVec3::splat(0.5),
            }),
        });
        let triangles = create_mesh(vertices, normals, uv, indices, material);
        let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
        triangles
            .into_iter()
            .map(|triangle| Arc::new(triangle) as Arc<dyn Hittable>)
            .for_each(|triangle| world.push(triangle));

        let world_as_slice = world.as_slice();
        let bvh = BVHNode::new(world_as_slice, 0.0, 0.0);

        let ray = Ray {
            origin: DVec3::new(-0.5, 0.0, 1.0),
            dir: DVec3::new(0., 0., -1.),
            time: 0.0,
        };

        let hit = bvh.hit(&ray, 0.00001, 10000.);

        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 1.0);
    }
}

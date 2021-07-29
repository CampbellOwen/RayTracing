use std::{cmp::Ordering, rc::Rc};

use super::{HitRecord, Hittable, AABB};
use rand::{self, Rng};

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittables: &[Rc<dyn Hittable>], time_0: f64, time_1: f64) -> BVHNode {
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
        let left = Rc::new(BVHNode::new(&objects[0..midpoint], time_0, time_1));
        let right = Rc::new(BVHNode::new(&objects[midpoint..], time_0, time_1));

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

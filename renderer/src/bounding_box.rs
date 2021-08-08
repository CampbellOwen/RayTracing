use super::Ray;
use glam::DVec3;

#[derive(Clone, Debug, PartialEq)]
pub struct AABB {
    pub min: DVec3,
    pub max: DVec3,
}

impl AABB {
    pub fn surrounding_box(box_1: &AABB, box_2: &AABB) -> AABB {
        AABB {
            min: DVec3::new(
                f64::min(box_1.min.x, box_2.min.x),
                f64::min(box_1.min.y, box_2.min.y),
                f64::min(box_1.min.z, box_2.min.z),
            ),

            max: DVec3::new(
                f64::max(box_1.max.x, box_2.max.x),
                f64::max(box_1.max.y, box_2.max.y),
                f64::max(box_1.max.z, box_2.max.z),
            ),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

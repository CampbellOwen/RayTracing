use glam::DVec3;
pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.dir * t
    }
}

use glam::DVec3;

pub struct OrthoNormalBasis {
    pub u: DVec3,
    pub v: DVec3,
    pub w: DVec3,
}

impl OrthoNormalBasis {
    pub fn from_w(w: &DVec3) -> Self {
        let a = if w.x.abs() > 0.9 {
            DVec3::new(0.0, 1.0, 0.0)
        } else {
            DVec3::new(1.0, 0.0, 0.0)
        };

        let v = w.cross(a).normalize();
        let u = w.cross(v);

        Self { u, v, w: *w }
    }

    pub fn local(&self, vec: &DVec3) -> DVec3 {
        (vec.x * self.u) + (vec.y * self.v) + (vec.z * self.w)
    }
}

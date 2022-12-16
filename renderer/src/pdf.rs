use glam::DVec3;

use crate::{rand_cosine_hemisphere, OrthoNormalBasis};

pub trait PDF {
    fn value(&self, direction: DVec3) -> f64;
    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3;
}

pub struct CosineWeightedHemispherePDF {
    basis: OrthoNormalBasis,
}

impl CosineWeightedHemispherePDF {
    pub fn new(normal: DVec3) -> Self {
        CosineWeightedHemispherePDF {
            basis: OrthoNormalBasis::from_w(&normal),
        }
    }
}

impl PDF for CosineWeightedHemispherePDF {
    fn value(&self, direction: DVec3) -> f64 {
        let cosine = direction.normalize().dot(self.basis.w);

        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        self.basis.local(&rand_cosine_hemisphere(rng))
    }
}

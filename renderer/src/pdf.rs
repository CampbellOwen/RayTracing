use glam::DVec3;
use rand::Rng;

use crate::{
    material::reflectance, rand_cosine_hemisphere, rand_in_unit_sphere, reflect, refract,
    OrthoNormalBasis, Ray,
};

pub trait PDF {
    fn value(&self, direction: DVec3) -> Option<f64>;
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
    fn value(&self, direction: DVec3) -> Option<f64> {
        let cosine = direction.normalize().dot(self.basis.w);

        if cosine <= 0.0 {
            Some(0.0)
        } else {
            Some(cosine / std::f64::consts::PI)
        }
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        self.basis.local(&rand_cosine_hemisphere(rng)).normalize()
    }
}

pub struct DiracDeltaPDF {
    dir: DVec3,
}

impl DiracDeltaPDF {
    pub fn new(dir: DVec3) -> Self {
        DiracDeltaPDF {
            dir: dir.normalize(),
        }
    }
}

impl PDF for DiracDeltaPDF {
    fn value(&self, direction: DVec3) -> Option<f64> {
        // To signal that you shouldn't be dividing your sample by the PDF for this distribution - it's a reflection
        None
    }

    fn generate(&self, _: &mut dyn rand::RngCore) -> DVec3 {
        self.dir
    }
}

pub struct FuzzyDiracDeltaPDF {
    pdf: DiracDeltaPDF,
    fuzziness: f64,
}

impl FuzzyDiracDeltaPDF {
    pub fn new(dir: DVec3, fuzziness: f64) -> Self {
        Self {
            pdf: DiracDeltaPDF::new(dir),
            fuzziness,
        }
    }
}

impl PDF for FuzzyDiracDeltaPDF {
    fn value(&self, direction: DVec3) -> Option<f64> {
        self.pdf.value(direction)
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        (self.pdf.generate(rng) + (rand_in_unit_sphere(rng) * self.fuzziness)).normalize()
    }
}

pub struct DielectricFresnelPDF {
    reflect_dir: DVec3,
    refract_dir: DVec3,
    reflectance: f64,
    cannot_refract: bool,
}

impl DielectricFresnelPDF {
    pub fn new(ray_in: &Ray, normal: DVec3, ior: f64) -> Self {
        let unit_direction = ray_in.dir.normalize();
        let cos_theta = (-unit_direction).dot(normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ior * sin_theta) > 1.0;
        let reflect_dir = reflect(unit_direction, normal).normalize();
        let refract_dir = refract(unit_direction, normal, ior).normalize();
        let reflectance_ratio = reflectance(cos_theta, ior);

        Self {
            reflect_dir,
            refract_dir,
            reflectance: reflectance_ratio,
            cannot_refract,
        }
    }
}

impl PDF for DielectricFresnelPDF {
    fn value(&self, direction: DVec3) -> Option<f64> {
        None
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        if self.cannot_refract || self.reflectance > rng.gen::<f64>() {
            self.reflect_dir
        } else {
            self.refract_dir
        }
    }
}

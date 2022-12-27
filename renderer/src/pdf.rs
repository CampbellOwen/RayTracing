use glam::DVec3;
use rand::{prelude::SliceRandom, Rng};

use crate::{
    material::reflectance, rand_cosine_hemisphere, rand_hemisphere, rand_in_unit_sphere, reflect,
    refract, OrthoNormalBasis, Ray,
};

pub trait PDF {
    fn value(&self, direction: DVec3) -> f64;
    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3;
    fn is_delta_distribution(&self) -> bool {
        false
    }
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
        self.basis.local(&rand_cosine_hemisphere(rng)).normalize()
    }
}

pub struct UniformHemispherePDF {
    basis: OrthoNormalBasis,
}

impl UniformHemispherePDF {
    pub fn new(normal: DVec3) -> Self {
        Self {
            basis: OrthoNormalBasis::from_w(&normal),
        }
    }
}

impl PDF for UniformHemispherePDF {
    fn value(&self, direction: DVec3) -> f64 {
        0.5 * std::f64::consts::FRAC_1_PI
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        self.basis.local(&rand_hemisphere(rng)).normalize()
    }
}

pub struct UniformSpherePDF {
    pub center: DVec3,
    pub radius: f64,
}

impl PDF for UniformSpherePDF {
    fn value(&self, _: DVec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI * (self.radius * self.radius))
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        (rand_in_unit_sphere(rng).normalize() * self.radius) + self.center
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
    fn value(&self, direction: DVec3) -> f64 {
        0.0
    }

    fn generate(&self, _: &mut dyn rand::RngCore) -> DVec3 {
        self.dir
    }
    fn is_delta_distribution(&self) -> bool {
        true
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
    fn value(&self, direction: DVec3) -> f64 {
        self.pdf.value(direction)
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        (self.pdf.generate(rng) + (rand_in_unit_sphere(rng) * self.fuzziness)).normalize()
    }

    fn is_delta_distribution(&self) -> bool {
        true
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
    fn value(&self, _: DVec3) -> f64 {
        0.0
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        if self.cannot_refract || self.reflectance > rng.gen::<f64>() {
            self.reflect_dir
        } else {
            self.refract_dir
        }
    }
    fn is_delta_distribution(&self) -> bool {
        true
    }
}

#[derive(PartialEq)]
pub enum MixtureMethod {
    Uniform,
    PowerHeuristic,
}

pub struct MixturePDF {
    pdfs: Vec<Box<dyn PDF>>,
    method: MixtureMethod,
}

impl MixturePDF {
    pub fn new(pdfs: Vec<Box<dyn PDF>>, method: MixtureMethod) -> Self {
        if method == MixtureMethod::PowerHeuristic && pdfs.len() > 2 {
            panic!("Power heuristic only implemented for 2 PDFS");
        }
        Self { pdfs, method }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: DVec3) -> f64 {
        match self.method {
            MixtureMethod::Uniform => {
                let weight = 1.0 / self.pdfs.len() as f64;
                self.pdfs
                    .iter()
                    .map(|pdf| pdf.value(direction) * weight)
                    .sum()
            }
            MixtureMethod::PowerHeuristic => todo!(),
        }
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        match self.method {
            MixtureMethod::Uniform => self
                .pdfs
                .choose(rng)
                .expect("Should always choose")
                .generate(rng)
                .normalize(),
            MixtureMethod::PowerHeuristic => todo!(),
        }
    }
}

pub struct UniformConePDF {
    basis: OrthoNormalBasis,
    cos_theta_max: f64,
}

impl UniformConePDF {
    pub fn new(normal: DVec3, cos_theta_max: f64) -> Self {
        Self {
            basis: OrthoNormalBasis::from_w(&normal),
            cos_theta_max,
        }
    }
}

impl PDF for UniformConePDF {
    fn value(&self, direction: DVec3) -> f64 {
        //let distrib_normal = self.basis.w;
        //let cos_theta = direction.dot(distrib_normal);

        //if cos_theta > (self.cos_theta_max) {
        //    return 0.0;
        //}

        1.0 / (2.0 * std::f64::consts::PI * (1.0 - self.cos_theta_max))
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> DVec3 {
        let r1: f64 = rng.gen();
        let cos_theta = (1.0 - r1) + (r1 * self.cos_theta_max);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        let phi = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
        self.basis.local(&DVec3::new(
            f64::cos(phi) * sin_theta,
            f64::sin(phi) * sin_theta,
            cos_theta,
        ))
    }
}

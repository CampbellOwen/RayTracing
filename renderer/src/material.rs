use std::sync::Arc;

use super::{HitRecord, Ray, SolidColour, Texture};
use crate::{
    hit, math::reflect, CosineWeightedHemispherePDF, DielectricFresnelPDF, FuzzyDiracDeltaPDF, PDF,
};
use glam::DVec3;

pub trait Material: std::fmt::Debug + Send + Sync {
    fn emitted(&self, _: f64, _: f64, _: DVec3) -> DVec3 {
        DVec3::new(0.0, 0.0, 0.0)
    }
    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Box<dyn PDF>>;
    fn brdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_out: &Ray) -> DVec3;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(colour: DVec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColour { colour }),
        }
    }
}

impl Material for Lambertian {
    fn scattering_pdf(&self, _: &Ray, hit_record: &HitRecord) -> Option<Box<dyn PDF>> {
        Some(Box::new(CosineWeightedHemispherePDF::new(
            hit_record.normal,
        )))
    }

    fn brdf(&self, _: &Ray, hit_record: &HitRecord, out_ray: &Ray) -> DVec3 {
        self.albedo
            .sample(hit_record.u, hit_record.v, hit_record.point)
            / std::f64::consts::PI
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: DVec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn brdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_out: &Ray) -> DVec3 {
        let cosine = ray_out.dir.dot(hit_record.normal);

        self.albedo / cosine // Divide by cos(theta) because we need to cancel out the cos(theta) from the rendering equation
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Box<dyn PDF>> {
        let reflected = reflect(ray_in.dir.normalize(), hit_record.normal);
        Some(Box::new(FuzzyDiracDeltaPDF::new(reflected, self.fuzz))) // Fuzziness makes this incorrect btw, no longer a dirac delta PDF
    }
}

#[derive(Debug)]
pub struct Dielectric {
    pub ior: f64,
}

/// Schlick's approximation
pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
}

impl Material for Dielectric {
    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Box<dyn PDF>> {
        let ior = if hit_record.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };
        Some(Box::new(DielectricFresnelPDF::new(
            ray_in,
            hit_record.normal,
            ior,
        )))
    }

    fn brdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_out: &Ray) -> DVec3 {
        let cosine = ray_out.dir.dot(hit_record.normal);

        DVec3::ONE / cosine // Divide by cos(theta) because we need to cancel out the cos(theta) from the rendering equation
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    pub emit_colour: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: DVec3) -> DVec3 {
        self.emit_colour.sample(u, v, p)
    }

    fn brdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_out: &Ray) -> DVec3 {
        DVec3::ZERO
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Box<dyn PDF>> {
        None
    }
}

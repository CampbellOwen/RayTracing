use std::sync::Arc;

use super::{HitRecord, Ray, SolidColour, Texture};
use crate::math::{near_zero, rand_in_unit_sphere, rand_unit_vector, reflect, refract};
use glam::DVec3;
use rand::Rng;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)>;
    fn emitted(&self, _: f64, _: f64, _: DVec3) -> DVec3 {
        DVec3::new(0.0, 0.0, 0.0)
    }
}

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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        let mut scatter_direction = hit_record.normal + rand_unit_vector();

        // Catch degenerate direction
        if near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let attenuation = self
            .albedo
            .sample(hit_record.u, hit_record.v, hit_record.point);

        Some((
            Ray {
                origin: hit_record.point,
                dir: scatter_direction,
                time: ray.time,
            },
            attenuation,
        ))
    }
}

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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        let reflected = reflect(ray.dir.normalize(), hit_record.normal);

        if reflected.dot(hit_record.normal) > 0.0 {
            return Some((
                Ray {
                    origin: hit_record.point,
                    dir: reflected + (rand_in_unit_sphere() * self.fuzz),
                    time: ray.time,
                },
                self.albedo,
            ));
        }
        return None;
    }
}

pub struct Dielectric {
    pub ior: f64,
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Schlick's approximation
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray.dir.normalize();

        let cos_theta = DVec3::dot(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some((
            Ray {
                origin: hit_record.point,
                dir: direction,
                time: ray.time,
            },
            DVec3::new(1.0, 1.0, 1.0),
        ))
    }
}

pub struct DiffuseLight {
    pub emit_colour: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Ray, DVec3)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: DVec3) -> DVec3 {
        self.emit_colour.sample(u, v, p)
    }
}

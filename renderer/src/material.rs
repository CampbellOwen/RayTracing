use std::rc::Rc;

use super::{HitRecord, Ray, SolidColour, Texture, Vec3};
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(colour: Vec3) -> Lambertian {
        Lambertian {
            albedo: Rc::new(SolidColour { colour }),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit_record.normal + Vec3::rand_unit_vector();

        // Catch degenerate direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let attenuation = self
            .albedo
            .sample(hit_record.u, hit_record.v, &hit_record.point);

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
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.dir.unit().reflect(&hit_record.normal);

        if reflected.dot(&hit_record.normal) > 0.0 {
            return Some((
                Ray {
                    origin: hit_record.point,
                    dir: reflected + (Vec3::rand_in_unit_sphere() * self.fuzz),
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray.dir.unit();

        let cos_theta = Vec3::dot(&-unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        Some((
            Ray {
                origin: hit_record.point,
                dir: direction,
                time: ray.time,
            },
            Vec3::new(1.0, 1.0, 1.0),
        ))
    }
}

pub struct DiffuseLight {
    emit_colour: Rc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit_colour.sample(u, v, p)
    }
}

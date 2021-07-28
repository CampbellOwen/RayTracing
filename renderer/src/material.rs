use super::{HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit_record.normal + Vec3::rand_unit_vector();

        // Catch degenerate direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((
            Ray {
                origin: hit_record.point,
                dir: scatter_direction,
            },
            self.albedo,
        ))
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.dir.unit().reflect(&hit_record.normal);

        if reflected.dot(&hit_record.normal) > 0.0 {
            return Some((
                Ray {
                    origin: hit_record.point,
                    dir: reflected,
                },
                self.albedo,
            ));
        }
        return None;
    }
}

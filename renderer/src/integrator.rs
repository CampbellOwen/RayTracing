use glam::DVec3;

use crate::{Hittable, Ray};

pub trait Integrator {
    fn ray_colour(
        &self,
        ray: Ray,
        background_colour: &fn(Ray) -> DVec3,
        world: &dyn Hittable,
        depth: i32,
    ) -> DVec3;
}

pub struct PathIntegrator {}

impl Integrator for PathIntegrator {
    fn ray_colour(
        &self,
        ray: Ray,
        background_colour: &fn(Ray) -> DVec3,
        world: &dyn Hittable,
        depth: i32,
    ) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }
        let mut rng = rand::thread_rng();

        if let Some(hr) = world.hit(&ray, 0.001, 100000.0) {
            let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

            if let Some(material_pdf) = hr.material.scattering_pdf(&ray, &hr) {
                let out_dir = material_pdf.generate(&mut rng);
                let ray_out = Ray {
                    origin: hr.point,
                    dir: out_dir,
                    time: ray.time,
                };
                let cos_theta = out_dir.dot(hr.normal);

                let pdf = material_pdf.value(out_dir).unwrap_or(1.0);
                let brdf = hr.material.brdf(&ray, &hr, &ray_out);

                emitted
                    + (brdf
                        * cos_theta
                        * self.ray_colour(ray_out, background_colour, world, depth - 1))
                        / pdf
            } else {
                emitted
            }

            //if let Some(SurfaceRecord {
            //    ray: scattered_ray,
            //    albedo,
            //    pdf,
            //}) = hr.material.scatter(&ray, &hr)
            //{
            //    let cos_theta = scattered_ray.dir.dot(hr.normal);

            //    if let Some(mat_pdf) = pdf {
            //        emitted
            //            + (albedo
            //                * cos_theta
            //                * self.ray_colour(
            //                    scattered_ray.clone(),
            //                    background_colour,
            //                    world,
            //                    depth - 1,
            //                ))
            //                / mat_pdf.value(scattered_ray.dir)
            //    } else {
            //        emitted
            //            + (albedo
            //                * self.ray_colour(scattered_ray, background_colour, world, depth - 1))
            //    }
            //} else {
            //    emitted
            //}
        } else {
            background_colour(ray)
        }
    }
}

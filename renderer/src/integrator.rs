use glam::DVec3;

use crate::{Hittable, Ray, Scene, UniformHemispherePDF};

pub trait Integrator {
    fn ray_colour(&self, ray: Ray, scene: &Scene, depth: i32) -> DVec3;
}

pub struct PathIntegrator {}

impl Integrator for PathIntegrator {
    fn ray_colour(
        &self,
        ray: Ray,
        scene: &Scene,
        //background_colour: &fn(Ray) -> DVec3,
        //world: &dyn Hittable,
        depth: i32,
    ) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }
        let mut rng = rand::thread_rng();

        if let Some(hr) = scene.hit(&ray, 0.001, 100000.0) {
            let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

            if let Some(material_pdf) = hr.material.scattering_pdf(&ray, &hr) {
                let scatter_pdf = material_pdf;

                //if material_pdf.is_delta_distribution() {
                //    material_pdf
                //} else {
                //    Box::new(UniformHemispherePDF::new(hr.normal))
                //    //material_pdf
                //};
                let out_dir = scatter_pdf.generate(&mut rng);
                let ray_out = Ray {
                    origin: hr.point,
                    dir: out_dir,
                    time: ray.time,
                };
                let cos_theta = out_dir.dot(hr.normal);

                let pdf = if scatter_pdf.is_delta_distribution() {
                    1.0
                } else {
                    scatter_pdf.value(out_dir)
                };
                let brdf = hr.material.brdf(&ray, &hr, &ray_out);

                emitted + (brdf * cos_theta * self.ray_colour(ray_out, scene, depth - 1)) / pdf
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
            (scene.background)(ray)
        }
    }
}

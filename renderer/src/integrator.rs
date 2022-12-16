use glam::DVec3;

use crate::{material::ScatterRecord, Hittable, Ray};

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

        if let Some(hr) = world.hit(&ray, 0.001, 100000.0) {
            let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

            if let Some(ScatterRecord {
                ray: scattered_ray,
                albedo,
                pdf,
            }) = hr.material.scatter(&ray, &hr)
            {
                if let Some(mat_pdf) = pdf {
                    emitted
                        + (albedo
                            * self.ray_colour(
                                scattered_ray.clone(),
                                background_colour,
                                world,
                                depth - 1,
                            ))
                            / mat_pdf.value(scattered_ray.dir)
                } else {
                    emitted
                        + (albedo
                            * self.ray_colour(scattered_ray, background_colour, world, depth - 1))
                }
            } else {
                emitted
            }
        } else {
            background_colour(ray)
        }
    }
}

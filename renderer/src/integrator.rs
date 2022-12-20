use glam::DVec3;
use rand::seq::SliceRandom;

use crate::{Hittable, MixturePDF, Ray, Scene, UniformHemispherePDF};

pub trait Integrator {
    fn ray_colour(&self, ray: Ray, scene: &Scene, depth: i32) -> DVec3;
}

pub fn power_heuristic(num_f: u32, pdf_f: f64, num_g: u32, pdf_g: f64) -> f64 {
    let f = num_f as f64 * pdf_f;
    let g = num_g as f64 * pdf_g;

    (f * f) / ((f * f) + (g * g))
}
pub struct MultipleImportanceSampleIntegrator {}

impl Integrator for MultipleImportanceSampleIntegrator {
    fn ray_colour(&self, ray: Ray, scene: &Scene, depth: i32) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }
        let mut rng = rand::thread_rng();

        let hit = scene.hit(&ray, 0.001, 100000.0);
        if hit.is_none() {
            return (scene.background)(ray);
        }

        let hr = hit.unwrap();

        let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

        let material_pdf = hr.material.scattering_pdf(&ray, &hr);
        if material_pdf.is_none() {
            return emitted;
        }

        let material_pdf = material_pdf.unwrap();

        if material_pdf.is_delta_distribution() {
            let ray_out = Ray {
                origin: hr.point,
                dir: material_pdf.generate(&mut rng).normalize(),
                time: ray.time,
            };
            let cos_theta = ray_out.dir.dot(hr.normal);

            return emitted
                + (hr.material.brdf(&ray, &hr, &ray_out)
                    * cos_theta
                    * self.ray_colour(ray_out, scene, depth - 1));
        }

        let light = scene.lights.choose(&mut rng).and_then(|light| {
            let light_pdf = light.pdf_for_point(hr.point);
            let dir = light_pdf.generate(&mut rng).normalize();

            let visibility_ray = Ray {
                origin: hr.point,
                dir,
                time: ray.time,
            };

            if hr.normal.dot(dir) < 0.0 {
                return None;
            }

            let visiblity_hit = scene.hit(&visibility_ray, 0.001, 10000.0)?;

            if (visiblity_hit.t - light.hit(&visibility_ray, 0.001, 10000.0)?.t).abs() > 0.0001 {
                return None;
            }
            Some((light, light_pdf, visibility_ray))
        });

        if let Some((light, light_pdf, light_ray)) = light {
            // HAVE LIGHT AND IS VISIBLE

            let light_pdf_value = light_pdf.value(light_ray.dir);

            let material_out = Ray {
                origin: hr.point,
                dir: material_pdf.generate(&mut rng).normalize(),
                time: ray.time,
            };

            let material_cos_theta = material_out.dir.dot(hr.normal);
            let material_out_pdf = material_pdf.value(material_out.dir);
            let material_weight = power_heuristic(1, material_out_pdf, 1, light_pdf_value);

            let material_contribution = hr.material.brdf(&ray, &hr, &material_out)
                * material_cos_theta
                * self.ray_colour(material_out, scene, depth - 1)
                * material_weight
                / material_out_pdf;

            let light_cos_theta = hr.normal.dot(light_ray.dir);
            let light_weight = power_heuristic(1, light_pdf_value, 1, material_out_pdf);
            let light_contribution = hr.material.brdf(&ray, &hr, &light_ray)
                * light_cos_theta
                * self.ray_colour(light_ray, scene, 1)
                * light_weight
                / light_pdf_value;

            emitted + material_contribution + light_contribution
        } else {
            let ray_out = Ray {
                origin: hr.point,
                dir: material_pdf.generate(&mut rng).normalize(),
                time: ray.time,
            };
            let cos_theta = ray_out.dir.dot(hr.normal);
            let pdf = material_pdf.value(ray_out.dir);

            emitted
                + (hr.material.brdf(&ray, &hr, &ray_out)
                    * cos_theta
                    * self.ray_colour(ray_out, scene, depth - 1))
                    / pdf
        }
    }
}
pub struct ImportanceSampleLightIntegrator {}

impl Integrator for ImportanceSampleLightIntegrator {
    fn ray_colour(&self, ray: Ray, scene: &Scene, depth: i32) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }
        let mut rng = rand::thread_rng();

        if let Some(hr) = scene.hit(&ray, 0.001, 100000.0) {
            let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

            if let Some(material_pdf) = hr.material.scattering_pdf(&ray, &hr) {
                let scatter_pdf = if material_pdf.is_delta_distribution() {
                    material_pdf
                } else {
                    let lights = &scene.lights;
                    if let Some(light) = lights.choose(&mut rng) {
                        Box::new(MixturePDF::new(
                            vec![/*material_pdf,*/ light.pdf_for_point(hr.point)],
                            crate::MixtureMethod::Uniform,
                        ))
                    } else {
                        material_pdf
                    }
                };
                let mut out_dir = scatter_pdf.generate(&mut rng);
                let mut ray_out = Ray {
                    origin: hr.point,
                    dir: out_dir,
                    time: ray.time,
                };
                let mut cos_theta = out_dir.dot(hr.normal);

                while cos_theta < 0.0 {
                    out_dir = scatter_pdf.generate(&mut rng);
                    ray_out = Ray {
                        origin: hr.point,
                        dir: out_dir,
                        time: ray.time,
                    };
                    cos_theta = out_dir.dot(hr.normal);
                }

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

pub struct BRDFSampledPathIntegrator {}
impl Integrator for BRDFSampledPathIntegrator {
    fn ray_colour(&self, ray: Ray, scene: &Scene, depth: i32) -> DVec3 {
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

pub struct UniformSampledPathIntegrator {}
impl Integrator for UniformSampledPathIntegrator {
    fn ray_colour(&self, ray: Ray, scene: &Scene, depth: i32) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }
        let mut rng = rand::thread_rng();

        if let Some(hr) = scene.hit(&ray, 0.001, 100000.0) {
            let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

            if let Some(material_pdf) = hr.material.scattering_pdf(&ray, &hr) {
                let scatter_pdf = if material_pdf.is_delta_distribution() {
                    material_pdf
                } else {
                    Box::new(MixturePDF::new(
                        vec![material_pdf, Box::new(UniformHemispherePDF::new(hr.normal))],
                        crate::MixtureMethod::Uniform,
                    ))
                };
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

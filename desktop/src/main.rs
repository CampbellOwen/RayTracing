use exporters::ppm::write_image;
use rand::Rng;
use renderer::{
    ray_colour, BVHNode, Camera, Dielectric, Hittable, Image, Lambertian, Material, Metal,
    MovingSphere, Sphere, Vec3,
};
use std::{io, io::Write, rc::Rc};

mod exporters;

#[allow(dead_code)]
fn create_simple_scene() -> Vec<Rc<dyn Hittable>> {
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });

    let center_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });

    let shiny_metal_material = Rc::new(Metal::new(Vec3::new(0.1, 0.1, 0.1), 0.0));

    let left_material = Rc::new(Dielectric { ior: 1.5 });

    let right_material: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    vec![
        Rc::new(Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: center_material.clone(),
        }),
        Rc::new(Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: left_material.clone(),
        }),
        Rc::new(Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.49,
            material: left_material.clone(),
        }),
        Rc::new(Sphere {
            center: Vec3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: right_material.clone(),
        }),
        Rc::new(Sphere {
            center: Vec3 {
                x: 0.3,
                y: -0.2,
                z: -0.5,
            },
            radius: 0.2,
            material: shiny_metal_material.clone(),
        }),
        Rc::new(Sphere {
            center: Vec3 {
                x: -0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: ground_material.clone(),
        }),
    ]
}

#[allow(dead_code)]
fn create_random_scene() -> Vec<Rc<dyn Hittable>> {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let ground_material = Rc::new(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });

    world.push(Rc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = rng.gen::<f64>();
            let center = Vec3 {
                x: (a as f64) + (0.9 * rng.gen::<f64>()),
                y: 0.2,
                z: (b as f64) + (0.9 * rng.gen::<f64>()),
            };

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if material_choice < 0.8 {
                    // Diffuse material
                    let albedo = Vec3::random() * Vec3::random();
                    let material = Rc::new(Lambertian { albedo });

                    let center_2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);

                    world.push(Rc::new(MovingSphere {
                        center_0: center,
                        center_1: center_2,
                        time_0: 0.0,
                        time_1: 1.0,
                        radius: 0.2,
                        material,
                    }));
                } else if material_choice < 0.95 {
                    // Metal
                    let albedo = Vec3::rand_in_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }));
                } else {
                    // Glass
                    let material = Rc::new(Dielectric { ior: 1.5 });
                    world.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }));
                }
            }
        }
    }

    let material = Rc::new(Dielectric { ior: 1.5 });
    world.push(Rc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let material = Rc::new(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    world.push(Rc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let material = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Rc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    return world;
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 200;
    let height = (width as f64 / aspect_ratio) as u32;

    let mut img = Image::new((width, height));

    let world = create_random_scene();
    //let world = create_simple_scene();

    let bvh = BVHNode::new(world.as_slice(), 0.0, 0.0);

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut rng = rand::thread_rng();

    for y in 0..img.size.1 {
        if y % 10 == 0 {
            print!("\rRendered {:0>4} lines", y);
            let _ = io::stdout().flush();
        }
        for x in 0..img.size.0 {
            let mut colour = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + rng.gen::<f64>()) / (height - 1) as f64;

                let ray = camera.get_ray(u, v);
                colour = colour + ray_colour(&ray, &bvh, max_depth);
            }

            colour = colour / (samples_per_pixel as f64);
            img.put(x, y, &colour);
        }
    }

    println!("\nSaving image");
    write_image(&img, "output.ppm").expect("Writing image failed");
}

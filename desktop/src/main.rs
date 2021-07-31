use exporters::ppm::write_image;
extern crate rand;
use rand::Rng;
use renderer::{
    ray_colour, AARect, BVHNode, Camera, CheckerTexture, Dielectric, DiffuseLight, Hittable, Image,
    Lambertian, Material, Metal, MovingSphere, Ray, SolidColour, Sphere, Vec3,
};
use std::{io, io::Write, rc::Rc};

mod exporters;

extern crate image;

type SceneDescription = (Vec<Rc<dyn Hittable>>, Camera, Box<dyn Fn(&Ray) -> Vec3>);

fn skybox(ray: &Ray) -> Vec3 {
    let unit_dir = ray.dir.unit();
    let t = 0.5 * unit_dir.y + 1.0;

    let white = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let blue = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    white * (1.0 - t) + blue * t
}

fn no_light(_: &Ray) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

#[allow(dead_code)]
fn create_simple_scene() -> SceneDescription {
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Rc::new(CheckerTexture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        )),
    });

    let center_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Rc::new(load_texture("textures/earthmap.jpg").unwrap()),
    });

    let shiny_metal_material = Rc::new(Metal::new(Vec3::new(0.1, 0.1, 0.1), 0.0));

    let left_material = Rc::new(Dielectric { ior: 1.5 });

    let right_material: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let world: Vec<Rc<dyn Hittable>> = vec![
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
    ];

    let aspect_ratio = 16.0 / 9.0;
    let look_from = Vec3::new(-5.0, 0.8, -3.5);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (look_at - look_from).length();
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        30.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, Box::new(skybox));
}

#[allow(dead_code)]
fn create_random_scene() -> SceneDescription {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

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
                    let material = Rc::new(Lambertian::new(albedo));

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

    let material = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
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

    let aspect_ratio = 16.0 / 9.0;
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

    return (world, camera, Box::new(skybox));
}

#[allow(dead_code)]
fn two_spheres() -> SceneDescription {
    let material = Rc::new(Lambertian {
        albedo: Rc::new(CheckerTexture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        )),
    });

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = 16.0 / 9.0;
    let aperture = 0.1;
    let focus_dist = (look_from - look_at).length();
    return (
        vec![
            Rc::new(Sphere {
                center: Vec3::new(0.0, -10.0, 0.0),
                radius: 10.0,
                material: material.clone(),
            }),
            Rc::new(Sphere {
                center: Vec3::new(0.0, 10.0, 0.0),
                radius: 10.0,
                material: material.clone(),
            }),
            Rc::new(Sphere {
                center: Vec3::new(10.0, 2.0, 0.5),
                radius: 1.0,
                material: Rc::new(DiffuseLight {
                    emit_colour: Rc::new(SolidColour {
                        colour: Vec3::new(4.0, 4.0, 4.0),
                    }),
                }),
            }),
        ],
        Camera::new_instant(
            look_from,
            look_at,
            up,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        ),
        Box::new(no_light),
    );
}

#[allow(dead_code)]
fn simple_light() -> SceneDescription {
    let world: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Rc::new(Lambertian::new(Vec3::new(0.9, 0.9, 0.9))),
        }),
        Rc::new(Sphere {
            center: Vec3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Rc::new(Lambertian {
                albedo: Rc::new(load_texture("textures/earthmap.jpg").unwrap()),
            }),
        }),
        Rc::new(Sphere {
            center: Vec3::new(0.0, 6.5, 0.0),
            radius: 2.0,
            material: Rc::new(DiffuseLight {
                emit_colour: Rc::new(SolidColour {
                    colour: Vec3::new(4.0, 4.0, 4.0),
                }),
            }),
        }),
        Rc::new(AARect {
            x_range: (3.0, 5.0),
            y_range: (1.0, 3.0),
            z: -2.0,
            material: Rc::new(DiffuseLight {
                emit_colour: Rc::new(SolidColour {
                    colour: Vec3::new(4.0, 4.0, 4.0),
                }),
            }),
        }),
    ];

    let look_from = Vec3::new(26.0, 3.0, 6.0);
    let look_at = Vec3::new(0.0, 2.0, 0.0);

    let camera = Camera::new_instant(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        (look_at - look_from).length(),
    );

    return (world, camera, Box::new(no_light));
}

fn load_texture(filename: &str) -> Option<Image> {
    let tex = image::open(filename).ok()?.into_rgb8();
    let mut tex_image = Image::new(tex.dimensions());
    for y in 0..tex.dimensions().1 {
        for x in 0..tex.dimensions().0 {
            let pixel = tex.get_pixel(x, y);
            tex_image.put(
                x,
                y,
                &Vec3::new(
                    pixel[0] as f64 / 255.0,
                    pixel[1] as f64 / 255.0,
                    pixel[2] as f64 / 255.0,
                ),
            );
        }
    }

    Some(tex_image)
}

fn main() {
    let width = 1080;
    let aspect_ratio = 16.0 / 9.0;
    let height = (width as f64 / aspect_ratio) as u32;

    let mut img = Image::new((width, height));

    //let (world, camera) = create_random_scene();
    //let (world, camera, background_colour) = create_simple_scene();
    //let (world, camera, background_colour) = two_spheres();
    let (world, camera, background_colour) = simple_light();

    let bvh = BVHNode::new(world.as_slice(), 0.0, 0.0);

    let samples_per_pixel = 10000;
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
                colour = colour + ray_colour(&ray, &background_colour, &bvh, max_depth);
            }

            colour = colour / (samples_per_pixel as f64);
            img.put(x, y, &colour);
        }
    }

    println!("\nSaving image");
    write_image(&img, "output.ppm").expect("Writing image failed");
}

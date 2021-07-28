use exporters::ppm::write_image;
use rand::Rng;
use renderer::{
    ray_colour, Camera, Dielectric, Hittable, Image, Lambertian, Material, Metal, Sphere, Vec3,
};
use std::{io, io::Write, rc::Rc};

mod exporters;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;

    let mut img = Image::new((width, height));

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });

    let center_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });

    let shiny_metal_material = Rc::new(Metal::new(Vec3::new(0.1, 0.1, 0.1), 0.0));

    //let center_material = Rc::new(Dielectric { ior: 1.5 });

    //let left_material: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let left_material = Rc::new(Dielectric { ior: 1.5 });

    let right_material: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: center_material.clone(),
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: left_material.clone(),
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.4,
            material: left_material.clone(),
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: right_material.clone(),
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: 0.3,
                y: -0.2,
                z: -0.5,
            },
            radius: 0.2,
            material: shiny_metal_material.clone(),
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: -0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: ground_material.clone(),
        }),
    ];

    let camera = Camera::new(90.0, aspect_ratio);

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
                colour = colour + ray_colour(&ray, &objects.as_slice(), max_depth);
            }

            colour = colour / (samples_per_pixel as f64);
            img.put(x, y, &colour);
        }
    }

    println!("\nSaving image");
    write_image(&img, "output.ppm").expect("Writing image failed");
}

use exporters::ppm::write_image;
use renderer::{ray_colour, Camera, Hittable, Image, Ray, Sphere, Vec3};

mod exporters;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let height = (width as f32 / aspect_ratio) as u32;

    let mut img = Image::new((width, height));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let objects: Vec<&dyn Hittable> = vec![
        &Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        },
        &Sphere {
            center: Vec3 {
                x: 1.0,
                y: 0.0,
                z: -3.0,
            },
            radius: 2.0,
        },
        &Sphere {
            center: Vec3 {
                x: -0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
        },
    ];

    let camera = Camera::new();

    for y in 0..img.size.1 {
        for x in 0..img.size.0 {
            let u = x as f32 / (width - 1) as f32;
            let v = y as f32 / (height - 1) as f32;

            let ray = camera.get_ray(u, v);
            let colour = ray_colour(&ray, &objects);
            img.put(x, y, &colour);
        }
    }

    write_image(&img, "output.ppm").expect("Writing image failed");
}

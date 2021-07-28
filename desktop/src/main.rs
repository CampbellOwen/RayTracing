use exporters::ppm::write_image;
use renderer::{ray_colour, Hittable, Image, Ray, Sphere, Vec3};

mod exporters;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let height = (width as f32 / aspect_ratio) as u32;

    let mut img = Image::new((width, height));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::origin();
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

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
                x: -5.0,
                y: 0.0,
                z: -10.0,
            },
            radius: 2.2,
        },
    ];

    for y in 0..img.size.1 {
        for x in 0..img.size.0 {
            //img.data.push((x as f32) / (img.size.0 - 1) as f32);
            //img.data.push((y as f32) / (img.size.1 - 1) as f32);
            //img.data.push(0.25);

            let u = x as f32 / (width - 1) as f32;
            let v = y as f32 / (height - 1) as f32;
            let ray = Ray {
                origin: origin,
                dir: lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            };
            let colour = ray_colour(&ray, &objects);
            img.put(x, y, &colour);
        }
    }

    write_image(&img, "output.ppm").expect("Writing image failed");
}

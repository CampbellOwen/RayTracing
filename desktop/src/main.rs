use exporters::ppm::write_image;
use renderer::{Image, Vec3};

mod exporters;

fn main() {
    let mut img = Image {
        size: (256, 256),
        data: Vec::new(),
    };

    for x in 0..img.size.0 {
        for y in 0..img.size.1 {
            img.data.push(Vec3 {
                x: (x as f32) / (img.size.0 - 1) as f32,
                y: (y as f32) / (img.size.1 - 1) as f32,
                z: 0.25,
            });
        }
    }

    write_image(&img, "output.ppm").expect("Writing image failed");

    println!("Hello, world!");
}

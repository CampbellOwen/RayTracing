use std::io::{self, BufWriter};
use std::{fs::File, io::Write};

use renderer::Image;

pub fn write_image(img: &Image, location: &str) -> Result<(), io::Error> {
    assert_eq!(img.size.0 as usize * img.size.1 as usize, img.data.len());

    println!("Writing PPM file {}", location);

    let file = File::create(location)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3\n{} {}\n255", img.size.0, img.size.1)?;

    for y in (0..img.size.1).rev() {
        for x in 0..img.size.0 {
            let pixel = match img.get(x, y) {
                Some(pixel) => pixel,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid pixel location: ({},{})", x, y),
                    ))
                }
            };

            let pixel = pixel.powf(0.5);
            let r = (pixel.x.clamp(0.0, 0.999) * 256.0) as u32;
            let g = (pixel.y.clamp(0.0, 0.999) * 256.0) as u32;
            let b = (pixel.z.clamp(0.0, 0.999) * 256.0) as u32;

            writeln!(writer, "{} {} {}", r, g, b)?;
        }
    }
    println!("\nDone!");

    Ok(())
}

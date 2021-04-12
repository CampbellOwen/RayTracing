use std::io;
use std::{fs::File, io::Write};

use renderer::Image;

pub fn write_image(img: &Image, location: &str) -> Result<(), io::Error> {
    assert_eq!(img.size.0 as usize * img.size.1 as usize, img.data.len());
    let mut file = File::create(&location)?;
    write!(file, "P3\n{} {}\n255\n", img.size.0, img.size.1)?;

    //for pixel in &img.data {
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
            let r = (pixel.x * 255.999) as u32;
            let g = (pixel.y * 255.999) as u32;
            let b = (pixel.z * 255.999) as u32;

            write!(file, "{} {} {}\n", r, g, b)?;
        }
    }

    Ok(())
}

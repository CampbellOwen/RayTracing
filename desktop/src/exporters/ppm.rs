use std::io;
use std::{fs::File, io::Write};

use renderer::Image;

pub fn write_image(img: &Image, location: &str) -> Result<(), io::Error> {
    assert_eq!(
        img.size.0 as usize * img.size.1 as usize * 3,
        img.data.len()
    );
    let mut file = File::create(&location)?;
    write!(file, "P3\n{} {}\n255\n", img.size.0, img.size.1)?;

    for y in (0..img.size.1).rev() {
        if y % 10 == 0 {
            print!("\r{:0>4} scanlines remaining", y);
            let _ = io::stdout().flush();
        }
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

            let pixel = pixel.sqrt();
            let r = (pixel.x.clamp(0.0, 0.999) * 256.0) as u32;
            let g = (pixel.y.clamp(0.0, 0.999) * 256.0) as u32;
            let b = (pixel.z.clamp(0.0, 0.999) * 256.0) as u32;

            write!(file, "{} {} {}\n", r, g, b)?;
        }
    }
    println!("\nDone!");

    Ok(())
}

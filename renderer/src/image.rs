use crate::Texture;

use glam::DVec3;

#[derive(Debug, Clone)]
pub struct Image {
    pub size: (u32, u32),
    pub data: Vec<DVec3>,
}

pub struct ImageTile {
    pub origin: (u32, u32),
    pub size: (u32, u32),
    pub pixels: Vec<DVec3>,
}

impl ImageTile {
    pub fn get_xy(&self, index: u32) -> (u32, u32) {
        (
            (index % self.size.0) + self.origin.0,
            (index / self.size.0) + self.origin.1,
        )
    }
}

impl Image {
    pub fn new(size: (u32, u32)) -> Image {
        let capacity: usize = size.0 as usize * size.1 as usize;
        Image {
            size: size,
            data: vec![DVec3::ZERO; capacity],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&DVec3> {
        let index: usize = (y * self.size.0) as usize + x as usize;

        return self.data.get(index);
    }

    pub fn put(&mut self, x: u32, y: u32, colour: &DVec3) {
        let index: usize = (y * self.size.0) as usize + x as usize;

        self.data[index] = *colour;
    }

    pub fn get_tile(&self, origin: (u32, u32), size: (u32, u32)) -> ImageTile {
        let actual_width = u32::min(origin.0 + size.0, self.size.0) - origin.0;
        let actual_height = u32::min(origin.1 + size.1, self.size.1) - origin.1;

        ImageTile {
            origin,
            size: (actual_width, actual_height),
            pixels: vec![DVec3::ZERO; (actual_width * actual_height) as usize],
        }
    }

    pub fn merge_tile(&mut self, tile: ImageTile) {
        let tile_size = tile.size.0 * tile.size.1;
        for i in 0..tile_size {
            let (x, y) = tile.get_xy(i);
            self.put(x, y, &tile.pixels[i as usize]);
        }
    }
}

impl Texture for Image {
    fn sample(&self, u: f64, v: f64, _: DVec3) -> DVec3 {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip vertical

        let x = (self.size.0 as f64 * u) as u32;
        let y = (self.size.1 as f64 * v) as u32;

        let x = x.clamp(0, self.size.0 - 1);
        let y = y.clamp(0, self.size.1 - 1);

        *self.get(x, y).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tile_works() {
        let img = Image::new((100, 100));

        let tile = img.get_tile((0, 0), (100, 100));

        assert_eq!(tile.origin, (0, 0));
        assert_eq!(tile.size, (100, 100));

        assert_eq!(tile.get_xy(0), (0, 0));
        assert_eq!(tile.get_xy((100 * 100) - 1), (99, 99));

        let tile = img.get_tile((50, 50), (50, 50));
        assert_eq!(tile.origin, (50, 50));
        assert_eq!(tile.size, (50, 50));

        assert_eq!(tile.get_xy(0), (50, 50));
        assert_eq!(tile.get_xy(49), (99, 50));
    }

    #[test]
    fn get_tile_on_boundary() {
        let img = Image::new((100, 100));
        let tile = img.get_tile((90, 0), (50, 50));

        assert_eq!(tile.origin, (90, 0));
        assert_eq!(tile.size, (10, 50));
        assert_eq!(tile.get_xy(10), (90, 1));
    }

    #[test]
    fn merge_tile_works() {
        let mut img = Image::new((4, 4));
        let mut tile = img.get_tile((0, 0), (2, 2));
        let mut tile_2 = img.get_tile((2, 2), (2, 2));

        for i in 0..4 {
            tile.pixels[i] = DVec3::new(10., 10., 10.);
            tile_2.pixels[i] = DVec3::new(100., 100., 100.);
        }

        img.merge_tile(tile);
        img.merge_tile(tile_2);

        assert_eq!(img.data[0], DVec3::new(10., 10., 10.));
        assert_eq!(img.data[1], DVec3::new(10., 10., 10.));
        assert_eq!(img.data[4], DVec3::new(10., 10., 10.));
        assert_eq!(img.data[5], DVec3::new(10., 10., 10.));

        assert_eq!(img.data[10], DVec3::new(100., 100., 100.));
        assert_eq!(img.data[11], DVec3::new(100., 100., 100.));
        assert_eq!(img.data[14], DVec3::new(100., 100., 100.));
        assert_eq!(img.data[15], DVec3::new(100., 100., 100.));

        assert_eq!(img.data[2], DVec3::ZERO);
        assert_eq!(img.data[3], DVec3::ZERO);
        assert_eq!(img.data[6], DVec3::ZERO);
        assert_eq!(img.data[7], DVec3::ZERO);
        assert_eq!(img.data[8], DVec3::ZERO);
        assert_eq!(img.data[9], DVec3::ZERO);
        assert_eq!(img.data[12], DVec3::ZERO);
        assert_eq!(img.data[13], DVec3::ZERO);
    }
}

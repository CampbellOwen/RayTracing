use super::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Image {
    pub size: (u32, u32),
    pub data: Vec<f64>,
}

impl Image {
    pub fn new(size: (u32, u32)) -> Image {
        let capacity: usize = size.0 as usize * size.1 as usize * 3 as usize;
        Image {
            size: size,
            data: vec![0.0; capacity],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Vec3> {
        let index: usize = (y * self.size.0) as usize + x as usize;
        let index = index * 3;

        let r = *self.data.get(index)?;
        let g = *self.data.get(index + 1)?;
        let b = *self.data.get(index + 2)?;

        return Some(Vec3 { x: r, y: g, z: b });
    }

    pub fn put(&mut self, x: u32, y: u32, colour: &Vec3) {
        let index: usize = (y * self.size.0) as usize + x as usize;
        let index = index * 3;

        self.data[index] = colour.x;
        self.data[index + 1] = colour.y;
        self.data[index + 2] = colour.z;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub size: (u32, u32),
    pub data: Vec<Vec3>,
}

impl Image {
    pub fn get(&self, x: u32, y: u32) -> Option<&Vec3> {
        let index: usize = (y * self.size.0) as usize + x as usize;

        return self.data.get(index);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

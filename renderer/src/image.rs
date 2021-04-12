use super::vec3::Vec3;

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

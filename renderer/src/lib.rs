mod image;
pub use image::Image;

mod vec3;
pub use vec3::Vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

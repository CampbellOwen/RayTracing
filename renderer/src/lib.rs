mod image;
pub use image::Image;

//mod vec3;
//pub use vec3::{cross, dot, Vec3};

use glam::DVec3;

mod ray;
pub use ray::Ray;

mod hit;
pub use hit::HitRecord;

mod shape;
pub use shape::*;

mod hittable;
pub use hittable::Hittable;

mod camera;
pub use camera::Camera;

mod material;
pub use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};

mod bounding_box;
pub use bounding_box::AABB;

mod bvh;
pub use bvh::BVHNode;

mod texture;
pub use texture::{CheckerTexture, SolidColour, Texture};

mod math;
pub use math::*;

mod mesh;
pub use mesh::*;

mod transform;
pub use transform::*;

mod integrator;
pub use integrator::*;

mod pdf;
pub use pdf::*;

mod onb;
pub use onb::*;

mod scene;
pub use scene::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

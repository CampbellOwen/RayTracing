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

pub fn ray_colour(
    ray: &Ray,
    background_colour: &dyn Fn(&Ray) -> DVec3,
    world: &dyn Hittable,
    depth: i32,
) -> DVec3 {
    if depth <= 0 {
        return DVec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(ray, 0.001, 100000.0) {
        let emitted = hr.material.emitted(hr.u, hr.v, hr.point);

        if let Some((scattered, attenuation)) = hr.material.scatter(ray, &hr) {
            return emitted
                + (attenuation * ray_colour(&scattered, background_colour, world, depth - 1));
        }

        return emitted;
    }

    background_colour(&ray)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

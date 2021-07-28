mod image;
pub use image::Image;

mod vec3;
pub use vec3::{cross, dot, Vec3};

mod ray;
pub use ray::Ray;

mod hit;
pub use hit::HitRecord;

mod shape;
pub use shape::Sphere;

mod hittable;
pub use hittable::Hittable;

mod camera;
pub use camera::Camera;

pub fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    let hr = world.hit(ray, 0.0, 100.0);
    if let Some(hr) = hr {
        return Vec3 {
            x: hr.normal.x + 1.0,
            y: hr.normal.y + 1.0,
            z: hr.normal.z + 1.0,
        } * 0.5;
    }

    let unit_dir = ray.dir.unit();
    let t = 0.5 * unit_dir.y + 1.0;

    let white = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let blue = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    white * (1.0 - t) + blue * t
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

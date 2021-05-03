mod image;
pub use image::Image;

mod vec3;
pub use vec3::{cross, dot, Vec3};

mod ray;
pub use ray::Ray;

mod hit;
pub use hit::{HitRecord, Hittable};

mod shape;
pub use shape::Sphere;

pub fn ray_colour(ray: &Ray) -> Vec3 {
    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    };
    let hr = sphere.hit(ray, 0.0, 100.0);
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

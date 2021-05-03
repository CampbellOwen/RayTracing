mod image;
pub use image::Image;

mod vec3;
pub use vec3::{cross, dot, Vec3};

mod ray;
pub use ray::Ray;

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    //let a = dot(&ray.dir, &ray.dir);
    //let b = 2.0 * dot(&oc, &ray.dir);
    //let c = dot(&oc, &oc) - radius * radius;
    //let discriminant = b * b - (4.0 * a * c);

    let a = ray.dir.length_squared();
    let half_b = dot(&oc, &ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (-half_b - discriminant.sqrt()) / a
}

pub fn ray_colour(ray: &Ray) -> Vec3 {
    let sphere = (
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
    );
    let t = hit_sphere(&sphere.0, sphere.1, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - sphere.0).unit();
        return Vec3 {
            x: normal.x + 1.0,
            y: normal.y + 1.0,
            z: normal.z + 1.0,
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

use super::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_dist);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random_in_lens = Vec3::rand_in_unit_sphere() * self.lens_radius;
        let offset = (self.u * random_in_lens.x) + (self.v * random_in_lens.y);

        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - self.origin
                - offset,
        }
    }
}

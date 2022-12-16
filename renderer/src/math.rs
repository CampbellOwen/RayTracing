use glam::DVec3;
use rand::Rng;

pub fn rand_in_range(rng: &mut dyn rand::RngCore, min: f64, max: f64) -> DVec3 {
    DVec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_in_unit_sphere(rng: &mut dyn rand::RngCore) -> DVec3 {
    loop {
        let point = rand_in_range(rng, -1.0, 1.0);
        if point.length() < 1.0 {
            return point;
        }
    }
}

pub fn rand_unit_vector(rng: &mut dyn rand::RngCore) -> DVec3 {
    rand_in_unit_sphere(rng).normalize()
}

pub fn rand_cosine_hemisphere(rng: &mut dyn rand::RngCore) -> DVec3 {
    let r1: f64 = rng.gen();
    let r2: f64 = rng.gen();
    let z = f64::sqrt(1.0 - r2);
    let phi = 2.0 * std::f64::consts::PI * r1;

    let sqrt_r2 = r2.sqrt();
    let x = f64::cos(phi) * sqrt_r2;
    let y = f64::sin(phi) * sqrt_r2;

    DVec3::new(x, y, z)
}

pub fn reflect(a: DVec3, b: DVec3) -> DVec3 {
    a - (b * a.dot(b) * 2.0)
}
pub fn near_zero(v: DVec3) -> bool {
    let epsilon = 1e-8;

    let v_abs = v.abs();

    (v_abs.x < epsilon) && (v_abs.y < epsilon) && (v_abs.z < epsilon)
}

pub fn refract(v: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = f64::min(DVec3::dot(-v, n), 1.0);
    let r_out_perp = (v + (n * cos_theta)) * etai_over_etat;
    let r_out_parallel = n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());

    r_out_perp + r_out_parallel
}

pub fn random() -> DVec3 {
    let mut rng = rand::thread_rng();

    DVec3::new(rng.gen(), rng.gen(), rng.gen())
}

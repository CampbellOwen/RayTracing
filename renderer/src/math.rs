use glam::DVec3;
use rand::Rng;

pub fn rand_in_range(min: f64, max: f64) -> DVec3 {
    let mut rng = rand::thread_rng();

    DVec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_in_unit_sphere() -> DVec3 {
    loop {
        let point = rand_in_range(-1.0, 1.0);
        if point.length() < 1.0 {
            return point;
        }
    }
}

pub fn rand_unit_vector() -> DVec3 {
    rand_in_unit_sphere().normalize()
}

pub fn reflect(a: DVec3, b: DVec3) -> DVec3 {
    a - (b * a.dot(b) * 2.0)
}
pub fn near_zero(v: DVec3) -> bool {
    let s = 1e-8;

    let v_abs = v.abs();

    return (v_abs.x < s) && (v_abs.y < s) && (v_abs.z < s);
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

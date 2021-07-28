use rand::Rng;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn rand_in_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let point = Vec3::rand_in_range(-1.0, 1.0);
            if point.length() < 1.0 {
                return point;
            }
        }
    }

    pub fn rand_unit_vector() -> Vec3 {
        Vec3::rand_in_unit_sphere().unit()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - (n * dot(self, n) * 2.0)
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(dot(&-self, n), 1.0);
        let r_out_perp = (self + (n * cos_theta)) * etai_over_etat;
        let r_out_parallel = n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());

        r_out_perp + r_out_parallel
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    u.cross(v)
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.dot(v)
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::{cross, dot, Vec3};
    #[test]
    fn test_dot() {
        assert_eq!(
            dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                }
            ),
            1.0
        );
        assert_eq!(
            dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                }
            ),
            0.0
        );
        assert_eq!(
            dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                }
            ),
            0.0
        );
        assert_eq!(
            dot(
                &Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                }
            ),
            0.0
        );
    }

    #[test]
    pub fn test_cross() {
        assert_eq!(
            cross(
                &Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                },
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                }
            ),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0
            }
        );
    }
}

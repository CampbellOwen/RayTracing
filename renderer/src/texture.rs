use std::sync::Arc;

use glam::DVec3;

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn sample(&self, u: f64, v: f64, p: DVec3) -> DVec3;
}

#[derive(Debug)]
pub struct SolidColour {
    pub colour: DVec3,
}

impl Texture for SolidColour {
    fn sample(&self, _: f64, _: f64, _: DVec3) -> DVec3 {
        self.colour
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: DVec3, c2: DVec3) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColour { colour: c1 }),
            even: Arc::new(SolidColour { colour: c2 }),
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, u: f64, v: f64, p: DVec3) -> DVec3 {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);

        if sines < 0.0 {
            self.odd.sample(u, v, p)
        } else {
            self.even.sample(u, v, p)
        }
    }
}

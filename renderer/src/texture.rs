use std::rc::Rc;

use glam::DVec3;

pub trait Texture {
    fn sample(&self, u: f64, v: f64, p: DVec3) -> DVec3;
}

pub struct SolidColour {
    pub colour: DVec3,
}

impl Texture for SolidColour {
    fn sample(&self, _: f64, _: f64, _: DVec3) -> DVec3 {
        self.colour
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: DVec3, c2: DVec3) -> CheckerTexture {
        CheckerTexture {
            odd: Rc::new(SolidColour { colour: c1 }),
            even: Rc::new(SolidColour { colour: c2 }),
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

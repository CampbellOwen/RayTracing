use std::rc::Rc;

use super::Vec3;

pub trait Texture {
    fn sample(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColour {
    pub colour: Vec3,
}

impl Texture for SolidColour {
    fn sample(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        self.colour
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Vec3, c2: Vec3) -> CheckerTexture {
        CheckerTexture {
            odd: Rc::new(SolidColour { colour: c1 }),
            even: Rc::new(SolidColour { colour: c2 }),
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);

        if sines < 0.0 {
            self.odd.sample(u, v, p)
        } else {
            self.even.sample(u, v, p)
        }
    }
}

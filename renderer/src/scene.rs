use std::sync::Arc;

use glam::DVec3;

use crate::{BVHNode, Camera, HitRecord, Hittable, Ray, AABB};

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Arc<dyn Hittable>>,
    pub lights: Vec<Arc<dyn Hittable>>,
    pub camera: Camera,
    pub background: fn(Ray) -> DVec3,
    bvh: Option<BVHNode>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            lights: Default::default(),
            camera: Default::default(),
            background: |_| DVec3::ZERO,
            bvh: None,
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(bvh) = &self.bvh {
            bvh.hit(ray, t_min, t_max)
        } else {
            self.objects.hit(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        if let Some(bvh) = &self.bvh {
            bvh.bounding_box(time_0, time_1)
        } else {
            self.objects.bounding_box(time_0, time_1)
        }
    }
}

impl Scene {
    pub fn build() -> SceneBuilder {
        SceneBuilder::default()
    }
}

#[derive(Default)]
pub struct SceneBuilder {
    scene: Scene,
    build_bvh: bool,
}

impl SceneBuilder {
    pub fn objects(mut self, objs: Vec<Arc<dyn Hittable>>) -> Self {
        self.scene.objects = objs;
        self
    }

    pub fn add_objects(mut self, objs: &[Arc<dyn Hittable>]) -> Self {
        for obj in objs {
            self.scene.objects.push(obj.clone());
        }
        self
    }

    pub fn background(mut self, background: fn(Ray) -> DVec3) -> Self {
        self.scene.background = background;
        self
    }

    pub fn lights(mut self, lights: Vec<Arc<dyn Hittable>>) -> Self {
        self.scene.lights = lights;
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.scene.camera = camera;
        self
    }

    pub fn build_bvh(mut self) -> Self {
        self.build_bvh = true;
        self
    }

    pub fn build(mut self) -> Scene {
        if self.build_bvh {
            let bvh = BVHNode::new(self.scene.objects.as_slice(), 0.0, 0.0);
            self.scene.bvh = Some(bvh);
        }

        self.scene
    }
}

use std::sync::Arc;

use crate::{bounding_box::AABB, hit::HitRecord, hittable::Hittable, material::Material, ray::Ray};

use glam::{DVec2, DVec3};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub normals: Vec<DVec3>,
    pub uv: Vec<DVec2>,
    pub material: Arc<dyn Material>,
}

pub fn create_mesh(
    vertices: Vec<DVec3>,
    normals: Vec<DVec3>,
    uv: Vec<DVec2>,
    indices: Vec<[u32; 3]>,
    material: Arc<dyn Material>,
) -> Vec<Arc<Triangle>> {
    let mesh = Arc::new(Mesh {
        vertices,
        normals,
        uv,
        material,
    });

    let mut triangles = Vec::new();

    for indices in indices {
        triangles.push(Arc::new(Triangle {
            indices: indices,
            data: mesh.clone(),
        }));
    }

    triangles
}

#[derive(Debug)]
pub struct Triangle {
    pub indices: [u32; 3],
    pub data: Arc<Mesh>,
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, _: f64, _: f64) -> Option<HitRecord> {
        let (v0, v1, v2) = (
            self.data.vertices[self.indices[0] as usize],
            self.data.vertices[self.indices[1] as usize],
            self.data.vertices[self.indices[2] as usize],
        );

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.dir.cross(edge2);
        let det = edge1.dot(h);
        if det > -f64::EPSILON && det < f64::EPSILON {
            return None;
        }

        let det_inv = 1.0 / det;
        let s = ray.origin - v0;

        let u = det_inv * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = det_inv * ray.dir.dot(q);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = det_inv * edge2.dot(q);
        if t <= f64::EPSILON {
            return None;
        }

        let (n0, n1, n2) = (
            self.data.normals[self.indices[0] as usize],
            self.data.normals[self.indices[1] as usize],
            self.data.normals[self.indices[2] as usize],
        );

        let (uv0, uv1, uv2) = (
            self.data.uv[self.indices[0] as usize],
            self.data.uv[self.indices[1] as usize],
            self.data.uv[self.indices[2] as usize],
        );

        let n = (u * n0) + (v * n1) + ((1.0 - (u + v)) * n2);
        let uv = (u * uv0) + (v * uv1) + ((1.0 - (u + v)) * uv2);

        Some(HitRecord {
            point: ray.at(t),
            material: &self.data.material,
            normal: n,
            u: uv.x,
            v: uv.y,
            t,
            front_face: ray.dir.dot(n) > 0.0,
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let (min, max) = self
            .indices
            .iter()
            .map(|index| self.data.vertices[*index as usize])
            .fold(
                (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN)),
                |(min, max), vertex| (DVec3::min(min, vertex), DVec3::max(max, vertex)),
            );

        Some(AABB { min, max })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lambertian, Ray};

    #[test]
    fn bbox() {
        let meshdata = Arc::new(Mesh {
            vertices: vec![
                DVec3::new(-1.0, -1.0, -1.0),
                DVec3::new(-1.0, 1.0, -1.0),
                DVec3::new(1.0, 1.1, -1.0),
            ],
            uv: Vec::new(),
            normals: Vec::new(),
            material: Arc::new(Lambertian::new(DVec3::splat(0.0))),
        });

        let triangle = Triangle {
            indices: [0, 1, 2],
            data: meshdata,
        };

        let bbox = triangle.bounding_box(0.0, 0.0).unwrap();
        assert_eq!(
            bbox,
            AABB {
                min: DVec3::new(-1.0, -1.0, -1.0),
                max: DVec3::new(1.0, 1.1, -1.0)
            }
        )
    }

    #[test]
    fn hit() {
        let meshdata = Arc::new(Mesh {
            vertices: vec![
                DVec3::new(1.0, 0.0, -1.0),
                DVec3::new(0.0, 1.0, -1.0),
                DVec3::new(0.0, 0.0, -1.0),
            ],
            uv: vec![
                DVec2::new(1.0, 0.0),
                DVec2::new(0.0, 1.0),
                DVec2::new(0.0, 0.0),
            ],
            normals: vec![
                DVec3::new(0.0, 0.0, 1.0),
                DVec3::new(0.0, 0.0, 1.0),
                DVec3::new(0.0, 0.0, 1.0),
            ],
            material: Arc::new(Lambertian::new(DVec3::splat(0.0))),
        });

        let triangle = Triangle {
            indices: [0, 1, 2],
            data: meshdata,
        };

        let ray = Ray {
            origin: DVec3::ZERO,
            dir: DVec3::new(0.5, 0.5, -1.0).normalize(),
            time: 0.0,
        };

        let hr = triangle.hit(&ray, 0.01, 10.0).unwrap();
        println!("{:#?}", hr);

        println!("{:#?}", ray.at(hr.t));
    }
}

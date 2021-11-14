use glam::{DVec2, DVec3};
use obj::*;
use renderer::{create_mesh, Material, Triangle};
use std::sync::Arc;

pub fn load_obj(file_name: &str, material: Arc<dyn Material>) -> Option<Vec<Vec<Triangle>>> {
    let obj = Obj::load(file_name).ok()?;

    let mut meshes = Vec::new();

    for object in obj.data.objects {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let mut index = 0;

        for group in object.groups {
            for poly in group.polys {
                assert!(poly.0.len() == 3);

                for index_tuple in poly.0 {
                    let pos = obj.data.position[index_tuple.0];
                    vertices.push(DVec3::new(pos[0] as f64, pos[1] as f64, pos[2] as f64));

                    uvs.push(match index_tuple.1 {
                        Some(uv_index) => {
                            let uv = obj.data.texture[uv_index];
                            DVec2::new(uv[0] as f64, uv[1] as f64)
                        }
                        None => DVec2::new(0.0, 0.0),
                    });

                    normals.push(match index_tuple.2 {
                        Some(normal_index) => {
                            let normal = obj.data.normal[normal_index];
                            DVec3::new(normal[0] as f64, normal[1] as f64, normal[2] as f64)
                        }
                        None => DVec3::X,
                    });
                }
                indices.push([index, index + 1, index + 2]);
                index = index + 3
            }
        }

        meshes.push(create_mesh(
            vertices,
            normals,
            uvs,
            indices,
            material.clone(),
        ));
    }

    Some(meshes)
}

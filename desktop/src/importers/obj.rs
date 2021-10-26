use glam::{DVec2, DVec3};
use renderer::{create_mesh, Material, Triangle};
use std::sync::Arc;
use tobj;

pub fn load_obj(file_name: &str, material: Arc<dyn Material>) -> Option<Vec<Vec<Triangle>>> {
    let load_options = tobj::LoadOptions {
        single_index: true,
        ..Default::default()
    };
    let (models, materials) = tobj::load_obj(file_name, &load_options).ok()?;

    let mut meshes = Vec::new();

    for model in models.iter() {
        let mesh = &model.mesh;

        if mesh.indices.len() % 3 != 0 {
            return None;
        }

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        for face_index in (0..mesh.indices.len()).step_by(3) {
            for vertex_index in [
                mesh.indices[face_index],
                mesh.indices[face_index + 1],
                mesh.indices[face_index + 2],
            ] {
                vertices.push(DVec3::new(
                    mesh.positions[(3 * vertex_index) as usize] as f64,
                    mesh.positions[(3 * vertex_index + 1) as usize] as f64,
                    mesh.positions[(3 * vertex_index + 2) as usize] as f64,
                ));

                normals.push(DVec3::new(
                    mesh.normals[(3 * vertex_index) as usize] as f64,
                    mesh.normals[(3 * vertex_index + 1) as usize] as f64,
                    mesh.normals[(3 * vertex_index + 2) as usize] as f64,
                ));

                uvs.push(DVec2::new(
                    mesh.texcoords[(2 * vertex_index) as usize] as f64,
                    mesh.texcoords[(2 * vertex_index + 1) as usize] as f64,
                ));
            }
            indices.push([
                mesh.indices[face_index],
                mesh.indices[face_index + 1],
                mesh.indices[face_index + 2],
            ]);
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

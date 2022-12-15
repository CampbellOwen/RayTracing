use exporters::ppm::write_image;
use glam::DVec2;
use rand::Rng;

use renderer::create_mesh;
use renderer::Transformable;
use renderer::Transformed;
use renderer::{
    rand_in_range, random, ray_colour, AARect, BVHNode, Camera, CheckerTexture, Dielectric,
    DiffuseLight, Hittable, Image, Lambertian, Material, Mesh, Metal, MovingSphere, Ray,
    SolidColour, Sphere, Triangle,
};

use glam::{DMat4, DVec3};

use rayon::prelude::*;

use core::num;
use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use importers::obj::load_obj;
mod importers;

mod exporters;

type SceneDescription = (Vec<Arc<dyn Hittable>>, Camera, fn(Ray) -> DVec3);

fn skybox(ray: Ray) -> DVec3 {
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * unit_dir.y + 1.0;

    let white = DVec3::new(1.0, 1.0, 1.0);
    let blue = DVec3::new(0.5, 0.7, 1.0);
    white * (1.0 - t) + blue * t
}

fn no_light(_: Ray) -> DVec3 {
    DVec3::new(0.0, 0.0, 0.0)
}

#[allow(dead_code)]
fn create_cube_scene() -> SceneDescription {
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    let cube_mat = Arc::new(Lambertian::new(DVec3::splat(0.5)));
    // let cube_mat: Arc<dyn Material> = Arc::new(Lambertian {
    //     albedo: Arc::new(load_texture("textures/earthmap.jpg").unwrap()),
    // });

    let mut world: Vec<Arc<dyn Hittable>> = vec![Arc::new(Sphere {
        center: DVec3::new(-0.0, -100.5, -1.0),
        radius: 100.0,
        material: ground_material.clone(),
    })];

    let aspect_ratio = 16.0 / 9.0;
    let look_from = DVec3::new(1.0, 3.0, 4.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (look_at - look_from).length();
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        30.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, skybox);
}
fn create_cornell_scene() -> SceneDescription {
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    let mut world: Vec<Arc<dyn Hittable>> = vec![
        //Arc::new(Transformed::new(
        //    DMat4::from_scale(DVec3::new(1.5, 0.01, 1.5)),
        //    Arc::new(Sphere {
        //        center: DVec3::ZERO,
        //        radius: 1.0,
        //        material: sphere_material.clone(),
        //    }),
        //)),
        Arc::new(Sphere {
            center: DVec3::new(-0.0, -101.0, -1.0),
            radius: 100.0,
            material: ground_material.clone(),
        }),
    ];

    let grey_material = Arc::new(Lambertian::new(DVec3::splat(0.1)));
    let grey_cube = Arc::new(BVHNode::from_mesh(
        load_obj("F:\\Models\\cube.obj", grey_material.clone())
            .unwrap()
            .pop()
            .unwrap(),
        0.0,
        0.0,
    ));

    let lit_material = Arc::new(DiffuseLight {
        emit_colour: Arc::new(SolidColour {
            colour: DVec3::splat(1.0),
        }),
    });

    let lit_cube = Arc::new(BVHNode::from_mesh(
        load_obj("F:\\Models\\cube.obj", lit_material.clone())
            .unwrap()
            .pop()
            .unwrap(),
        0.0,
        0.0,
    ));

    let red_material = Arc::new(Lambertian::new(DVec3::new(1.0, 0.0, 0.0)));
    let red_cube = Arc::new(BVHNode::from_mesh(
        load_obj("F:\\Models\\cube.obj", red_material.clone())
            .unwrap()
            .pop()
            .unwrap(),
        0.0,
        0.0,
    ));

    let green_material = Arc::new(Lambertian::new(DVec3::new(0.0, 1.0, 0.0)));
    let green_cube = Arc::new(BVHNode::from_mesh(
        load_obj("F:\\Models\\cube.obj", green_material.clone())
            .unwrap()
            .pop()
            .unwrap(),
        0.0,
        0.0,
    ));

    let back_wall_transform = DMat4::from_translation(DVec3::new(0.0, 0.0, -1.5))
        * DMat4::from_scale(DVec3::new(1.5, 1.5, 0.01));
    let back_wall = Transformed::new(back_wall_transform, lit_cube.clone());
    world.push(Arc::new(back_wall));

    let right_wall_transform = DMat4::from_translation(DVec3::new(1.5, 0.0, 0.0))
        * DMat4::from_scale(DVec3::new(0.001, 1.5, 1.5));
    let right_wall = Transformed::new(right_wall_transform, red_cube.clone());
    world.push(Arc::new(right_wall));

    let left_wall_transform = DMat4::from_translation(DVec3::new(-1.5, 0.0, 0.0))
        * DMat4::from_scale(DVec3::new(0.001, 1.5, 1.5));
    let left_wall = Transformed::new(left_wall_transform, green_cube.clone());
    world.push(Arc::new(left_wall));

    let floor_transform = DMat4::from_translation(DVec3::new(0.0, -1.0, 0.0))
        * DMat4::from_scale(DVec3::new(1.5, 0.1, 2.0));
    let floor = Transformed::new(floor_transform, grey_cube.clone());
    world.push(Arc::new(floor));

    let ceiling_transform = DMat4::from_translation(DVec3::new(0.0, 1.0, 0.0))
        * DMat4::from_scale(DVec3::new(1.5, 0.1, 2.0));
    let ceiling = Transformed::new(ceiling_transform, grey_cube.clone());
    world.push(Arc::new(ceiling));

    let left_box_transform = DMat4::from_translation(DVec3::new(0.0, 0.0, 0.0));
    let left_box = Transformed::new(left_box_transform, grey_cube.clone());
    world.push(Arc::new(left_box));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = DVec3::new(0.0, 0.0, 5.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (look_at - look_from).length();
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        30.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, no_light);
}
fn create_sphere_scene() -> SceneDescription {
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    let sphere_material = Arc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.2));

    let mut world: Vec<Arc<dyn Hittable>> = vec![
        //Arc::new(Transformed::new(
        //    DMat4::from_scale(DVec3::new(1.5, 0.01, 1.5)),
        //    Arc::new(Sphere {
        //        center: DVec3::ZERO,
        //        radius: 1.0,
        //        material: sphere_material.clone(),
        //    }),
        //)),
        Arc::new(Sphere {
            center: DVec3::new(-0.0, -101.0, -1.0),
            radius: 100.0,
            material: ground_material.clone(),
        }),
    ];

    let mut test_mesh = load_obj(
        "F:\\Models\\cube.obj",
        //"F:\\Models\\JapaneseTemple\\model_triangulated.obj",
        sphere_material.clone(),
    )
    .unwrap();

    let cube = test_mesh.pop().unwrap();

    let bvh_cube = BVHNode::from_mesh(cube, 0.0, 1.0);

    let transformation = DMat4::from_scale(DVec3::new(2.0, 1.0, 1.0))
        * DMat4::from_rotation_y(0.785398)
        * DMat4::from_translation(DVec3::new(1.0, 0.0, 0.0));

    let transformed_cube = Transformed::new(transformation, Arc::new(bvh_cube));

    world.push(Arc::new(transformed_cube));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = DVec3::new(0.0, 0.0, 5.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (look_at - look_from).length();
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        30.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, skybox);
}

#[allow(dead_code)]
fn create_simple_scene() -> SceneDescription {
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    let center_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(load_texture("textures/earthmap.jpg").unwrap()),
    });

    let shiny_metal_material = Arc::new(Metal::new(DVec3::new(0.1, 0.1, 0.1), 0.0));

    let left_material = Arc::new(Dielectric { ior: 1.5 });

    let right_material: Arc<dyn Material> = Arc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 1.0));

    let world: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Transformed::new(
            DMat4::from_scale(DVec3::splat(1.4)),
            Arc::new(Sphere {
                center: DVec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: center_material.clone(),
            }),
        )),
        Arc::new(Sphere {
            center: DVec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: left_material.clone(),
        }),
        Arc::new(Sphere {
            center: DVec3::new(-1.0, 0.0, -1.0),
            radius: -0.49,
            material: left_material.clone(),
        }),
        Arc::new(Sphere {
            center: DVec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: right_material.clone(),
        }),
        Arc::new(
            Sphere {
                center: DVec3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                material: shiny_metal_material.clone(),
            }
            .transform(
                &(DMat4::from_scale(DVec3::new(0.5, 0.5, 0.5))
                    * DMat4::from_translation(DVec3::new(0.0, 2.0, 0.0))),
            ),
        ),
        Arc::new(Sphere {
            center: DVec3::new(-0.0, -100.5, -1.0),
            radius: 100.0,
            material: ground_material.clone(),
        }),
    ];

    let aspect_ratio = 16.0 / 9.0;
    let look_from = DVec3::new(-5.0, 0.8, -3.5);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (look_at - look_from).length();
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        30.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, skybox);
}

#[allow(dead_code)]
fn create_random_scene() -> SceneDescription {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    let ground_material = Arc::new(Lambertian::new(DVec3::new(0.5, 0.5, 0.5)));

    world.push(Arc::new(Sphere {
        center: DVec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    //let mut rng = Pcg64::seed_from_u64(2);
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = rng.gen::<f64>();
            let center = DVec3::new(
                (a as f64) + (0.9 * rng.gen::<f64>()),
                0.2,
                (b as f64) + (0.9 * rng.gen::<f64>()),
            );

            if (center - DVec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if material_choice < 0.8 {
                    // Diffuse material
                    let albedo = random() * random();
                    let material = Arc::new(Lambertian::new(albedo));

                    let center_2 = center + DVec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);

                    world.push(Arc::new(MovingSphere {
                        center_0: center,
                        center_1: center_2,
                        time_0: 0.0,
                        time_1: 1.0,
                        radius: 0.2,
                        material,
                    }));
                } else if material_choice < 0.95 {
                    // Metal
                    let albedo = rand_in_range(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }));
                } else {
                    // Glass
                    let material = Arc::new(Dielectric { ior: 1.5 });
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    }));
                }
            }
        }
    }

    let material = Arc::new(Dielectric { ior: 1.5 });
    world.push(Arc::new(Sphere {
        center: DVec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let material = Arc::new(Lambertian::new(DVec3::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere {
        center: DVec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let material = Arc::new(Metal::new(DVec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(Sphere {
        center: DVec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = DVec3::new(13.0, 2.0, 3.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, skybox);
}

#[allow(dead_code)]
fn two_spheres() -> SceneDescription {
    let material = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    let look_from = DVec3::new(13.0, 2.0, 3.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = 16.0 / 9.0;
    let aperture = 0.1;
    let focus_dist = (look_from - look_at).length();
    return (
        vec![
            Arc::new(Sphere {
                center: DVec3::new(0.0, -10.0, 0.0),
                radius: 10.0,
                material: material.clone(),
            }),
            Arc::new(Sphere {
                center: DVec3::new(0.0, 10.0, 0.0),
                radius: 10.0,
                material: material.clone(),
            }),
            Arc::new(Sphere {
                center: DVec3::new(10.0, 2.0, 0.5),
                radius: 1.0,
                material: Arc::new(DiffuseLight {
                    emit_colour: Arc::new(SolidColour {
                        colour: DVec3::new(4.0, 4.0, 4.0),
                    }),
                }),
            }),
        ],
        Camera::new_instant(
            look_from,
            look_at,
            up,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        ),
        no_light,
    );
}

fn simple_triangle_scene() -> SceneDescription {
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    let triangle_mat = Arc::new(Lambertian::new(DVec3::splat(0.5)));

    let mut world: Vec<Arc<dyn Hittable>> = vec![Arc::new(Sphere {
        center: DVec3::new(-0.0, -100.5, -1.0),
        radius: 100.0,
        material: ground_material.clone(),
    })];

    create_mesh(
        vec![
            DVec3::new(-0.5, -0.5, 0.5),  // Front bottom left
            DVec3::new(0.5, 0.5, 0.5),    // Front top right
            DVec3::new(-0.5, 0.5, 0.5),   // Front top left
            DVec3::new(0.5, -0.5, 0.5),   // Front bottom right
            DVec3::new(0.5, -0.5, -0.5),  // Back bottom right
            DVec3::new(0.5, 0.5, -0.5),   // Back top right
            DVec3::new(-0.5, 0.5, -0.5),  // Back top left
            DVec3::new(-0.5, -0.5, -0.5), // Back bottom left
        ],
        vec![
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(0.0, -1.0, 0.0),
        ],
        vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(1.0, 1.0),
            DVec2::new(0.0, 1.0),
            DVec2::new(1.0, 0.0),
            DVec2::new(1.0, 0.0),
            DVec2::new(1.0, 1.0),
            DVec2::new(1.0, 0.0),
            DVec2::new(0.0, 0.0),
        ],
        vec![
            [0, 1, 2],
            [0, 3, 1],
            [3, 4, 1],
            [4, 5, 1],
            [2, 1, 6],
            [1, 5, 6],
            [7, 0, 2],
            [7, 2, 6],
            [6, 5, 7],
            [7, 5, 4],
        ],
        triangle_mat,
    )
    .into_iter()
    .map(|triangle| Arc::new(triangle) as Arc<dyn Hittable>)
    .for_each(|triangle| world.push(triangle));

    let aspect_ratio = 16.0 / 9.0;
    let look_from = DVec3::new(0.0, 0.5, -2.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (look_at - look_from).length();
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        90.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    return (world, camera, skybox);
}

#[allow(dead_code)]
fn simple_light() -> SceneDescription {
    let world: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere {
            center: DVec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Arc::new(Lambertian::new(DVec3::new(0.9, 0.9, 0.9))),
        }),
        Arc::new(Sphere {
            center: DVec3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Arc::new(Lambertian {
                albedo: Arc::new(load_texture("textures/earthmap.jpg").unwrap()),
            }),
        }),
        Arc::new(Sphere {
            center: DVec3::new(0.0, 6.5, 0.0),
            radius: 2.0,
            material: Arc::new(DiffuseLight {
                emit_colour: Arc::new(SolidColour {
                    colour: DVec3::new(4.0, 4.0, 4.0),
                }),
            }),
        }),
        Arc::new(AARect {
            x_range: (3.0, 5.0),
            y_range: (1.0, 3.0),
            z: -2.0,
            material: Arc::new(DiffuseLight {
                emit_colour: Arc::new(SolidColour {
                    colour: DVec3::new(4.0, 4.0, 4.0),
                }),
            }),
        }),
    ];

    let look_from = DVec3::new(26.0, 3.0, 6.0);
    let look_at = DVec3::new(0.0, 2.0, 0.0);

    let camera = Camera::new_instant(
        look_from,
        look_at,
        DVec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        (look_at - look_from).length(),
    );

    return (world, camera, no_light);
}

fn mesh_scene() -> SceneDescription {
    let triangle_mat = Arc::new(Lambertian {
        albedo: Arc::new(load_texture("F:\\Models\\JapaneseTemple\\Textures\\albedo.png").unwrap()),
    });

    //let triangle_mat = Arc::new(Lambertian::new(DVec3::splat(0.5)));

    //let test_mesh = load_obj(
    //    //"F:\\Models\\JapaneseTemple\\model_triangulated.obj",
    //    "F:\\Models\\cube.obj",
    //    triangle_mat,
    //)
    //.unwrap();

    let test_mesh = load_obj(
        //"F:\\Models\\cube.obj",
        "F:\\Models\\JapaneseTemple\\model_triangulated.obj",
        triangle_mat,
    )
    .unwrap();

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    test_mesh.into_iter().for_each(|mesh| {
        mesh.into_iter()
            .map(|triangle| {
                Arc::new(Transformed::new(
                    DMat4::from_translation(DVec3::splat(5.0)),
                    Arc::new(triangle),
                )) as Arc<dyn Hittable>
            })
            .for_each(|triangle| world.push(triangle))
    });

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(CheckerTexture::new(
            DVec3::new(0.2, 0.3, 0.1),
            DVec3::new(0.9, 0.9, 0.9),
        )),
    });

    world.push(Arc::new(Sphere {
        center: DVec3::new(40.0, 40.0, 20.0),
        radius: 25.0,
        material: Arc::new(DiffuseLight {
            emit_colour: Arc::new(SolidColour {
                colour: DVec3::new(4.0, 4.0, 4.0),
            }),
        }),
    }));

    //world.push(Arc::new(Sphere {
    //    center: DVec3::new(-40.0, 40.0, -20.0),
    //    radius: 25.0,
    //    material: Arc::new(DiffuseLight {
    //        emit_colour: Arc::new(SolidColour {
    //            colour: DVec3::new(4.0, 4.0, 4.0),
    //        }),
    //    }),
    //}));

    //world.push(Arc::new(Sphere {
    //    center: DVec3::new(0.0, -1000.0, 0.0),
    //    radius: 1000.0,
    //    material: ground_material,
    //}));

    let look_from = DVec3::new(-2.0, 30.0, 30.0);
    let look_at = DVec3::new(0.0, 15.0, 0.0);
    //let look_from = DVec3::new(-2.0, 2.0, 2.0);
    //let look_at = DVec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new_instant(
        look_from,
        look_at,
        DVec3::new(0.0, 1.0, 0.0),
        80.0,
        16.0 / 9.0,
        0.1,
        (look_at - look_from).length(),
    );

    return (world, camera, no_light);
}

fn load_texture(filename: &str) -> Option<Image> {
    let tex = image::open(filename).ok()?.into_rgb8();
    let mut tex_image = Image::new(tex.dimensions());
    for y in 0..tex.dimensions().1 {
        for x in 0..tex.dimensions().0 {
            let pixel = tex.get_pixel(x, y);
            tex_image.put(
                x,
                y,
                &DVec3::new(
                    (pixel[0] as f64 / 255.0).powi(2), // Gamma correction approximation
                    (pixel[1] as f64 / 255.0).powi(2),
                    (pixel[2] as f64 / 255.0).powi(2),
                ),
            );
        }
    }

    Some(tex_image)
}

// https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
fn aces_tonemapping(pixel: DVec3) -> DVec3 {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;

    return ((pixel * (a * pixel + b)) / (pixel * (c * pixel + d) + e))
        .clamp(DVec3::ZERO, DVec3::ONE);
}

fn main() {
    let width = 100;
    let aspect_ratio = 16.0 / 9.0;
    let height = (width as f64 / aspect_ratio) as u32;

    //let (world, camera, background_colour) = simple_triangle_scene();
    let (world, camera, background_colour) = mesh_scene();
    //let (world, camera, background_colour) = create_random_scene();
    //let (world, camera, background_colour) = create_simple_scene();
    //let (world, camera, background_colour) = create_sphere_scene();
    //let (world, camera, background_colour) = create_cornell_scene();
    //let (world, camera, background_colour) = two_spheres();
    //let (world, camera, background_colour) = simple_light();

    println!("Building BVH for scene with {} triangles", world.len());
    let bvh = BVHNode::new(world.as_slice(), 0.0, 0.0);
    println!("Done!");

    let samples_per_pixel = 10;
    let max_depth = 10;
    println!(
        "Rendering scene with {} samples per pixel, {} max bounces, at a resolution of {}x{}",
        samples_per_pixel, max_depth, width, height
    );

    let tile_size = 16;
    let num_tiles = (
        (width + tile_size - 1) / (tile_size),
        (height + tile_size - 1) / (tile_size),
    );

    println!(
        "Breaking image into {}x{} tiles of size {}",
        num_tiles.0, num_tiles.1, tile_size
    );

    let img = Arc::new(Mutex::new(Image::new((width, height))));

    let img_clone = img.clone();

    ctrlc::set_handler(move || {
        write_image(&img_clone.lock().unwrap(), "output.ppm").expect("Writing image failed");
        exit(0);
    })
    .expect("Setting handler failed");

    let render_start_time = Instant::now();

    (0..(num_tiles.0 * num_tiles.1))
        .into_par_iter()
        //.into_iter()
        .for_each(|tile_index| {
            let tile_xy = (tile_index % num_tiles.0, tile_index / num_tiles.0);
            let tile_origin = (tile_xy.0 * tile_size, tile_xy.1 * tile_size);

            let mut tile = {
                let locked_img = img.lock().unwrap();
                locked_img.get_tile(tile_origin, (tile_size, tile_size))
            };

            let mut rng = rand::thread_rng();
            for index in 0..(tile.size.0 * tile.size.1) {
                let (x, y) = tile.get_xy(index);

                let mut colour = DVec3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (x as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / (height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    colour = colour
                        + ray_colour(
                            ray,
                            &background_colour,
                            //&world.as_slice(),
                            &bvh,
                            max_depth,
                        );
                }

                colour = colour / (samples_per_pixel as f64);

                colour = aces_tonemapping(colour);

                tile.pixels[index as usize] = colour;
            }

            let mut locked_img = img.lock().unwrap();
            locked_img.merge_tile(tile);
            //println!("Finished rendering tile {}", tile_index);
        });

    let render_end_time = Instant::now();
    let render_time = render_end_time - render_start_time;

    println!("Rendering complete in {:?}", render_time);

    println!("\nSaving image");
    let locked_image = img.lock().unwrap();
    write_image(&locked_image, "output.ppm").expect("Writing image failed");
}

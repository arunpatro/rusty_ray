use crate::{composites, datastructures::AABBNode, image_utils, primitives, utils};
use kdam::tqdm;
use nalgebra::Vector3;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::Write; // added this line

fn write_to_file(repr: &str) -> Result<()> {
    let mut file = File::create("repr.txt")?;
    file.write_all(repr.as_bytes())?;
    Ok(())
}

fn inorder_repr(node: &AABBNode) -> String {
    let mut result = String::new();

    if let Some(left) = node.left.as_ref() {
        result.push_str(&inorder_repr(left)); // left child in binary heap is at 2*parent_idx + 1
    }

    if let Some(triangle_idx) = node.object_idx {
        result.push_str(&format!("N-{} T-{} ", node.id, triangle_idx));
    }

    if let Some(right) = node.right.as_ref() {
        result.push_str(&inorder_repr(right)); // right child in binary heap is at 2*parent_idx + 2
    }

    result
}

pub fn repr_test() {
    let mesh = composites::Mesh::from_off_file("data/bunny.off");
    // let mesh =composites::Mesh::from_off_file("data/dragon.off");
    let repr = inorder_repr(&mesh.bvh.root);
    println!("{}", repr);
    // write repr to file without utils
    write_to_file(&repr).unwrap();
}

pub fn bvh_box_repr() {
    let mesh = composites::Mesh::from_off_file("data/bunny.off");
    // let mesh =composites::Mesh::from_off_file("data/dragon.off");

    // bfs iterate over the bvh and print the bounding boxes
    let mut queue = vec![&mesh.bvh.root];
    while !queue.is_empty() {
        let node = queue.remove(0);
        println!(
            "T-{} N-{} [{:.6} {:.6} {:.6}] [{:.6} {:.6} {:.6}]",
            match node.object_idx {
                Some(idx) => idx.to_string(),
                None => "-1".to_string(),
            },
            node.id,
            node.bbox.min.x,
            node.bbox.min.y,
            node.bbox.min.z,
            node.bbox.max.x,
            node.bbox.max.y,
            node.bbox.max.z,
        );
        if let Some(left) = node.left.as_ref() {
            queue.push(left);
        }
        if let Some(right) = node.right.as_ref() {
            queue.push(right);
        }
    }
}

pub fn render_scene() {
    // set the objects
    let objects: Vec<Box<dyn primitives::Object>> =
        vec![Box::new(composites::Mesh::from_off_file("data/bunny.off"))];
    // vec![Box::new(composites::Mesh::from_off_file("data/dragon.off"))];

    // set the materials
    let material = primitives::Material::new(
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(0.2, 0.2, 0.2),
        256.,
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.7, 0.7, 0.7),
    );

    // set the lights
    let lights: Vec<primitives::Light> = vec![
        primitives::Light::new(Vector3::new(8., 8., 0.), Vector3::new(16., 16., 16.)),
        primitives::Light::new(Vector3::new(6., -8., 0.), Vector3::new(16., 16., 16.)),
        primitives::Light::new(Vector3::new(4., 8., 0.), Vector3::new(16., 16., 16.)),
        primitives::Light::new(Vector3::new(2., -8., 0.), Vector3::new(16., 16., 16.)),
        primitives::Light::new(Vector3::new(0., 8., 0.), Vector3::new(16., 16., 16.)),
        primitives::Light::new(Vector3::new(-2., -8., 0.), Vector3::new(16., 16., 16.)),
        primitives::Light::new(Vector3::new(-4., 8., 0.), Vector3::new(16., 16., 16.)),
    ];

    // set the scene
    let ambient_light = Vector3::new(0.2, 0.2, 0.2);
    let ambient_color = Vector3::new(0.0, 0.5, 0.0);
    // let ambient_color = Vector3::new(0.5, 0.1, 0.1);
    let ambient_color = ambient_color.component_mul(&ambient_light);
    let scene = primitives::Scene::new(objects, lights, ambient_color);

    // set the camera
    let mut camera = primitives::Camera::new(
        0.3491,
        5.,
        320,
        240,
        Vector3::new(0., 0., 2.),
        primitives::CameraKind::PERSPECTIVE,
    );

    // render
    for i in tqdm!(125..175) {
        // for i in 0..camera.width {
        for j in 0..camera.height {
            let ray = camera.ray(i, j);
            // if i == 125 && j == 120 {
            //     println!("{:?}", ray);
            // } else {
            //     continue;
            // }

            // if 125 < i && i < 175  {
            //     // println!("{:?}", ray);
            // } else {
            //     continue;
            // }
            let color = utils::shoot_ray(&ray, &scene, &material, 5);
            camera.image[(i, j)] = color;
        }
    }

    image_utils::save_as_png(camera, "bvh.png");
}

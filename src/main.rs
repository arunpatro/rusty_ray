use nalgebra::Vector3;
use rusty_ray::{composites, image_utils, primitives, utils};
use kdam::tqdm;

fn render_scene() {
    // set the objects
    let objects: Vec<Box<dyn primitives::Object>> =
        vec![Box::new(composites::Mesh::from_off_file("data/bunny.off"))];
        // vec![Box::new(composites::Mesh::from_off_file("data/dragon.off"))];

    // set the materials
    let material = primitives::Material::new(
        Vector3::new(0.5, 0.1, 0.1),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(0.2, 0.2, 0.2),
        256.,
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.7, 0.7, 0.7),
    );

    // set the lights
    let ambient_light = Vector3::new(0.4, 0.4, 0.4);
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
    let background_color = Vector3::new(0., 0.1, 0.7);
    let scene = primitives::Scene::new(objects, lights, ambient_light, background_color);

    // set the camera
    let mut camera = primitives::Camera::new(
        0.3491,
        2.,
        320,
        240,
        Vector3::new(0., 0., 2.),
        primitives::CameraKind::PERSPECTIVE,
    );

    // render
    for i in tqdm!(0..camera.width) {
        for j in 0..camera.height {
            let ray = camera.ray(i, j);
            let color = utils::shoot_ray(&ray, &scene, &material, 5);
            camera.image[(i, j)] = color;
        }
    }

    image_utils::save_as_png(camera, "mesh.png");
}

fn main() {
    println!("Welcome to rusty ray tracer!");
    render_scene();
}

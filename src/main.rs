use kdam::tqdm;
use nalgebra::Vector3;
use rusty_ray::{
    composites, image_utils, primitives,
    raster::{self, Program, Uniform},
    utils,
};

fn raytracing_task() {
    // set the objects
    let objects: Vec<Box<dyn primitives::Object>> = vec![
        Box::new(primitives::Sphere::new(Vector3::new(10., 0., 1.), 1.)),
        Box::new(primitives::Sphere::new(Vector3::new(7., 0.05, -1.), 1.)),
        Box::new(primitives::Sphere::new(Vector3::new(4., 0.1, 1.), 1.)),
        Box::new(primitives::Sphere::new(Vector3::new(1., 0.2, -1.), 1.)),
        Box::new(primitives::Sphere::new(Vector3::new(-2., 0.4, 1.), 1.)),
        Box::new(primitives::Sphere::new(Vector3::new(-5., 0.8, -1.), 1.)),
        Box::new(primitives::Sphere::new(Vector3::new(-8., 1.6, 1.), 1.)),
        Box::new(primitives::Parallelogram::new(
            Vector3::new(-100., -1.25, -100.),
            Vector3::new(100., 0., -100.),
            Vector3::new(-100., -1.2, 100.),
        )),
    ];

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
    let ambient_color = Vector3::new(0.5, 0.1, 0.1);
    let ambient_color = ambient_color.component_mul(&ambient_light);
    let scene = primitives::Scene::new(objects, lights, ambient_color);

    // set the camera
    let mut camera = primitives::Camera::new(
        0.7854,
        10.,
        1200,
        800,
        Vector3::new(0., 1., 10.),
        primitives::CameraKind::PERSPECTIVE,
    );

    // render
    for i in 0..camera.width {
        for j in 0..camera.height {
            let ray = camera.ray(i, j);
            let color = utils::shoot_ray(&ray, &scene, &material, 5);
            camera.image[(i, j)] = color;
        }
    }

    image_utils::save_as_png(camera, "raytracing.png");
}

fn bvh_task() {
    // set the objects
    let objects: Vec<Box<dyn primitives::Object>> =
        // vec![Box::new(composites::Mesh::from_off_file("data/bunny.off"))];
    vec![Box::new(composites::Mesh::from_off_file("data/dragon.off"))];

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
    let ambient_color = Vector3::new(0.5, 0.1, 0.1);
    let ambient_color = ambient_color.component_mul(&ambient_light);
    let scene = primitives::Scene::new(objects, lights, ambient_color);

    // set the camera
    let mut camera = primitives::Camera::new(
        0.3491,
        5.,
        640,
        480,
        Vector3::new(0., 0., 2.),
        primitives::CameraKind::PERSPECTIVE,
    );

    // render
    for i in 0..camera.width {
        for j in 0..camera.height {
            let ray = camera.ray(i, j);
            let color = utils::shoot_ray(&ray, &scene, &material, 5);
            camera.image[(i, j)] = color;
        }
    }

    image_utils::save_as_png(camera, "bvh.png");
}

fn raster_task() {
    // let mesh = composites::Mesh::from_off_file("data/bunny.off");
    let mesh = composites::Mesh::from_off_file("data/dragon.off");

    let ambient_color = Vector3::new(0.2, 0.2, 0.2);
    let light = primitives::Light::new(Vector3::new(-1., 1., 3.), Vector3::new(0.2, 0.5, 0.1));

    // set the camera
    let mut camera = primitives::Camera::new(
        0.8,
        1.,
        500,
        500,
        Vector3::new(0., 0., 2.),
        primitives::CameraKind::ORTHOGRAPHIC,
        // primitives::CameraKind::PERSPECTIVE,
    );

    // global params for rasterization are unifrom and program
    let uniform = Uniform::new(
        camera.position,
        camera.focal_length,
        camera.fov,
        camera.width as f64 / camera.height as f64,
        ambient_color,
        light.position,
        light.color,
    );

    let program = Program::new(
        raster::vertex_shader,
        raster::fragment_shader,
        raster::blending_shader,
    );

    // render via rasterization
    raster::rasterize(&mesh, uniform, program, &mut camera.image);

    image_utils::save_as_png(camera, "raster.png");
}

fn main() {
    raytracing_task();
    // bvh_task();
    // raster_task();
}

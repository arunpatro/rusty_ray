use kdam::tqdm;
use nalgebra::Vector3;
use rusty_ray::{composites, image_utils, primitives, raster, utils};

fn render_scene() {
    // objects are now only meshes because they need to have the triangles property which other
    // let objects: Vec<composites::Mesh> = vec![composites::Mesh::from_off_file("data/bunny.off")];
    // let mesh = composites::Mesh::from_off_file("data/test.off");
    let mesh = composites::Mesh::from_off_file("data/bunny.off");

    // let ambient_light = Vector3::new(0.4, 0.4, 0.4);
    let lights: Vec<primitives::Light> = vec![primitives::Light::new(
        Vector3::new(-1., 1., 0.),
        Vector3::new(0.2, 0.5, 0.1),
    )];

    // set the camera
    let mut camera = primitives::Camera::new(
        0.3491,
        5.,
        500,
        500,
        Vector3::new(0., 0., 2.),
        primitives::CameraKind::PERSPECTIVE,
    );

    // render via rasterization
    raster::rasterize(&mesh, &mut camera, &lights);

    image_utils::save_as_png(camera, "raster.png");
}

fn main() {
    println!("Welcome to rusty ray tracer!");
    render_scene();
}

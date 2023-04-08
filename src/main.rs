use kdam::tqdm;
use nalgebra::Vector3;
use rusty_ray::{composites, image_utils, primitives, utils, raster};

fn render_scene() {
    // objects are now only meshes because they need to have the triangles property which other
    // let objects: Vec<composites::Mesh> = vec![composites::Mesh::from_off_file("data/bunny.off")];
    let mesh = composites::Mesh::from_off_file("data/test.off");
    // let mesh = composites::Mesh::from_off_file("data/bunny.off");

    let ambient_light = Vector3::new(0.4, 0.4, 0.4);
    // let lights: Vec<primitives::Light> = vec![primitives::Light::new(Vector3::new(8., 8., 0.))];

    // set the camera
    let mut camera = primitives::Camera::new(
        0.3491,
        5.,
        640,
        480,
        Vector3::new(0., 0., 2.),
        primitives::CameraKind::PERSPECTIVE,
    );

    // render via rasterization
    let mut frame_buffer = image::ImageBuffer::new(camera.width as u32, camera.height as u32);
    raster::rasterize(&mesh, &camera, &mut frame_buffer);

    // save the image
    frame_buffer.save("output.png").unwrap();
}

fn main() {
    println!("Welcome to rusty ray tracer!");
    render_scene();
}

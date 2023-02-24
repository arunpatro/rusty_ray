mod primitives;
use nalgebra::Vector3;
mod image_utils;

// q: this gives an error image_utils can't be found "use crate::image_utils::save_as_png;" doesn't work
// q: why does this work? "use image_utils::save_as_png;" doesn't work
// a: because image_utils is in the same directory as main.rs


// fn save_as_png(camera: primitives::Camera, filename: &str) {
//     let image_size = camera.image.nrows();
//     // convert to u8 for all channels and imgbuf to save as png
//     let mut imgbuf = image::ImageBuffer::new(image_size as u32, image_size as u32);
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let i = x as usize;
//         let j = y as usize;
//         let color = (camera.image[(i, j)] * 255.) as u8;
//         let alpha = (camera.alpha[(i, j)] * 255.) as u8;
//         *pixel = image::Rgba([color, color, color, alpha]);
//     }

//     imgbuf.save(filename).unwrap();
// }


fn render_sphere_orthographic() {
    let sphere = primitives::Sphere::new(Vector3::new(0., 0., 0.), 0.5);
    let mut camera =
        primitives::Camera::new(Vector3::new(0., 0., 5.,), Vector3::new(-1., 1., 1.), 600);
    let light = primitives::Light::new(Vector3::new(-1., 1., 1.));

    // render
    for i in 0..camera.image.nrows() {
        for j in 0..camera.image.ncols() {
            let ray = camera.ray(i, j);
            assert!(ray.direction.z == -1.);
            let intersection = sphere.intersects(&ray);
            match intersection {
                Some(intersection) => {
                    // if intersects, calculate color
                    let normal = sphere.normal(&intersection);
                    let light_vector = (light.position - intersection).normalize();
                    let itensity = normal.dot(&light_vector);

                    camera.image[(i, j)] = itensity;
                    camera.alpha[(i, j)] = 1.;
                    // print!(" Intersected at ({}, {}, {})", intersection.x, intersection.y, intersection.z);
                }
                None => {
                    // print!(" Failed to intersect");
                }
            }
        }
    }

    image_utils::save_as_png(camera, "sphere_orthographic.png");
}

fn main() {
    println!("Hello, world!");
    render_sphere_orthographic();
}

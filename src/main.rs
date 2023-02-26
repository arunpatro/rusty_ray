mod primitives;
use nalgebra::{Vector3, Vector4};
use primitives::{Material, Object};
mod utils;
use utils::{find_closest_point, is_light_visible};
mod image_utils;

fn render_scene() {
    // set the objects
    let objects: Vec<primitives::Sphere> = vec![
        primitives::Sphere::new(Vector3::new(10., 0., 1.), 1.),
        primitives::Sphere::new(Vector3::new(7., 0.05, -1.), 1.),
        primitives::Sphere::new(Vector3::new(4., 0.1, 1.), 1.),
        primitives::Sphere::new(Vector3::new(1., 0.2, -1.), 1.),
        primitives::Sphere::new(Vector3::new(-2., 0.4, 1.), 1.),
        primitives::Sphere::new(Vector3::new(-5., 0.8, -1.), 1.),
        primitives::Sphere::new(Vector3::new(-8., 1.6, 1.), 1.),
    ];

    // set the materials
    let material = Material::new(
        Vector3::new(0.5, 0.1, 0.1),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(0.2, 0.2, 0.2),
        256.,
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.7, 0.7, 0.7),
    );

    // set the lights
    let ambient_light = Vector3::new(0.2, 0.2, 0.2);
    let lights: Vec<primitives::Light> = vec![
        primitives::Light::new(Vector3::new(8., 8., 0.)),
        primitives::Light::new(Vector3::new(6., -8., 0.)),
        primitives::Light::new(Vector3::new(4., 8., 0.)),
        primitives::Light::new(Vector3::new(2., -8., 0.)),
        primitives::Light::new(Vector3::new(0., 8., 0.)),
        primitives::Light::new(Vector3::new(-2., -8., 0.)),
        primitives::Light::new(Vector3::new(-4., 8., 0.)),
    ];

    // set the camera
    let mut camera = primitives::Camera::new(0.7854, 5., 800, 400);

    // render
    for i in 0..camera.width {
        for j in 0..camera.height {
            let ray = camera.ray(i, j);

            let ans = find_closest_point(&ray, &objects);
            match ans {
                Some((t, intersection, normal)) => {
                    let light_color = Vector3::new(1., 1., 1.) * 16.; // white light

                    let mut total_color = Vector3::new(0., 0., 0.);
                    for light in &lights {
                        let light_vector = (light.position - intersection).normalize();

                        // check if light visible
                        if is_light_visible(&light, &intersection, &objects) {
                            let bisector_direction = (light_vector - ray.direction).normalize();
                            let diffuse_coeff = normal.dot(&light_vector).max(0.);
                            let specular_coeff = normal.dot(&bisector_direction).max(0.).powf(256.);

                            let diffuse = diffuse_coeff * material.diffuse_color;
                            let specular = specular_coeff * material.specular_color;

                            let d_light_squared = (light.position - intersection).norm_squared();
                            total_color +=
                                light_color.component_mul(&(diffuse + specular)) / d_light_squared;
                        }
                    }

                    let ambient_color = material.ambient_color.component_mul(&ambient_light);
                    let color = ambient_color + total_color;
                    camera.image[(i, j)] = Vector4::new(color.x, color.y, color.z, 1.);
                }
                None => {}
            }
        }
    }

    image_utils::save_as_png(camera, "scene.png");
}

fn main() {
    println!("Welcome to rusty ray tracer!");

    render_scene();
}

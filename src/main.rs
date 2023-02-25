mod primitives;
use nalgebra::{Vector3, Vector4};
use primitives::{Material, Object};
mod utils;
use utils::find_closest_point;
mod image_utils;

fn render_scene() {
    // setup the scene
    let objects: Vec<primitives::Sphere> = vec![
        // primitives::Sphere::new(Vector3::new(10., 0., 1.), 1.),
        // primitives::Sphere::new(Vector3::new(7., 0.05, -1.), 1.),
        primitives::Sphere::new(Vector3::new(4., 0.1, 1.), 1.),
        primitives::Sphere::new(Vector3::new(1., 0.2, -1.), 1.),
        // primitives::Sphere::new(Vector3::new(-2., 0.4, 1.), 1.),
        // primitives::Sphere::new(Vector3::new(-5., 0.8, -1.), 1.),
        // primitives::Sphere::new(Vector3::new(-8., 1.6, 1.), 1.),
        
    ];

    let material = Material::new(
        Vector3::new(0.5, 0.1, 0.1),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(0.2, 0.2, 0.2),
        256.0,
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.7, 0.7, 0.7),
    );

    let ambient_light = Vector3::new(0.2, 0.2, 0.2);
    let lights: Vec<primitives::Light> = vec![
        primitives::Light::new(Vector3::new(8., 8., 0.)),
        // primitives::Light::new(Vector3::new(6., -8., 0.)),
        // primitives::Light::new(Vector3::new(4., 8., 0.)),
        primitives::Light::new(Vector3::new(2., -8., 0.)),
        // primitives::Light::new(Vector3::new(0., 8., 0.)),
        // primitives::Light::new(Vector3::new(-2., -8., 0.)),
        // primitives::Light::new(Vector3::new(-4., 8., 0.)),
    ];

    let mut camera = primitives::Camera::new(0.7854, Vector3::new(0., 0., 4.), 1600, 800);

    let ambient_color = material.ambient_color.component_mul(&ambient_light);

    // render
    for i in 0..camera.image.nrows() {
        for j in 0..camera.image.ncols() {
            let ray = camera.ray(i, j);

            let closest_point = find_closest_point(&ray, &objects);
            match closest_point.0 {
                Some(index) => {
                    let intersection = objects[index].intersects(&ray).unwrap();
                    let normal = objects[index].normal(&intersection);
                    let light_color = Vector3::new(1., 1., 1.) * 16.;
                    let mut total_light = Vector3::new(0., 0., 0.);

                    for light in &lights {
                        let light_vector = (light.position - intersection).normalize();

                        // check if the light is visible
                        let shadow_ray = primitives::Ray::new(intersection, light_vector);
                        let mut is_shadowed = false;
                        for object in &objects {
                            if object.intersects(&shadow_ray).is_some() {
                                is_shadowed = true;
                                break;
                            }
                        }
                        // if is_shadowed {
                        //     continue;
                        // } 

                        let bisector_direction = (light_vector - ray.direction).normalize();
                        let diffuse_contrib =
                            normal.dot(&light_vector).max(0.) * material.diffuse_color;
                        let specular_contrib = normal.dot(&bisector_direction).max(0.).powf(256.)
                            * material.specular_color;
                        
                        let distance_squared = (light.position - intersection).norm_squared();
                        let light_contrib = diffuse_contrib + specular_contrib;

                        total_light += light_contrib.component_mul(&light_color) / distance_squared;
                    }

                    let color = ambient_color + total_light;
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

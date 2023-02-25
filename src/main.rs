mod primitives;
use nalgebra::Vector3;
mod image_utils;

fn render_scene() {
    // setup the scene
    let objects: Vec<primitives::Sphere> = vec![
        primitives::Sphere::new(Vector3::new(10., 0., 1.), 1.),
        primitives::Sphere::new(Vector3::new(7., 0.05, -1.), 1.),
        primitives::Sphere::new(Vector3::new(4., 0.1, 1.), 1.),
        primitives::Sphere::new(Vector3::new(1., 0.2, -1.), 1.),
        primitives::Sphere::new(Vector3::new(-2., 0.4, 1.), 1.),
        primitives::Sphere::new(Vector3::new(-5., 0.8, -1.), 1.),
        primitives::Sphere::new(Vector3::new(-8., 1.6, 1.), 1.),
    ];

    // let sphere = primitives::Sphere::new(Vector3::new(0., 0., 0.), 0.5);
    let mut camera =
        primitives::Camera::new(Vector3::new(0., 0., 4.), Vector3::new(-15., 15., 1.), 600);
    let light = primitives::Light::new(Vector3::new(-1., 1., 1.));

    // render
    for i in 0..camera.image.nrows() {
        for j in 0..camera.image.ncols() {
            let ray = camera.ray(i, j);

            let mut closest_intersection_and_normal: Option<(Vector3<f32>, Vector3<f32>)> = None;

            for object in &objects {
                let intersection = object.intersects(&ray);
                match intersection {
                    Some(intersection) => match closest_intersection_and_normal {
                        Some(closest_interaction) => {
                            if (intersection - camera.position).norm_squared()
                                < (closest_interaction.0 - camera.position).norm_squared()
                            {
                                let normal = object.normal(&intersection);
                                closest_intersection_and_normal = Some((intersection, normal));
                            }
                        }
                        None => {
                            let normal = object.normal(&intersection);
                            closest_intersection_and_normal = Some((intersection, normal));
                        }
                    },
                    None => {}
                }
            }

            match closest_intersection_and_normal {
                Some(intersection) => {
                    let normal = intersection.1;
                    let light_vector = (light.position - intersection.0).normalize();
                    let bisector_direction = (light_vector - ray.direction).normalize();
                    let specular_intensity = normal.dot(&bisector_direction).max(0.).powf(256.);
                    let diffuse_itensity = normal.dot(&light_vector);
                    let color = 0.2 + 0.3 * diffuse_itensity + 0.3 * specular_intensity;

                    camera.image[(i, j)] = color;
                    camera.alpha[(i, j)] = 1.;
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

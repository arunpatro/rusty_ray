mod primitives;
use nalgebra::Vector3;
mod image_utils;

fn render_sphere_orthographic() {
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
                    let itensity = normal.dot(&light_vector);
                    let color = 0.3 + 0.7 * itensity; // 0.3 is ambient light

                    camera.image[(i, j)] = color;
                    camera.alpha[(i, j)] = 1.;
                }
                None => {}
            }

            // let intersection = sphere.intersects(&ray);
            // match intersection {
            //     Some(intersection) => {
            //         // if intersects, calculate color
            //         let normal = sphere.normal(&intersection);
            //         let light_vector = (light.position - intersection).normalize();
            //         let itensity = normal.dot(&light_vector);

            //         camera.image[(i, j)] = itensity;
            //         camera.alpha[(i, j)] = 1.;
            //         // print!(" Intersected at ({}, {}, {})", intersection.x, intersection.y, intersection.z);
            //     }
            //     None => {
            //         // print!(" Failed to intersect");
            //     }
            // }
        }
    }

    image_utils::save_as_png(camera, "sphere_perspective.png");
}

fn main() {
    println!("Hello, world!");
    render_sphere_orthographic();
}

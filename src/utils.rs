use crate::primitives::{self, Scene};
use crate::primitives::{Ray};
use nalgebra::{Vector3, Vector4};
use crate::textures::procedural_texture;

pub fn find_closest_point(
    ray: &Ray,
    objects: &Vec<Box<dyn primitives::Object>>,
) -> Option<(usize, f32, Vector3<f32>, Vector3<f32>)> {
    let mut closest_point = None;

    for (index, object) in objects.iter().enumerate() {
        if let Some((t, p, n)) = object.intersects(&ray) {
            match closest_point {
                Some((_, old_t, _, _)) => {
                    if t < old_t {
                        closest_point = Some((index, t, p, n));
                    }
                }
                None => {
                    closest_point = Some((index, t, p, n));
                }
            }
        }
    }
    return closest_point;
}

// check if the light is visible
pub fn is_light_visible(
    light: &primitives::Light,
    point: &Vector3<f32>,
    objects: &Vec<Box<dyn primitives::Object>>,
) -> bool {
    let light_ray = Ray::new(*point, (light.position - point).normalize());
    let ans = find_closest_point(&light_ray, objects);
    match ans {
        Some((_, _, p, _)) => {
            // check if the light is visible
            if (p - point).norm() > (light.position - point).norm() {
                return true;
            } else {
                return false;
            }
        }
        // no obstacle
        None => return true,
    }
}


pub fn shoot_ray(
    ray: &Ray,
    scene: &Scene,
    material: &primitives::Material,
    max_bounce: usize,
) -> Vector4<f32> {
    let ans = find_closest_point(&ray, &scene.objects);
    match ans {
        Some((index, _, intersection, normal)) => {
            let ambient_color = material.ambient_color.component_mul(&scene.ambient_light);

            // diffuse and specular
            let mut total_color = Vector3::new(0., 0., 0.);
            for light in &scene.lights {
                if is_light_visible(&light, &intersection, &scene.objects) {
                    let mut diffuse_color = material.diffuse_color;
                    // procedural texture
                    if index == 4 {
                            // Compute UV coodinates for the point on the sphere
                            // let xyz = intersection - Vector3::new(-2., 0.4, 1.);
                            // let tu = (xyz.z / 1. ).acos() / std::f32::consts::PI;
                            // let tv = (std::f32::consts::PI + xyz.y.atan2(xyz.x)) / (2. * std::f32::consts::PI);
                            // diffuse_color = procedural_texture(tu, tv);
                    }


                    let light_vector = (light.position - intersection).normalize();
                    let bisector_direction = (light_vector - ray.direction).normalize();
                    let diffuse_coeff = normal.dot(&light_vector).max(0.);
                    let specular_coeff = normal.dot(&bisector_direction).max(0.).powf(256.);

                    let diffuse = diffuse_coeff * diffuse_color;
                    let specular = specular_coeff * material.specular_color;

                    let attenuation = (light.position - intersection).norm_squared(); // attenuation is square of distance
                    total_color += light.color.component_mul(&(diffuse + specular)) / attenuation;
                }
            }

            // reflection
            if max_bounce > 0 {
                // let reflection_direction = (ray.direction - 2. * normal.dot(&ray.direction) * normal).normalize();
                // let adjusted_origin = intersection + 1e-5 * reflection_direction;
                // let reflection_ray = Ray::new(adjusted_origin, reflection_direction);
                // let reflection_color = shoot_ray(
                //     &reflection_ray,
                //     &scene,
                //     &material,
                //     max_bounce - 1,
                // );

                // total_color += material.reflection_color.component_mul(&reflection_color.xyz());
            }

            let color = ambient_color + total_color;
            return Vector4::new(color.x, color.y, color.z, 1.);
        }
        None => {
            // no intersection
            return Vector4::new(0., 0., 0., 0.);
        }
    }
}

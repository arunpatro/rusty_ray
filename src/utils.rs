use crate::primitives;
use crate::primitives::{Ray};
use nalgebra::{Vector3, Vector4};

pub fn find_closest_point(
    ray: &Ray,
    objects: &Vec<Box<dyn primitives::Object>>,
) -> Option<(f32, Vector3<f32>, Vector3<f32>)> {
    let mut closest_point: Option<(f32, Vector3<f32>, Vector3<f32>)> = None;

    for object in objects {
        if let Some((t, p, n)) = object.intersects(&ray) {
            match closest_point {
                Some((old_t, _, _)) => {
                    if t < old_t {
                        closest_point = Some((t, p, n));
                    }
                }
                None => {
                    closest_point = Some((t, p, n));
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
        Some((_, p, _)) => {
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
    objects: &Vec<Box<dyn primitives::Object>>,
    lights: &Vec<primitives::Light>,
    ambient_light: &Vector3<f32>,
    material: &primitives::Material,
    max_bounce: usize,
) -> Vector4<f32> {
    let ans = find_closest_point(&ray, &objects);
    match ans {
        Some((_, intersection, normal)) => {
            let ambient_color = material.ambient_color.component_mul(&ambient_light);

            let mut total_color = Vector3::new(0., 0., 0.);
            for light in lights {
                if is_light_visible(&light, &intersection, &objects) {
                    let light_vector = (light.position - intersection).normalize();
                    let bisector_direction = (light_vector - ray.direction).normalize();
                    let diffuse_coeff = normal.dot(&light_vector).max(0.);
                    let specular_coeff = normal.dot(&bisector_direction).max(0.).powf(256.);

                    let diffuse = diffuse_coeff * material.diffuse_color;
                    let specular = specular_coeff * material.specular_color;

                    let attenuation = (light.position - intersection).norm_squared(); // attenuation is square of distance
                    total_color += light.color.component_mul(&(diffuse + specular)) / attenuation;
                }
            }

            // reflection
            if max_bounce > 0 {
                let reflection_direction = (ray.direction - 2. * normal.dot(&ray.direction) * normal).normalize();
                let adjusted_origin = intersection + 1e-5 * reflection_direction;
                let reflection_ray = Ray::new(adjusted_origin, reflection_direction);
                let reflection_color = shoot_ray(
                    &reflection_ray,
                    &objects,
                    &lights,
                    &ambient_light,
                    &material,
                    max_bounce - 1,
                );

                total_color += material.reflection_color.component_mul(&reflection_color.xyz());
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

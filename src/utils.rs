use crate::primitives;
use crate::primitives::{Object, Ray};
use nalgebra::Vector3;

pub fn find_closest_point(
    ray: &Ray,
    objects: &Vec<primitives::Sphere>,
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
    objects: &Vec<primitives::Sphere>,
) -> bool {
    let light_ray = Ray::new(*point, (light.position - point).normalize());
    let ans = find_closest_point(&light_ray, objects);
    match ans {
        Some((_, p, _)) => {
            // check if the light is visible
            if (p - light.position).norm() < (p - point).norm() {
                return false;
            } else {
                return true;
            }
        }
        // no obstacle
        None => return true,
    }
}

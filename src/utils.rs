use crate::primitives;
use crate::primitives::{Object, Ray};
use nalgebra::Vector3;


pub fn find_closest_point(ray: &Ray, objects: &Vec<primitives::Sphere>) -> (Option<usize>, Option<Vector3<f32>>) {
    let mut closest_point: Option<Vector3<f32>> = None;
    let mut index: Option<usize> = None;

    for (i, object) in objects.iter().enumerate() {
        if let Some(intersection) = object.intersects(&ray) {
            match closest_point {
                Some(old_closest_point) => {
                    if (intersection - ray.origin).norm_squared()
                        < (old_closest_point - ray.origin).norm_squared()
                    {
                        closest_point = Some(intersection);
                        index = Some(i);
                    }
                }
                None => {
                    closest_point = Some(intersection);
                    index = Some(i);
                }
            }
        }
    }
    return (index, closest_point);
}
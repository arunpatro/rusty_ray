// find intersection and normal of the closest object
fn find_nearest_object(ray: &Ray, scene: &Scene) -> Option<(Intersection, Normal)> {
    let mut closest_intersection: Option<(Intersection, Normal)> = None;
    for object in &scene.objects {
        let intersection = object.intersects(ray);
        match intersection {
            Some(intersection) => match closest_intersection {
                Some(closest_intersection) => {
                    if intersection.distance < closest_intersection.distance {
                        let normal = object.normal(&intersection);
                        closest_intersection = Some((intersection, normal));
                    }
                }
                None => {
                    let normal = object.normal(&intersection);
                    closest_intersection = Some((intersection, normal));
                }
            }
            None => {}
        }
    }
    return closest_intersection;
}

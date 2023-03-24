use crate::primitives::{HitPoint, Object, Ray, Triangle};
use nalgebra::Vector3;
use std::cmp::Ordering;

#[derive(Default)]
pub struct AlignedBox3d {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AlignedBox3d {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }

    pub fn extend(&mut self, point: Vector3<f32>) {
        self.min = Vector3::new(
            self.min.x.min(point.x),
            self.min.y.min(point.y),
            self.min.z.min(point.z),
        );
        self.max = Vector3::new(
            self.max.x.max(point.x),
            self.max.y.max(point.y),
            self.max.z.max(point.z),
        );
    }

    pub fn extend_triangle(&mut self, triangle: &Triangle) {
        self.extend(triangle.point1);
        self.extend(triangle.point2);
        self.extend(triangle.point3);
    }

    pub fn contains(&self, point: Vector3<f32>) -> bool {
        let check_dim = |dim, min, max| point[dim] >= min && point[dim] <= max;

        check_dim(0, self.min.x, self.max.x)
            && check_dim(1, self.min.y, self.max.y)
            && check_dim(2, self.min.z, self.max.z)
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let inv_dir = Vector3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );
        let tx1 = (self.min.x - ray.origin.x) * inv_dir.x;
        let tx2 = (self.max.x - ray.origin.x) * inv_dir.x;
        let ty1 = (self.min.y - ray.origin.y) * inv_dir.y;
        let ty2 = (self.max.y - ray.origin.y) * inv_dir.y;
        let tz1 = (self.min.z - ray.origin.z) * inv_dir.z;
        let tz2 = (self.max.z - ray.origin.z) * inv_dir.z;

        let tmin = tx1.min(tx2).max(ty1.min(ty2)).max(tz1.min(tz2));
        let tmax = tx1.max(tx2).min(ty1.max(ty2)).min(tz1.max(tz2));

        if tmax < 0. || tmin > tmax {
            return false;
        }

        true
    }
}

pub struct AABBNode {
    pub bbox: AlignedBox3d,
    pub left: Option<Box<AABBNode>>,
    pub right: Option<Box<AABBNode>>,
    pub object: Option<Box<dyn Object>>,
}

impl AABBNode {
    pub fn new(
        bbox: AlignedBox3d,
        left: Option<Box<AABBNode>>,
        right: Option<Box<AABBNode>>,
        object: Option<Box<dyn Object>>,
    ) -> Self {
        Self {
            bbox,
            left,
            right,
            object,
        }
    }
}

pub struct BVH {
    pub root: AABBNode,
}

impl BVH {
    pub fn new(triangles: &Vec<Triangle>) -> Self {
        Self {
            root: Self::create_node(triangles),
        }
    }

    fn create_node(triangles: &[Triangle]) -> AABBNode {
        if triangles.is_empty() {
            panic!("Cannot create an AABBNode with no triangles.");
        }

        let mut bbox = AlignedBox3d::default();
        for triangle in triangles {
            bbox.extend_triangle(&triangle);
        }

        // if leaf node
        if triangles.len() == 1 {
            return AABBNode::new(bbox, None, None, Some(Box::new(triangles[0].clone())));
        }

        // if not leaf node
        let diag = bbox.max - bbox.min;
        let axis_index = diag.x.max(diag.y.max(diag.z)) as usize;

        let mut sorted_triangles = triangles.to_vec();
        // sorted_triangles.sort_by(|a, b| {
        //     let a_center = a.centroid();
        //     let b_center = b.centroid();

        //     a_center[axis_index]
        //         .partial_cmp(&b_center[axis_index])
        //         .unwrap_or(Ordering::Equal)
        // });

        let mid = sorted_triangles.len() / 2;
        let (left_triangles, right_triangles) = sorted_triangles.split_at(mid);

        let left = Self::create_node(left_triangles);
        let right = Self::create_node(right_triangles);

        AABBNode::new(bbox, Some(Box::new(left)), Some(Box::new(right)), None)
    }

    pub fn intersects(&self, ray: &Ray) -> Option<HitPoint> {
        recur_intersect(&self.root, ray)
    }
}

fn recur_intersect(node: &AABBNode, ray: &Ray) -> Option<HitPoint> {
    if !node.bbox.intersects(ray) {
        return None;
    }

    match (&node.object, &node.left, &node.right) {
        (Some(object), _, _) => object.intersects(ray),
        (_, Some(left), Some(right)) => {
            let left_res = recur_intersect(left, ray);
            let right_res = recur_intersect(right, ray);

            left_res
                .into_iter()
                .chain(right_res.into_iter())
                .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal))
        }
        _ => None,
    }
}

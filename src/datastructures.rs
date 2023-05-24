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
    pub object_idx: Option<usize>,
}

impl AABBNode {
    pub fn new(
        bbox: AlignedBox3d,
        left: Option<Box<AABBNode>>,
        right: Option<Box<AABBNode>>,
        object_idx: Option<usize>,
    ) -> Self {
        Self {
            bbox,
            left,
            right,
            object_idx,
        }
    }
}

pub struct BVH {
    pub root: AABBNode,

}

impl BVH {
    pub fn new(triangles: &Vec<Triangle>) -> Self {
        let triangle_indices: Vec<usize> = (0..triangles.len()).collect();
        Self {
            root: Self::create_node(&triangles, triangle_indices),
        }
    }

    fn create_node(triangles: &[Triangle], triangle_indices: Vec<usize>) -> AABBNode {
        if triangle_indices.is_empty() {
            panic!("Cannot create an AABBNode with no triangles.");
        }

        let mut bbox = AlignedBox3d::default();
        for idx in &triangle_indices {
            bbox.extend_triangle(&triangles[*idx]);
        }

        // if leaf node
        if triangle_indices.len() == 1 {
            return AABBNode::new(bbox, None, None, Some(triangle_indices[0]));
        }

        // if not leaf node
        let diag = bbox.max - bbox.min;
        let axis_index = diag.x.max(diag.y.max(diag.z)) as usize;

        let mut sorted_indices = triangle_indices;
        // sorted_indices.sort_by(|a, b| {
        //     let a_center = triangles[*a].centroid();
        //     let b_center = triangles[*b].centroid();

        //     a_center[axis_index]
        //         .partial_cmp(&b_center[axis_index])
        //         .unwrap_or(Ordering::Equal)
        // });

        let mid = sorted_indices.len() / 2;
        let (left_triangle_indices, right_triangle_indices) = sorted_indices.split_at(mid);

        let left = Self::create_node(triangles, left_triangle_indices.to_vec());
        let right = Self::create_node(triangles, right_triangle_indices.to_vec());

        AABBNode::new(bbox, Some(Box::new(left)), Some(Box::new(right)), None)
    }

    
}

// fn recur_intersect(node: &AABBNode, ray: &Ray) -> Option<HitPoint> {
//     if !node.bbox.intersects(ray) {
//         return None;
//     }

//     match (&node.object, &node.left, &node.right) {
//         (Some(object), _, _) => object.intersects(ray),
//         (_, Some(left), Some(right)) => {
//             let left_res = recur_intersect(left, ray);
//             let right_res = recur_intersect(right, ray);

//             left_res
//                 .into_iter()
//                 .chain(right_res.into_iter())
//                 .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal))
//         }
//         _ => None,
//     }
// }

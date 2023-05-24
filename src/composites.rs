use crate::datastructures::{AABBNode, BVH};
use crate::primitives::{HitPoint, Object, Ray, Triangle};
use nalgebra::{DMatrix, Matrix3, Vector3, Vector4};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bvh: BVH,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let bvh = BVH::new(&triangles);
        Self { triangles, bvh }
    }

    pub fn from_off_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut header = String::new();
        reader.read_line(&mut header).unwrap();
        assert!(header.starts_with("OFF"));

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let tokens: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let (num_vertices, num_faces) = (tokens[0], tokens[1]);

        let mut vertices = Vec::new();
        for _ in 0..num_vertices {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let vertex: Vec<f32> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            vertices.push(Vector3::new(vertex[0], vertex[1], vertex[2]));
        }

        let mut triangles = Vec::new();
        for _ in 0..num_faces {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let face: Vec<u32> = line
                .split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect();
            let triangle = Triangle::new(
                vertices[face[0] as usize],
                vertices[face[1] as usize],
                vertices[face[2] as usize],
            );
            triangles.push(triangle);
        }

        Self::new(triangles)
    }

    fn stack_intersect(&self, node: &AABBNode, ray: &Ray) -> Option<HitPoint> {
        let mut stack = vec![node];

        let mut closest_hit_point = None;
        let mut closest_t = f32::INFINITY;

        while let Some(node) = stack.pop() {
            if !node.bbox.intersects(ray) {
                continue;
            }

            match (node.object_idx, &node.left, &node.right) {
                (Some(object_idx), _, _) => {
                    if let Some(hit_point) = self.triangles[object_idx].intersects(ray) {
                        if hit_point.t < closest_t {
                            closest_hit_point = Some(hit_point);
                            closest_t = hit_point.t;
                        }
                    }
                }
                (_, Some(left), Some(right)) => {
                    stack.push(left);
                    stack.push(right);
                }
                _ => {}
            }
        }

        closest_hit_point
    }
}

impl Object for Mesh {
    fn intersects(&self, ray: &Ray) -> Option<HitPoint> {
        // self.bvh.intersects(&ray)
        self.stack_intersect(&self.bvh.root, ray)

        // this is the brute force way of doing it
        // self.triangles
        //     .iter()
        //     .filter_map(|triangle| triangle.intersects(ray))
        //     .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal))
    }

    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        // this is not correct, because its very cumbersome to calculate which triangle the point is in and then get the normal from that triangle
        // we can calculate baricentric coordinates to get the triangle, and accelerate with a BVH
        Vector3::new(0., 0., 0.)
    }
}

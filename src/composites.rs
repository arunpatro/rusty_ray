use crate::primitives::{Triangle, Ray, Object};
use nalgebra::{DMatrix, Matrix3, Vector3, Vector4};
use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
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

        Self { triangles }
    }
}

impl Object for Mesh {
    fn intersects(&self, ray: &Ray) -> Option<(f32, Vector3<f32>, Vector3<f32>)> {
        let mut closest_t = std::f32::MAX;
        let mut closest_point = Vector3::new(0., 0., 0.);
        let mut closest_normal = Vector3::new(0., 0., 0.);

        for triangle in &self.triangles {
            if let Some((t, point, normal)) = triangle.intersects(ray) {
                if t < closest_t {
                    closest_t = t;
                    closest_point = point;
                    closest_normal = normal;
                }
            }
        }

        if closest_t == std::f32::MAX {
            None
        } else {
            Some((closest_t, closest_point, closest_normal))
        }
    }

    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        // this is not correct, because its very cumbersome to calculate which triangle the point is in and then get the normal from that triangle
        // we can calculate baricentric coordinates to get the triangle, and accelerate with a BVH
        Vector3::new(0., 0., 0.)
    }
}

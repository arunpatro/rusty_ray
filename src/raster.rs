use crate::{composites, primitives};
use nalgebra::{Matrix3, Vector3};

pub fn rasterize(
    mesh: &composites::Mesh,
    camera: &primitives::Camera,
    frame_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) {
    // init the meta triangles
    // using literal construction is more verbose than implementing the new constructor
    let mut triangles: Vec<MetaTriangle> = Vec::new();
    for triangle in &mesh.triangles {
        let normal = triangle.normal();
        let v0 = Vertex {
            position: triangle.point1,
            normal: normal,
        };
        let v1 = Vertex {
            position: triangle.point2,
            normal: normal,
        };
        let v2 = Vertex {
            position: triangle.point3,
            normal: normal,
        };
        triangles.push(MetaTriangle {
            v0: v0,
            v1: v1,
            v2: v2,
        });
    }

    // run the vertex shader, modify the vertices of the triangles
    for triangle in &mut triangles {
        triangle.v0 = VertexShader(&triangle.v0);
        triangle.v1 = VertexShader(&triangle.v1);
        triangle.v2 = VertexShader(&triangle.v2);
    }

    // run the fragment shader, rasterize the triangles
    // for every pixel in the frame buffer, find the triangle that contains it
    // and run the fragment shader on it
    for triangle in &triangles {
        for i in 0..frame_buffer.width() {
            for j in 0..frame_buffer.height() {
                let pixel = nalgebra::Vector3::new(i as f32 + 0.5, j as f32 + 0.5, 1.);
                let bary_coords = barycentric_coords(&triangle, &pixel);
                if bary_coords.min() >= 0. {
                    let v = vertexInterpolation(&triangle, bary_coords);
                    // only render in the biunit cube
                    if v.position.z >= -1. && v.position.z <= 1. {
                        let fragment = FragmentShader(&v);
                        let blend = BlendingShader(&fragment, &frame_buffer.get_pixel(i, j));
                        frame_buffer.put_pixel(i, j, blend);
                    }
                }
            }
        }
    }
}

fn barycentric_coords(triangle: &MetaTriangle, pixel: &Vector3<f32>) -> Vector3<f32> {
    let p = Matrix3::new(
        triangle.v0.position.x,
        triangle.v1.position.x,
        triangle.v2.position.x,
        triangle.v0.position.y,
        triangle.v1.position.y,
        triangle.v2.position.y,
        1.0,
        1.0,
        1.0,
    );

    match p.try_inverse() {
        Some(p_inverse) => p_inverse * pixel,
        None => Vector3::new(-1.0, -1.0, -1.0),
    }
}

fn vertexInterpolation(triangle: &MetaTriangle, bary_coords: nalgebra::Vector3<f32>) -> Vertex {
    let vertex = Vertex {
        position: triangle.v0.position * bary_coords.x
            + triangle.v1.position * bary_coords.y
            + triangle.v2.position * bary_coords.z,
        normal: triangle.v0.normal * bary_coords.x
            + triangle.v1.normal * bary_coords.y
            + triangle.v2.normal * bary_coords.z,
    };
    vertex
}

// this is required for the rasterization algorithm,
// this is the data structure that is used to store
// the triangles with additional vertex data like normals
pub struct MetaTriangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

pub struct Vertex {
    pub position: nalgebra::Vector3<f32>,
    pub normal: nalgebra::Vector3<f32>,
}

pub struct Fragment {
    pub position: nalgebra::Vector3<f32>,
    pub color: nalgebra::Vector3<f32>,
}

fn VertexShader(vertex: &Vertex) -> Vertex {
    let vertex = Vertex {
        position: vertex.position,
        normal: vertex.normal,
    };
    vertex
}

fn FragmentShader(vertex: &Vertex) -> Fragment {
    let color = nalgebra::Vector3::new(0., 1., 0.);
    let fragment = Fragment {
        position: vertex.position,
        color: color,
    };
    fragment
}

fn BlendingShader(fragment: &Fragment, pixel: &image::Rgba<u8>) -> image::Rgba<u8> {
    let color = image::Rgba([
        (fragment.color.x * 255.) as u8,
        (fragment.color.y * 255.) as u8,
        (fragment.color.z * 255.) as u8,
        255,
    ]);
    color
}


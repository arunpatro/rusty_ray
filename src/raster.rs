use crate::{
    composites,
    primitives::{self, Camera, Light},
};
use nalgebra::{DMatrix, Matrix3, Vector3, Vector4};
#[macro_export]

macro_rules! print_matrix_row_major {
    ($var_name:expr, $matrix:expr) => {
        println!(
            "{}: {:.4}, {:.4}, {:.4}",
            $var_name,
            $matrix[(0, 0)],
            $matrix[(0, 1)],
            $matrix[(0, 2)]
        );
        println!(
            "   {:.4}, {:.4}, {:.4}",
            $matrix[(1, 0)],
            $matrix[(1, 1)],
            $matrix[(1, 2)]
        );
        println!(
            "   {:.4}, {:.4}, {:.4}",
            $matrix[(2, 0)],
            $matrix[(2, 1)],
            $matrix[(2, 2)]
        );
    };
}

pub fn rasterize(
    mesh: &composites::Mesh,
    uniform: Uniform,
    program: Program,
    frame_buffer: &mut DMatrix<Vector4<f64>>,
) {
    // init the meta triangles
    // using literal construction is more verbose than implementing the new constructor
    // let frame_buffer = &mut camera.image;
    let mut z_buffer = DMatrix::from_element(
        frame_buffer.nrows(),
        frame_buffer.ncols(),
        f64::NEG_INFINITY,
    );
    let mut triangles: Vec<ShaderTriangle> = Vec::new();
    for (i, triangle) in mesh.triangles.iter().enumerate() {
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

        triangles.push(ShaderTriangle {
            v0: v0,
            v1: v1,
            v2: v2,
        });
    }

    // run the vertex shader, modify the vertices of the triangles
    for triangle in &mut triangles {
        triangle.v0 = vertex_shader(&triangle.v0, &uniform);
        triangle.v1 = vertex_shader(&triangle.v1, &uniform);
        triangle.v2 = vertex_shader(&triangle.v2, &uniform);
    }

    // rasterize the triangles by running the fragment shader on every pixel
    // for every triangle, find pixels that are inside the bounding box of the triangle
    // compute the barycentric coordinates of the pixel, its interpolated attributes
    // and run the fragment shader on it
    for (i, triangle) in triangles.iter().enumerate() {
        // Collect coordinates into a matrix and convert to canonical representation
        let mut p = nalgebra::Matrix3::new(
            triangle.v0.position.x,
            triangle.v0.position.y,
            triangle.v0.position.z,
            triangle.v1.position.x,
            triangle.v1.position.y,
            triangle.v1.position.z,
            triangle.v2.position.x,
            triangle.v2.position.y,
            triangle.v2.position.z,
        );

        // print_matrix_row_major!("<p", p);

        // Coordinates are in -1..1, rescale to pixel size (x,y only)
        // TODO row_mut gives wrong ans, column mut works, really confused about this notation!
        p.column_mut(0)
            .iter_mut()
            .for_each(|x| *x = (*x + 1.) / 2. * frame_buffer.ncols() as f64);
        p.column_mut(1)
            .iter_mut()
            .for_each(|y| *y = (*y + 1.) / 2. * frame_buffer.nrows() as f64);

        let lx = p.column(0).min().floor() as usize;
        let ly = p.column(1).min().floor() as usize;
        let ux = p.column(0).max().ceil() as usize;
        let uy = p.column(1).max().ceil() as usize;

        // clamp the bounding box to the frame buffer
        let lx = lx.max(0).min(frame_buffer.ncols() - 1);
        let ly = ly.max(0).min(frame_buffer.nrows() - 1);
        let ux = ux.max(0).min(frame_buffer.ncols() - 1);
        let uy = uy.max(0).min(frame_buffer.nrows() - 1);

        // print_matrix_row_major!("p", p);
        // println!("lx: {}, ly: {}, ux: {}, uy: {}", lx, ly, ux, uy);

        // Build the implicit triangle representation from matrix p
        // and set the last row to 1
        let mut matrix_A = p.transpose();
        matrix_A[(2, 0)] = 1.;
        matrix_A[(2, 1)] = 1.;
        matrix_A[(2, 2)] = 1.;

        let matrix_A_inv = matrix_A.try_inverse().unwrap();
        // this is the same as using adjugate logic, differs from cpp eigen!!

        // print_matrix_row_major!("A", matrix_A);
        // print_matrix_row_major!("Ai", matrix_A_inv);

        for i in lx..=ux {
            for j in ly..=uy {
                let pixel = nalgebra::Vector3::new(i as f64 + 0.5, j as f64 + 0.5, 1.);
                let bary_coords = matrix_A_inv * pixel;
                // println!("pixel: {:.4}, {:.4}, {:.4}", pixel.x, pixel.y, pixel.z);
                // println!(
                //     "i: {}, j: {}, b: {:.4}, {:.4}, {:.4}",
                //     i, j, bary_coords[0], bary_coords[1], bary_coords[2]
                // );
                if bary_coords.min() >= 0. {
                    let v = vertex_interpolation(&triangle, bary_coords);
                    // println!(
                    //     "va: {:.4}, {:.4}, {:.4}",
                    //     v.position.x, v.position.y, v.position.z
                    // );
                    // only render in the biunit cube
                    if v.position.z <= 1. {
                        // if v.position.z >= -1. && v.position.z <= 1. {
                        let fragment = fragment_shader(&v, &uniform);
                        // let blend = blending_shader(&fragment, camera.image[(i, j)]);
                        let (blend, new_z) =
                            blending_shader(&fragment, frame_buffer[(i, j)], z_buffer[(i, j)]);
                        frame_buffer[(i, j)] = blend;
                        z_buffer[(i, j)] = new_z;
                    }
                }
            }
        }

        // println!("value at 350 350 {:?}", frame_buffer[(350, 350)]);
    }
}

fn vertex_interpolation(triangle: &ShaderTriangle, bary_coords: nalgebra::Vector3<f64>) -> Vertex {
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

// this should store some globals that the shader can access
pub struct Uniform {
    camera_pos: Vector3<f64>,
    ambient_color: Vector3<f64>,
    light_pos: Vector3<f64>,
    light_color: Vector3<f64>,
}

impl Uniform {
    pub fn new(
        camera_pos: Vector3<f64>,
        ambient_color: Vector3<f64>,
        light_pos: Vector3<f64>,
        light_color: Vector3<f64>,
    ) -> Self {
        Self {
            camera_pos,
            ambient_color,
            light_pos,
            light_color,
        }
    }
}

// this is required for the rasterization algorithm,
// this is the data structure that is used to store
// the triangles with additional vertex data like normals
#[derive(Debug)]
pub struct ShaderTriangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

#[derive(Debug)]
pub struct Vertex {
    pub position: nalgebra::Vector3<f64>,
    pub normal: nalgebra::Vector3<f64>,
}

pub struct Fragment {
    pub position: nalgebra::Vector3<f64>,
    pub color: nalgebra::Vector3<f64>,
}

pub struct Program {
    pub vertex_shader: fn(vertex: &Vertex, uniform: &Uniform) -> Vertex,
    pub fragment_shader: fn(vertex: &Vertex, uniform: &Uniform) -> Fragment,
    pub blending_shader: fn(&Fragment, Vector4<f64>, f64) -> (Vector4<f64>, f64),
}

// not sure where to initialize this, or make it default
impl Program {
    pub fn new(
        vertex_shader: fn(&Vertex, &Uniform) -> Vertex,
        fragment_shader: fn(&Vertex, &Uniform) -> Fragment,
        blending_shader: fn(&Fragment, Vector4<f64>, f64) -> (Vector4<f64>, f64),
    ) -> Program {
        Program {
            vertex_shader,
            fragment_shader,
            blending_shader,
        }
    }
}

// TODO provide the shader definitions in the main program
// for now here
pub fn vertex_shader(vertex: &Vertex, uniform: &Uniform) -> Vertex {

    // model transform: model matrix
    let model_matrix = nalgebra::Matrix4::identity();

    // view transform: camera matrix
    let camera_matrix = nalgebra::Matrix4::new(
        1., 0., 0., -uniform.camera_pos.x,
        0., 1., 0., -uniform.camera_pos.y,
        0., 0., 1., -uniform.camera_pos.z,
        0., 0., 0., 1.,
    );

    // projection transform: perspective matrix
    

    let final_matrix = camera_matrix * model_matrix;
    let new_pos = final_matrix * vertex.position.push(1.);
    let vertex = Vertex {
        position: new_pos.xyz(),
        normal: vertex.normal,
    };
    vertex
}

pub fn fragment_shader(vertex: &Vertex, uniform: &Uniform) -> Fragment {
    let light_direction = (uniform.light_pos - vertex.position).normalize();
    let mut normal = vertex.normal.normalize();
    if light_direction.dot(&normal) < 0. {
        normal = -normal;
    }
    let diffuse = light_direction.dot(&normal).max(0.);
    let diffuse_color = diffuse * uniform.light_color;
    let ambient_color = uniform.ambient_color;

    let fragment = Fragment {
        position: vertex.position,
        color: diffuse_color + ambient_color,
    };
    fragment
}

pub fn blending_shader(
    fragment: &Fragment,
    old_color: Vector4<f64>,
    old_z: f64,
) -> (Vector4<f64>, f64) {
    let color = Vector4::new(fragment.color.x, fragment.color.y, fragment.color.z, 1.);
    let z = fragment.position.z;
    if z > old_z {
        (color, z)
    } else {
        (old_color, old_z)
    }
}

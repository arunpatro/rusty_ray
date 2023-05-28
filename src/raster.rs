use crate::{composites, primitives};
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
    camera: &mut primitives::Camera,
    lights: &[primitives::Light],
) {
    // init the meta triangles
    // using literal construction is more verbose than implementing the new constructor
    let frame_buffer = &mut camera.image;
    let mut z_buffer = DMatrix::from_element(camera.width, camera.height, f32::NEG_INFINITY);
    let mut triangles: Vec<ShaderTriangle> = Vec::new();
    for (i, triangle) in mesh.triangles.iter().enumerate() {
        // if i == 25 {
        //     println!("T> {}:", i);
        // }
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
        // print the new vertices like this
        // this seems correct
        // println!(
        //     "v[{}]: {:.4}, {:.4}, {:.4}",
        //     3*i, v0.position.x, v0.position.y, v0.position.z
        // );
        // println!(
        //     "v[{}]: {:.4}, {:.4}, {:.4}",
        //     3*i+1, v1.position.x, v1.position.y, v1.position.z
        // );
        // println!(
        //     "v[{}]: {:.4}, {:.4}, {:.4}",
        //     3*i+2, v2.position.x, v2.position.y, v2.position.z
        // );

        triangles.push(ShaderTriangle {
            v0: v0,
            v1: v1,
            v2: v2,
        });
    }

    // run the vertex shader, modify the vertices of the triangles
    for triangle in &mut triangles {
        triangle.v0 = vertex_shader(&triangle.v0);
        triangle.v1 = vertex_shader(&triangle.v1);
        triangle.v2 = vertex_shader(&triangle.v2);
    }

    // rasterize the triangles by running the fragment shader on every pixel
    // for every triangle, find pixels that are inside the bounding box of the triangle
    // compute the barycentric coordinates of the pixel, its interpolated attributes
    // and run the fragment shader on it
    for (i, triangle) in triangles.iter().enumerate() {
        if i >= 5 {
            break;
        }
        // println!("Raster T: {}", i);
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
            .for_each(|x| *x = (*x + 1.) / 2. * camera.width as f32);
        p.column_mut(1)
            .iter_mut()
            .for_each(|y| *y = (*y + 1.) / 2. * camera.height as f32);

        let lx = p.column(0).min().floor() as usize;
        let ly = p.column(1).min().floor() as usize;
        let ux = p.column(0).max().ceil() as usize;
        let uy = p.column(1).max().ceil() as usize;

        // clamp the bounding box to the frame buffer
        let lx = lx.max(0).min(camera.width - 1);
        let ly = ly.max(0).min(camera.height - 1);
        let ux = ux.max(0).min(camera.width - 1);
        let uy = uy.max(0).min(camera.height - 1);

        // print_matrix_row_major!("p", p);

        println!("lx: {}, ly: {}, ux: {}, uy: {}", lx, ly, ux, uy);

        // Build the implicit triangle representation from matrix p
        let mut matrix_A = p.transpose();
        // set the last row to 1
        matrix_A[(2, 0)] = 1.;
        matrix_A[(2, 1)] = 1.;
        matrix_A[(2, 2)] = 1.;

        let matrix_A_inv = matrix_A.try_inverse().unwrap();
        // this is the same as using adjugate logic, differs from cpp eigen!!

        print_matrix_row_major!("A", matrix_A);
        print_matrix_row_major!("Ai", matrix_A_inv);

        // // print the matrix A in row major
        // std::printf("A: %f, %f, %f\n", A(0,0), A(0,1), A(0,2));
        // std::printf("   %f, %f, %f\n", A(1,0), A(1,1), A(1,2));
        // std::printf("   %f, %f, %f\n", A(2,0), A(2,1), A(2,2));

        for i in lx..=ux {
            for j in ly..=uy {
                let pixel = nalgebra::Vector3::new(i as f32 + 0.5, j as f32 + 0.5, 1.);
                let bary_coords = matrix_A_inv * pixel;
                // println!("pixel: {:.4}, {:.4}, {:.4}", pixel.x, pixel.y, pixel.z);
                println!(
                    "i: {}, j: {}, b: {:.4}, {:.4}, {:.4}",
                    i, j, bary_coords[0], bary_coords[1], bary_coords[2]
                );
                // if i == 250 && j == 240 {
                // println!("i {} j {}", i, j);
                // println!("baryx {:?}", bary_coords);
                // println!("pixel {:?}", pixel);
                // print_matrix_row_major!("mA", mA);
                // print_matrix_row_major!("mAi", mAi);
                // println!("v {:?}", v);
                // }
                if bary_coords.min() >= 0. {
                    let v = vertex_interpolation(&triangle, bary_coords);
                    println!(
                        "va: {:.4}, {:.4}, {:.4}",
                        v.position.x, v.position.y, v.position.z
                    );
                    // only render in the biunit cube
                    if v.position.z >= -1. && v.position.z <= 1. {
                        let fragment = fragment_shader(&v, lights);
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

fn vertex_interpolation(triangle: &ShaderTriangle, bary_coords: nalgebra::Vector3<f32>) -> Vertex {
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
pub struct Uniform {}

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
    pub position: nalgebra::Vector3<f32>,
    pub normal: nalgebra::Vector3<f32>,
}

pub struct Fragment {
    pub position: nalgebra::Vector3<f32>,
    pub color: nalgebra::Vector3<f32>,
}

pub struct Program {
    pub vertex_shader: fn(&Vertex) -> Vertex,
    pub fragment_shader: fn(&Vertex, &[primitives::Light]) -> Fragment,
    pub blending_shader: fn(&Fragment, &Fragment) -> Fragment,
}

// not sure where to initialize this, or make it default
impl Program {
    pub fn new(
        vertex_shader: fn(&Vertex) -> Vertex,
        fragment_shader: fn(&Vertex, &[primitives::Light]) -> Fragment,
        blending_shader: fn(&Fragment, &Fragment) -> Fragment,
    ) -> Program {
        Program {
            vertex_shader,
            fragment_shader,
            blending_shader,
        }
    }
}

// TODO provide the shader definitions in the main program
fn vertex_shader(vertex: &Vertex) -> Vertex {
    let vertex = Vertex {
        position: vertex.position,
        normal: vertex.normal,
    };
    vertex
}

fn fragment_shader(vertex: &Vertex, lights: &[primitives::Light]) -> Fragment {
    let light_direction = (lights[0].position - vertex.position).normalize();
    let mut normal = vertex.normal.normalize();
    if light_direction.dot(&normal) < 0. {
        normal = -normal;
    }
    let diffuse = light_direction.dot(&normal).max(0.);
    let diffuse_color = diffuse * lights[0].color;
    let ambient_color = Vector3::new(0.3, 0.3, 0.3);

    let fragment = Fragment {
        position: vertex.position,
        color: diffuse_color + ambient_color,
    };
    fragment
}

fn blending_shader(
    fragment: &Fragment,
    old_color: Vector4<f32>,
    old_z: f32,
) -> (Vector4<f32>, f32) {
    let color = Vector4::new(fragment.color.x, fragment.color.y, fragment.color.z, 1.);
    let z = fragment.position.z;
    if z > old_z {
        (color, z)
    } else {
        (old_color, old_z)
    }
}

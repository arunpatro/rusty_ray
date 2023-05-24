use crate::{composites, primitives};
use nalgebra::{DMatrix, Matrix3, Vector3, Vector4};
#[macro_export]

macro_rules! print_matrix_row_major {
    ($var_name:expr, $matrix:expr) => {
        println!("Matrix {}:", $var_name);
        for row in 0..$matrix.nrows() {
            for col in 0..$matrix.ncols() {
                print!(" > {:.3} ", $matrix[(row, col)]);
            }
            println!();
        }
    };
}

pub fn rasterize(mesh: &composites::Mesh, camera: &mut primitives::Camera, lights: &[primitives::Light]) {
    // init the meta triangles
    // using literal construction is more verbose than implementing the new constructor
    let frame_buffer = &mut camera.image;
    let mut z_buffer = DMatrix::from_element(camera.width, camera.height, -2.);
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
        let mut p = nalgebra::Matrix3::new(
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

        // println!(" p {:?} ", p);
        // Scale and translate columns 0 and 1
        p.row_mut(0)
            .iter_mut()
            .for_each(|x| *x = (*x + 1.) / 2. * camera.width as f32);
        p.row_mut(1)
            .iter_mut()
            .for_each(|y| *y = (*y + 1.) / 2. * camera.height as f32);

        let lx = p.row(0).min() as usize;
        let ly = p.row(1).min() as usize;
        let ux = p.row(0).max() as usize;
        let uy = p.row(1).max() as usize;

        // clamp the bounding box to the frame buffer
        let lx = lx.max(0).min(camera.width - 1);
        let ly = ly.max(0).min(camera.height - 1);
        let ux = ux.max(0).min(camera.width - 1);
        let uy = uy.max(0).min(camera.height - 1);

        // (lx, ly, ux, uy)
        // print_matrix_row_major!("p_new", p);
        // println!("{} {} {} {}", lx, ly, ux, uy);

        // Build the implicit triangle representation from matrix p
        let matrix_A = p;
        let matrix_A_inv = matrix_A.try_inverse().unwrap();

        for i in lx..ux {
            for j in ly..uy {
                let pixel = nalgebra::Vector3::new(i as f32 + 0.5, j as f32 + 0.5, 1.);
                let bary_coords = matrix_A_inv * pixel;
                let v = vertexInterpolation(&triangle, bary_coords);
                if i == 250 && j == 240 {
                    // println!("i {} j {}", i, j);
                    // println!("baryx {:?}", bary_coords);
                    // println!("pixel {:?}", pixel);
                    // print_matrix_row_major!("mA", mA);
                    // print_matrix_row_major!("mAi", mAi);
                    // println!("v {:?}", v);
                }
                if bary_coords.min() >= 0. {
                    // only render in the biunit cube
                    if v.position.z >= -1. && v.position.z <= 1. {
                        let fragment = FragmentShader(&v, lights);
                        // let blend = BlendingShader(&fragment, camera.image[(i, j)]);
                        let (blend, new_z) = BlendingShader(&fragment, frame_buffer[(i, j)], z_buffer[(i, j)]);
                        frame_buffer[(i, j)] = blend;
                        z_buffer[(i, j)] = new_z;
                    }
                }
            }
        }

        // println!("value at 350 350 {:?}", frame_buffer[(350, 350)]);
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
#[derive(Debug)]
pub struct MetaTriangle {
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

fn VertexShader(vertex: &Vertex) -> Vertex {
    let vertex = Vertex {
        position: vertex.position,
        normal: vertex.normal,
    };
    vertex
}

fn FragmentShader(vertex: &Vertex, lights: &[primitives::Light]) -> Fragment {
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

fn BlendingShader(fragment: &Fragment, old_color: Vector4<f32>, old_z: f32) -> (Vector4<f32>, f32) {
    let color = Vector4::new(fragment.color.x, fragment.color.y, fragment.color.z, 1.);
    let z = fragment.position.z;
    if z > old_z {
        (color, z)
    } else {
        (old_color, old_z)
    }
}
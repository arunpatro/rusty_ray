use nalgebra::{DMatrix, Matrix3, Vector3, Vector4};

pub enum CameraKind {
    ORTHOGRAPHIC,
    PERSPECTIVE,
}

pub struct Camera {
    pub fov: f32,
    pub focal_length: f32,
    pub width: usize,
    pub height: usize,
    pub image: DMatrix<Vector4<f32>>,
    pub position: Vector3<f32>,
    pub kind: CameraKind,
    screen_origin: Vector3<f32>,
    x_displacement: Vector3<f32>,
    y_displacement: Vector3<f32>,
}

impl Camera {
    pub fn new(
        fov: f32,
        focal_length: f32,
        width: usize,
        height: usize,
        position: Vector3<f32>,
        kind: CameraKind,
    ) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let image_y = 2. * (fov / 2.0).tan() * focal_length;
        let image_x = image_y * aspect_ratio;
        let screen_origin = Vector3::new(-image_x, image_y, position.z - focal_length);
        let x_displacement = Vector3::new(2.0 / width as f32 * image_x, 0., 0.);
        let y_displacement = Vector3::new(0., -2.0 / height as f32 * image_y, 0.);

        Self {
            fov,
            focal_length,
            width,
            height,
            x_displacement,
            y_displacement,
            screen_origin,
            image: DMatrix::from_element(width, height, Vector4::zeros()),
            position,
            kind,
        }
    }

    pub fn ray(&self, i: usize, j: usize) -> Ray {
        let screen_point = self.screen_origin
            + (i as f32 + 0.5) * self.x_displacement
            + (j as f32 + 0.5) * self.y_displacement;

        match self.kind {
            CameraKind::ORTHOGRAPHIC => {
                let origin = self.position + Vector3::new(screen_point.x, screen_point.y, 0.);
                let direction = Vector3::new(0., 0., -1.);
                Ray::new(origin, direction)
            }
            CameraKind::PERSPECTIVE => {
                let origin = self.position;
                let direction = (screen_point - self.position).normalize();
                Ray::new(origin, direction)
            }
        }
    }
}

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl Light {
    pub fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self { position, color }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }
}
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<HitPoint> {
        let a = ray.direction.norm_squared();
        let b = 2. * ray.direction.dot(&(ray.origin - self.center));
        let c = (ray.origin - self.center).norm_squared() - self.radius.powi(2);
        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            None
        } else {
            let t1 = (-b + discriminant.sqrt()) / (2. * a);
            let t2 = (-b - discriminant.sqrt()) / (2. * a);
            let t = if t1 < t2 { t1 } else { t2 };
            if t > 0. {
                let point = ray.origin + t * ray.direction;
                Some(HitPoint {
                    t,
                    point,
                    normal: self.normal(&point),
                })
            } else {
                None
            }
        }
    }

    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        (point - self.center).normalize()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitPoint {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<HitPoint>;
    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32>;
}

pub struct Material {
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,
    pub specular_exponent: f32,
    pub reflection_color: Vector3<f32>,
    pub refraction_color: Vector3<f32>,
}

impl Material {
    pub fn new(
        diffuse_color: Vector3<f32>,
        specular_color: Vector3<f32>,
        specular_exponent: f32,
        reflection_color: Vector3<f32>,
        refraction_color: Vector3<f32>,
    ) -> Self {
        Self {
            diffuse_color,
            specular_color,
            specular_exponent,
            reflection_color,
            refraction_color,
        }
    }
}

pub struct Parallelogram {
    pub point1: Vector3<f32>,
    pub point2: Vector3<f32>,
    pub point3: Vector3<f32>,
}

impl Parallelogram {
    pub fn new(point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) -> Self {
        Self {
            point1,
            point2,
            point3,
        }
    }
}

impl Object for Parallelogram {
    fn intersects(&self, ray: &Ray) -> Option<HitPoint> {
        let bystem = self.point1 - ray.origin;
        let asystem = Matrix3::from_columns(&[
            self.point1 - self.point2,
            self.point1 - self.point3,
            ray.direction,
        ]);
        let uvt = asystem.lu().solve(&bystem).unwrap();
        let t = uvt[2];

        if t < 1e-6 {
            // if t < 0. {
            None
        } else {
            let point = ray.origin + t * ray.direction;
            let mut normal = self.normal(&point);
            if normal.dot(&ray.direction) > 0. {
                normal = -normal;
            }
            Some(HitPoint { t, point, normal })
        }
    }

    fn normal(&self, _point: &Vector3<f32>) -> Vector3<f32> {
        (self.point2 - self.point1)
            .cross(&(self.point3 - self.point1))
            .normalize()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub point1: Vector3<f32>,
    pub point2: Vector3<f32>,
    pub point3: Vector3<f32>,
}

impl Triangle {
    pub fn new(point1: Vector3<f32>, point2: Vector3<f32>, point3: Vector3<f32>) -> Self {
        Self {
            point1,
            point2,
            point3,
        }
    }

    pub fn centroid(&self) -> Vector3<f32> {
        (self.point1 + self.point2 + self.point3) / 3.
    }

    pub fn normal(&self) -> Vector3<f32> {
        (self.point2 - self.point1)
            .cross(&(self.point3 - self.point1))
            .normalize()
    }
}

impl Object for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<HitPoint> {
        let bystem = self.point1 - ray.origin;
        let asystem = Matrix3::from_columns(&[
            self.point1 - self.point2,
            self.point1 - self.point3,
            ray.direction,
        ]);
        let uvt = asystem.lu().solve(&bystem);
        match uvt {
            Some(uvt) => {
                let u = uvt[0];
                let v = uvt[1];
                let t = uvt[2];

                if u < 0. || v < 0. || u + v >= 1. || t < 1e-6 {
                    None
                } else {
                    let point = ray.origin + t * ray.direction;
                    // let mut normal = self.normal(&point);
                    let mut normal = Object::normal(self, &point); //this is required because we wan't to have a default implementation of the normal function in the base struct 
                    if normal.dot(&ray.direction) > 0. {
                        normal = -normal;
                    }
                    Some(HitPoint { t, point, normal })
                }
            }
            None => None,
        }
    }

    fn normal(&self, _point: &Vector3<f32>) -> Vector3<f32> {
        self.normal()
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub ambient_color: Vector3<f32>,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<dyn Object>>,
        lights: Vec<Light>,
        ambient_color: Vector3<f32>,
    ) -> Self {
        Self {
            objects,
            lights,
            ambient_color,
        }
    }
}

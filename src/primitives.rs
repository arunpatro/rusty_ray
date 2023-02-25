use nalgebra::{DMatrix, Vector3, Vector4};

pub struct Camera {
    pub fov: f32,
    pub position: Vector3<f32>,
    screen_origin: Vector3<f32>,
    pub image: DMatrix<Vector4<f32>>,
    width: usize,
    height: usize,
}

impl Camera {
    pub fn new(fov: f32, position: Vector3<f32>, width: usize, height: usize) -> Self {
        let aspect_ratio = width as f32 / height as f32;

        Self {
            fov: fov,
            position: position,
            width: width,
            height: height,
            screen_origin: Vector3::new(-12., 6., 1.),
            image: DMatrix::from_element(width, height, Vector4::zeros()),
        }
    }

    pub fn ray(&self, i: usize, j: usize) -> Ray {
        let screen_point = self.screen_origin
            + Vector3::new(
                i as f32 * 24. / self.width as f32,
                j as f32 * 12. / self.height as f32 * -1., // y displacement is negative
                0.,                                        // z is always 0
            );

        // let direction = screen_point - self.position;
        let direction = Vector3::new(0., 0., -1.);
        let direction = direction.normalize();
        Ray::new(screen_point, direction)
    }
}

pub struct Light {
    pub position: Vector3<f32>,
}

impl Light {
    pub fn new(position: Vector3<f32>) -> Self {
        Self { position }
    }
}

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
    center: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Vector3<f32>> {
        let a = ray.direction.norm_squared();
        let b = 2. * ray.direction.dot(&(ray.origin - self.center));
        let c = (ray.origin - self.center).norm_squared() - self.radius.powi(2);
        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            return None;
        } else {
            let t1 = (-b + discriminant.sqrt()) / (2. * a);
            let t2 = (-b - discriminant.sqrt()) / (2. * a);
            let t = if t1 < t2 { t1 } else { t2 };
            return Some(ray.origin + t * ray.direction);
        }
    }

    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        let normal = (point - self.center).normalize();
        return normal;
    }
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<Vector3<f32>>;
    fn normal(&self, point: &Vector3<f32>) -> Vector3<f32>;
}


pub struct Material {
    pub ambient_color: Vector3<f32>,
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,
    pub specular_exponent: f32,
    pub reflection_color: Vector3<f32>,
    pub refraction_color: Vector3<f32>,
}

impl Material {
    pub fn new(
        ambient_color: Vector3<f32>,
        diffuse_color: Vector3<f32>,
        specular_color: Vector3<f32>,
        specular_exponent: f32,
        reflection_color: Vector3<f32>,
        refraction_color: Vector3<f32>,
    ) -> Self {
        Self {
            ambient_color,
            diffuse_color,
            specular_color,
            specular_exponent,
            reflection_color,
            refraction_color,
        }
    }
}
    
// pub struct Scene {
//     pub camera: Camera,
//     pub light: Light,
//     pub objects: Vec<dyn Object>,
// }

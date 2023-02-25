use nalgebra::{DMatrix, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    screen_origin: Vector3<f32>,
    pub image: DMatrix<f32>,
    pub alpha: DMatrix<f32>,
    width: usize,
    height: usize,
}

impl Camera {
    pub fn new(position: Vector3<f32>, width: usize, height: usize) -> Self {
        Self {
            position: position,
            width: width,
            height: height,
            screen_origin: Vector3::new(-15., 10., 1.),
            image: DMatrix::zeros(width, height),
            alpha: DMatrix::zeros(width, height),
        }
    }

    pub fn ray(&self, i: usize, j: usize) -> Ray {
        let screen_point = self.screen_origin
            + Vector3::new(
                i as f32 * 30. / self.width as f32,
                j as f32 * 20. / self.height as f32 * -1., // y displacement is negative
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

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
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

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<Vector3<f32>> {
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

    pub fn normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        let normal = (point - self.center).normalize();
        return normal;
    }
}

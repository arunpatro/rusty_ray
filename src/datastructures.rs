use nalgebra::Vector3;

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
    pub object: Option<Box<dyn Object>>,
}

impl AABBNode {
    pub fn new(
        bbox: AlignedBox3d,
        left: Option<Box<AABBNode>>,
        right: Option<Box<AABBNode>>,
        object: Option<Box<dyn Object>>,
    ) -> Self {
        Self {
            bbox,
            left,
            right,
            object,
        }
    }
}

pub struct bvh {
    pub root: Option<Box<AABBNode>>,
}

impl bvh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self {
            root: recur_tree(triangles),
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<(f32, Vector3<f32>, Vector3<f32>)> {
        let mut t = f32::MAX;
        let mut normal = Vector3::new(0., 0., 0.);
        let mut point = Vector3::new(0., 0., 0.);

        let mut stack = Vec::new();
        stack.push(self.root);

        while stack.len() > 0 {
            let node = stack.pop().unwrap();
            if node.bbox.intersects(ray) {
                if node.object.is_some() {
                    let object = node.object.unwrap();
                    let (t_temp, normal_temp, point_temp) = object.intersects(ray);
                    if t_temp < t {
                        t = t_temp;
                        normal = normal_temp;
                        point = point_temp;
                    }
                } else {
                    stack.push(node.left);
                    stack.push(node.right);
                }
            }
        }

        if t == f32::MAX {
            None
        } else {
            Some((t, normal, point))
        }
    }
}

fn recur_tree(triangles: Vec<Triangle>) {
    if triangles.len() == 0 {
        return;
    }

    let mut bbox = AlignedBox3d::default();
    for triangle in triangles {
        bbox.extend_triangle(&triangle);
    }

    // if leaf node
    if triangles.len() == 1 {
        Box::new(AABBNode::new(
            bbox,
            None,
            None,
            Some(Box::new(triangles[0])),
        ));
    }

    // if not leaf node
    let mut left = Vec::new();
    let mut right = Vec::new();

    let diag = bbox.max - bbox.min;
    let longest_axis = diag.x.max(diag.y.max(diag.z));

    if longest_axis == diag.x {
        triangles.sort_by(|a, b| a.point1.x.partial_cmp(&b.point1.x).unwrap());
    } else if longest_axis == diag.y {
        triangles.sort_by(|a, b| a.point1.y.partial_cmp(&b.point1.y).unwrap());
    } else {
        triangles.sort_by(|a, b| a.point1.z.partial_cmp(&b.point1.z).unwrap());
    }

    let mid = triangles.len() / 2;
    for i in 0..mid {
        left.push(triangles[i]);
    }
    for i in mid..triangles.len() {
        right.push(triangles[i]);
    }

    let left_node = recur_tree(left);
    let right_node = recur_tree(right);

    Box::new(AABBNode::new(bbox, Some(left_node), Some(right_node), None));
}

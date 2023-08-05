use crate::{vector3::{Vector3, Point3}, ray3::Ray3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;// 焦距，也就是相机距离viewport的距离

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        // viewport 的左下角
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray3 {
        Ray3::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}
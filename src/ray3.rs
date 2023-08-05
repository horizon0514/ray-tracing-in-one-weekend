use crate::vector3::{Point3, Vector3, Color};

pub struct Ray3 {
    origin: Point3,
    direction: Vector3,
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vector3) -> Ray3 {
        Ray3 {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn ray_color(ray: &Ray3) -> Color {
    let unit_direction = ray.direction.unit_vector();

    let white_color = Color::new(1.0, 1.0, 1.0);
    let blue_color = Color::new(0.5, 0.7, 1.0);

    // 此处使用y轴的方向，越往上越白，越往下越蓝。总体模拟天空的颜色
    // unit_direction.y()的值域是-1到1
    // unit_direction.y() + 1.0 的值域是0到2
    // 0.5 * (unit_direction.y() + 1.0) 的值域就是0到1
    let t = 0.5 * (unit_direction.y + 1.0);

    white_color*(1.0 -t) + blue_color*t
}
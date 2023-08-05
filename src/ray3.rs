use crate::{vector3::{Point3, Vector3, Color}, sphere::Sphere};

pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vector3,
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

pub fn ray_color(ray: &Ray3, sphere: &Sphere) -> Color {
    // 如果射线命中了球,就返回球的颜色
    let parameter = sphere.hit_parameter(ray);
    if parameter > 0.0 {
        let normal = (ray.at(parameter) - sphere.center).unit_vector();
        // normal 的xyz在[-1,1]， 所以需要+1并且/2 映射到[0,1]
        return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
    }

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
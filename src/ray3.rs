use crate::{vector3::{Point3, Vector3, Color}, sphere::Sphere, hittable_list::HittableList, hittable::Hittable};

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

pub fn ray_color(ray: &Ray3, world: &HittableList, max_depth: u32) -> Color {
    // 如果超过最大深度则返回黑色
    if max_depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.0001, f32::INFINITY) {
        // 如果命中，则继续构造射线，并且递归计算颜色
        // 以rec.point 为单位球球心，以rec.normal 为单位球法线，构造一个半球体
        // 并随机选择一个单位向量
        let target = rec.point + Sphere::random_in_hemisphere(&rec.normal);
        // 继续构造反射的ray,并递归计算颜色
        let r = ray_color(&Ray3::new(rec.point, target - rec.point), world, max_depth - 1);
        return r * 0.5; // 光线强度衰减50%。在光追中,颜色值代表了光线的强度，而更高的颜色值,代表更强的光线强度
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
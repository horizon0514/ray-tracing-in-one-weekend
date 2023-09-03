use std::fs::File;
use std::io::Write;
use crate::{vector3::{Vector3, Point3, Color}, ray3::Ray3, util::{degree_to_radians, self}, hittable_list::HittableList, hittable::Hittable, color::write_color};

pub struct Camera {
    aspect_ratio: f32,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, v_fov: f32, look_from: Point3, look_at: Point3, v_up: Vector3) -> Camera {
        // let v_fov = 90.0; // the vertical field-of-view angle
        let focal_length = (look_at - look_from).length();// 焦距，也就是相机距离viewport的距离

        // viewport_height
        let tan_theta = (degree_to_radians(v_fov) / 2.0).tan();
        let viewport_height = 2.0 * tan_theta * focal_length;
        let viewport_width = aspect_ratio * viewport_height;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u).unit_vector();


        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        // viewport 的左下角
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focal_length;

        Camera {
            aspect_ratio,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn render(&self, file: &mut File, world: &HittableList) {

        let sample_count = 100;
        let max_depth = 50;

        let image_width = 400;
        let image_height = (image_width as f32 / self.aspect_ratio) as i32;
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", image_width, image_height).unwrap();
        writeln!(file, "255").unwrap();
        // write color 
        for  y in (0..image_height).rev() {

            for x in 0..image_width {
                let mut color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..sample_count {
                    let u = ((x as f32) + util::random_double()) / (image_width-1) as f32;
                    let v = (y as f32 + util::random_double()) / (image_height-1) as f32;
                    let ray = self.get_ray(u, v);

                    color += self.ray_color(&ray, world, max_depth);
                }
                write_color(file, color, sample_count);
            }
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray3 {
        Ray3::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }

    pub fn ray_color(&self, ray: &Ray3, world: &HittableList, max_depth: u32) -> Color {
        // 如果超过最大深度则返回黑色
        if max_depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
    
        if let Some(rec) = world.hit(ray, 0.0001, f32::INFINITY) {
            if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
                return attenuation * self.ray_color(&scattered, world, max_depth-1);
            }
            return Color::new(0.0, 0.0, 0.0);
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
}
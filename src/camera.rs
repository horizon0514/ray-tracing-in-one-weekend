use std::fs::File;
use std::io::Write;
use crate::{vector3::{Vector3, Point3, Color}, ray3::Ray3, util::{degree_to_radians, self, write_color, random_in_unit_disk}, hittable_list::HittableList, hittable::Hittable};

pub struct Camera {
    aspect_ratio: f32,
    origin: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel_center_lower_left_corner: Point3,
    defocus_angle: f32,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
    focus_distance: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, v_fov: f32, look_from: Point3, look_at: Point3, v_up: Vector3, defocus_angle: f32, focus_distance: f32, image_width: f32) -> Camera {
        let image_height = image_width / aspect_ratio;

        // let v_fov = 90.0; // the vertical field-of-view angle
        let focal_length = (look_at - look_from).length();// 焦距，也就是相机距离viewport的距离

        // viewport_height
        let tan_theta = (degree_to_radians(v_fov) / 2.0).tan();
        let viewport_height = 2.0 * tan_theta * focus_distance;
        let viewport_width = aspect_ratio * viewport_height;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u).unit_vector();


        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let pixel_delta_u = horizontal / image_width;
        let pixel_delta_v = vertical / image_height;
        // viewport 的左下角
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;
        let pixel_center_lower_left_corner = lower_left_corner + (pixel_delta_u + pixel_delta_v) * 0.5;
        
        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = defocus_angle / 180.0 * std::f32::consts::PI;
        let defocus_tan_theta = (defocus_radius/2.0).tan();
        let defocus_disk_u = u * defocus_tan_theta * focus_distance;
        let defocus_disk_v = v * defocus_tan_theta * focus_distance;
        Camera {
            aspect_ratio,
            origin,
            pixel_delta_u,
            pixel_delta_v,
            pixel_center_lower_left_corner,
            focus_distance,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
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
                    let u = ((x as f32) + util::random_double());
                    let v = (y as f32 + util::random_double());
                    let ray = self.get_ray(u, v);

                    color += self.ray_color(&ray, world, max_depth);
                }
                write_color(file, color, sample_count);
            }
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray3 {
        let pixel_center = self.pixel_center_lower_left_corner + self.pixel_delta_u * u + self.pixel_delta_v * v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let origin = if self.defocus_angle<=0.0 {
            self.origin
        } else {
            self.origin + self.defocus_disk_sample()
        };
        Ray3::new(origin, pixel_sample - origin)
    }

    // return a random square vector on the pixel
    fn pixel_sample_square(&self) -> Vector3 {
        let px = util::random_double() * 0.5;
        let py = util::random_double() * 0.5;

        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let center = random_in_unit_disk();

        self.defocus_disk_u * center.x +  self.defocus_disk_v * center.y
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
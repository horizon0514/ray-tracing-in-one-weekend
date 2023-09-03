use std::{fs, ops::AddAssign};
use std::io::Write;

mod vector3;
use vector3::{Color, Vector3, Point3};
mod color;
use color::write_color;

use crate::material::{Lambertian, Metal, Dielectric};
use crate::{ray3::{Ray3, ray_color}, util::random_double};
mod ray3;

mod sphere;
use sphere::Sphere;

mod hittable;
use hittable::Hittable;

mod hittable_list;
use hittable_list::HittableList;

mod camera;
use camera::Camera;

mod util;

mod material;
use material::Material;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    println!("image_width: {}, image_height: {}", image_width, image_height);

    // Camera, 位置在原点,朝向为负Z轴
    let camera = Camera::new(aspect_ratio);

    // Material
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Dielectric {
        ir: 1.5,
    };
    let material_left2 = Dielectric {
        ir: 1.5,
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };
    
    // World
    let mut world = HittableList::new();
    let sphere = Sphere::new(Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, material_center);
    let ground = Sphere::new(Vector3 { x: 0.0, y: -100.5, z: -1.0 }, 100.0, material_ground);
    let left= Sphere::new(Vector3 { x: -1.0, y: 0.0, z: -1.0 }, 0.5, material_left);
    let right = Sphere::new(Vector3 { x: 1.0, y: 0.0, z: -1.0 }, 0.5, material_right);
    let left2 = Sphere::new(Vector3 { x: -1.0, y: 0.0, z: -1.0 }, -0.4, material_left2);
    world.add(Box::new(sphere));
    world.add(Box::new(ground));
    world.add(Box::new(left));
    world.add(Box::new(right));
    world.add(Box::new(left2));
    // Render

    let file_name = "image.ppm";
    let mut file = fs::File::create(file_name).unwrap();
    // write ppm file header
    writeln!(&file, "P3").unwrap();
    writeln!(&file, "{} {}", image_width, image_height).unwrap();
    writeln!(&file, "255").unwrap();


    let sample_count = 100;
    let max_depth = 50;
    // write color 
    for  y in (0..image_height).rev() {

        for x in 0..image_width {
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..sample_count {
                let u = ((x as f32) + util::random_double()) / (image_width-1) as f32;
                let v = (y as f32 + util::random_double()) / (image_height-1) as f32;
                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &world, max_depth);
            }
            write_color(&mut file, color, sample_count);
        }
    }
}

use std::{fs, ops::AddAssign};
use std::io::Write;

mod vector3;
use vector3::{Color, Vector3, Point3};
mod color;
use color::write_color;

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

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    println!("image_width: {}, image_height: {}", image_width, image_height);

    // Camera, 位置在原点,朝向为负Z轴
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();
    let sphere = Sphere::new(Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5);
    let ground = Sphere::new(Vector3 { x: 0.0, y: -100.5, z: -1.0 }, 100.0);
    world.add(Box::new(sphere));
    world.add(Box::new(ground));
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
        print!("\rProgress: [{}%]", (image_height-y)/image_height*100); 

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

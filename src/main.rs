use std::fs;
use std::io::Write;

mod vector3;
use vector3::{Color, Vector3, Point3};
mod color;
use color::write_color;

use crate::ray3::{Ray3, ray_color};
mod ray3;

mod sphere;
use sphere::Sphere;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    println!("image_width: {}, image_height: {}", image_width, image_height);

    // Camera, 位置在原点,朝向为负Z轴
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;// 焦距，也就是相机距离viewport的距离

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    // viewport 的左下角
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Render
    let sphere = Sphere::new(Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5);

    let file_name = "image.ppm";

    let mut file = fs::File::create(file_name).unwrap();

    // write ppm file header
    writeln!(&file, "P3").unwrap();
    writeln!(&file, "{} {}", image_width, image_height).unwrap();
    writeln!(&file, "255").unwrap();


    // write color 
    for  y in (0..image_height).rev() {
        print!("\rProgress: [{}%]", (image_height-y)/image_height*100); 

        for x in 0..image_width {
            // let r = x as f32 / image_width as f32;
            // let g = y as f32 / image_height as f32;
            // let b = 0.25;
            // let color = Color::new(r, g, b);
            // write_color(&mut file, color);
            let u = x as f32 / (image_width-1) as f32;
            let v = y as f32 / (image_height-1) as f32;
            let ray = Ray3::new(origin, lower_left_corner + horizontal*u + vertical*v);

            write_color(&mut file, ray_color(&ray, &sphere));
        }
    }
}

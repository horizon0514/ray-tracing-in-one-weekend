use std::fs;

mod vector3;
use vector3::{Color, Vector3, Point3};

use crate::material::{Lambertian, Metal, Dielectric};
mod ray3;

mod sphere;
use sphere::Sphere;

mod hittable;

mod hittable_list;
use hittable_list::HittableList;

mod camera;
use camera::Camera;

mod util;
mod material;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400_f32;

    // Camera, 位置在原点,朝向为负Z轴
    let camera = Camera::new(
        aspect_ratio, 
        20.0,
        Point3::new(-2.0, 2.0,1.0), 
        Point3::new(0.0, 0.0, -1.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        10.0,
        3.4,
        image_width
    );

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
        fuzz: 0.0
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
    // write: Sphere<Lambertian> ppm file header
    

    camera.render(&mut file, &world);
}

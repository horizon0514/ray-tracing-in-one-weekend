use std::fs;
use std::io::Write;

mod vector3;
use vector3::Color;
mod color;
use color::write_color;

fn main() {
    let image_width = 256;
    let image_height = 256;

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
            let r = x as f32 / image_width as f32;
            let g = y as f32 / image_height as f32;
            let b = 0.25;
            let color = Color::new(r, g, b);
            write_color(&mut file, color);
        }
    }
}

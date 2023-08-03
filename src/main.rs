use std::fs;
use std::io::Write;
fn main() {
    println!("Hello, world!");
    let image_width = 256;
    let image_height = 256;

    let file_name = "image.ppm";

    let file = fs::File::create(file_name).unwrap();

    // std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";
    writeln!(&file, "P3").unwrap();
    writeln!(&file, "{} {}", image_width, image_height).unwrap();
    writeln!(&file, "255").unwrap();
    for  y in (0..image_height).rev() {
        print!("\rProgress: [{}%]", (image_height-y)/image_height*100); 

        for x in 0..image_width {
            // auto r = double(i) / (image_width-1);
            // auto g = double(j) / (image_height-1);
            // auto b = 0.25;

            // int ir = static_cast<int>(255.999 * r);
            // int ig = static_cast<int>(255.999 * g);
            // int ib = static_cast<int>(255.999 * b);
            let r = x as f64 / image_width as f64;
            let g = y as f64 / image_height as f64;
            let b = 0.25;
            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            writeln!(&file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}

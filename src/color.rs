use std::fs::File;
use std::io::Write;

use crate::vector3::Color;

pub fn write_color(file: &mut File, color: Color) {
    // color value is in [0, 1]
    writeln!(file, "{} {} {}", 255.999 * color.x, 255.999 * color.y, 255.999 * color.z).unwrap();
}
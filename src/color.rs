use std::fs::File;
use std::io::Write;

use crate::util::clamp;
use crate::vector3::Color;

pub fn write_color(file: &mut File, mut color: Color, sample_count: u32) {
    // color value is in [0, 1]
    color = color / (sample_count as f32);
    let r = clamp(color.x, 0.0, 0.999);
    let g = clamp(color.y, 0.0, 0.999);
    let b = clamp(color.z, 0.0, 0.999);
    writeln!(file, "{} {} {}", 256.0 * r, 256.0 * g, 256.0 * b).unwrap();
}
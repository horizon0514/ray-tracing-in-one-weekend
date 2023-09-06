use std::fs::File;
use std::io::Write;

use rand::Rng;

use crate::vector3::{Vector3, Color};

pub fn write_color(file: &mut File, mut color: Color, sample_count: u32) {
    // color value is in [0, 1]
    color = color / (sample_count as f32);
    let r = clamp(color.x.sqrt(), 0.0, 0.999);
    let g = clamp(color.y.sqrt(), 0.0, 0.999);
    let b = clamp(color.z.sqrt(), 0.0, 0.999);
    writeln!(file, "{} {} {}", 256.0 * r, 256.0 * g, 256.0 * b).unwrap();
}


pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() // generates a float between 0 and 1
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max) // generates a float between min and max
}

/**
 * 生成一个随机单位向量
 */
pub fn random_unit_vector() -> Vector3 {
    loop {
        let v = Vector3::random_in_range(-1.0, 1.0);
        if v.length_squared() >= 1.0 {
            continue;
        }

        return v.unit_vector();
    }
}

/**
 * generate a random point in the unit circle disk
 */
pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

/*
 * 求反射后的向量
 * @param v: 原向量
 * @param n: 反射点的法线
 * @return 反射后的向量
*/
pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n * 2.0 * v.dot(n)
}

/**
 * 折射
 * 𝐑′⊥=𝜂𝜂′(𝐑+(−𝐑⋅𝐧)𝐧)
 */
pub fn refract(v: Vector3, n: Vector3, ni_over_nt: f32) -> Vector3 {
    let uv = v.unit_vector();

    let cos_theta = f32::min(-uv.dot(n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * ni_over_nt;
    let r_out_parallel = n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt()) ;
    r_out_perp + r_out_parallel
}

pub fn degree_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
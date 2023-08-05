use rand::Rng;


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
    rng.gen::<f32>()
}
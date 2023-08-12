use crate::{hittable::HitRecord, ray3::Ray3, vector3::Vector3};
use std::option::Option;
pub trait Material {
    fn scatter(&self, _ray: &Ray3, _rec: &HitRecord<Self>) -> Option<Scattered>;
}

pub struct Scattered {
    pub attenuation: Vector3, // 反射率
    pub scattered: Ray3, // 反射出的光线
}


pub struct Lambertian {
    pub albedo: Vector3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray3, rec: &HitRecord<Lambertian>) -> Option<Scattered> {
        let mut scattered_direction = rec.normal + Vector3::random_unit_vector();
        if scattered_direction.near_zero() {
            scattered_direction = rec.normal;
        }

        let scattered_ray = Ray3 {
            origin: rec.point,
            direction: scattered_direction,
        };
        Some(Scattered {
            attenuation: self.albedo,
            scattered: scattered_ray,
        })
    }
}

pub struct Metal {
    pub albedo: Vector3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray3, rec: &HitRecord<Metal>) -> Option<Scattered> {
        let mut scattered_direction = reflect(ray.direction, rec.normal);

        Some(Scattered {
            attenuation: self.albedo,
            scattered: Ray3 {
                origin: rec.point,
                direction: scattered_direction,
            },
        })
    }
}


fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n* 2.0 * v.dot(n)
}
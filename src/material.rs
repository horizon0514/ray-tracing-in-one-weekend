use crate::hittable::{Hittable, HitRecord};
use crate::ray3::Ray3;
use crate::sphere::Sphere;
use crate::util::{random_unit_vector, reflect, refract};
use crate::vector3::{Color, Vector3 };


pub trait Material {
    fn scatter(&self, ray: &Ray3, rec: &HitRecord) -> Option<(Color, Ray3)>;
}


/**
 * 完全漫反射材质
 */
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray3, rec: &HitRecord) -> Option<(Color, Ray3)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray3::new(rec.point, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

/**
 * 镜面材质
 */
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray3, rec: &HitRecord) -> Option<(Color, Ray3)> {
        let reflected = reflect(ray.direction.unit_vector(), rec.normal);
        let scattered = Ray3::new(rec.point, reflected + random_unit_vector() * self.fuzz);
        let attenuation = self.albedo;
        
        Some((attenuation, scattered))
    }
}
    

/**
 * 绝缘体材质
 */
pub struct  Dielectric {
    pub ir: f32, // Index of Refraction 折射系数
}

impl Material for  Dielectric {
    fn scatter(&self, _ray: &Ray3, _rec: &HitRecord) -> Option<(Color, Ray3)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if _rec.is_front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = _ray.direction.unit_vector();

        let refracted = refract(unit_direction, _rec.normal, refraction_ratio);

        Some((attenuation, Ray3::new(_rec.point, refracted)))
    }
}
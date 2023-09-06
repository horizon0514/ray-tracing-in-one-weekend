use crate::hittable::HitRecord;
use crate::ray3::Ray3;
use crate::util::{random_unit_vector, reflect, refract, self, random_double};
use crate::vector3::Color;


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
        
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
    

/**
 * 绝缘体材质
 */
pub struct  Dielectric {
    pub ir: f32, // Index of Refraction 折射系数
}

impl Dielectric {
    pub fn reflectance(cos_theta: f32, index_of_refraction: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - index_of_refraction) / (1.0 + index_of_refraction)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for  Dielectric {
    fn scatter(&self, _ray: &Ray3, _rec: &HitRecord) -> Option<(Color, Ray3)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        // if the side is front face, it means the ray is from outside(Air) to inside
        // so the refraction ratio should be Air_refraction / glass_refraction
        // it's 1.0 / self.ir
        // on the contrary , if the side is back face, it means the ray is from inside(Air) to outside
        // so the refraction ratio should be glass_refraction / Air_refraction
        // it's self.ir / 1.0
        let refraction_ratio = if _rec.is_front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = _ray.direction.unit_vector();

        let cos_theta = f32::min(-unit_direction.dot(_rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // if refraction_ratio * sin_theta > 1.0, it means the ray cannot refract
        // so the ray is reflected
        let direction = if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
            reflect(unit_direction, _rec.normal)
        } else {
            refract(unit_direction, _rec.normal, refraction_ratio)
        };

        Some((attenuation, Ray3::new(_rec.point, direction)))
    }
}
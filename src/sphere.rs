use crate::material::Material;
use crate::util::random_unit_vector;
use crate::vector3::Vector3;
use crate::{vector3::Point3, ray3::Ray3};
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f32, material: M) -> Sphere<M> {
        Sphere { center, radius, material }
    }

    /**
     * 以原点为球心，在半单位球体内随机返回一个单位向量
     * @param point
     * @param normal 半球体的法线
     */
    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = random_unit_vector();
        let cos_theta = in_unit_sphere.dot(*normal);
        if cos_theta > 1.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * (-1.0)
        }

    } 

    pub fn is_hit(&self, ray: &Ray3) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        true
    }

    pub fn hit_parameter(&self, ray: &Ray3) -> f32 {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return -1.0;
        }

        (-half_b - discriminant.sqrt()) / a
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray3, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut t = (-half_b - discriminant.sqrt()) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discriminant.sqrt()) / a;
        }

        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let mut normal = (point - self.center).unit_vector();
        let is_front_face = ray.direction.dot(normal) < 0.0;

        if !is_front_face {
            normal = normal * (-1.0);
        }

        Some(HitRecord {
            t,
            point,
            normal,
            is_front_face,
            material: &self.material
        })
    }
}
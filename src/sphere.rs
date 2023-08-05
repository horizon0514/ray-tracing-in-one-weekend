use crate::{vector3::Point3, ray3::Ray3};
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
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

impl Hittable for Sphere {
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
            is_front_face
        })
    }
}
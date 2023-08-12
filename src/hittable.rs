use crate::vector3::{Point3, Vector3};
use crate::ray3::Ray3;
/**
 * 相交记录
 */
#[derive(Debug)]
pub struct HitRecord<M> {
    pub t: f32,
    pub point: Point3,
    pub normal: Vector3,
    pub is_front_face: bool,
    pub material: M,
}

pub trait Hittable<M> {
    fn hit(&self, ray: &Ray3, t_min: f32, t_max: f32) -> Option<HitRecord<M>>;
}
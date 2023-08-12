use crate::{hittable::{Hittable, HitRecord}, ray3::Ray3, material::Material};

pub struct HittableList<M> {
    pub objects: Vec<Box<dyn Hittable<M>>>,
}

impl <M> HittableList<M> {
    pub fn new() -> HittableList<M> {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable<M>>) {
        self.objects.push(object);
    }
}

impl<M> Hittable<M> for HittableList<M> {
    fn hit(&self, ray: &Ray3, t_min: f32, t_max: f32) -> Option<HitRecord<M>> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}
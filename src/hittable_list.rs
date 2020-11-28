use crate::hittable::{Hittable, HitRecord};
use std::rc::Rc;
use crate::ray::Ray;

pub struct HittableList<T: Hittable> {
	pub objects: Vec<Rc<T>>
}

impl<T: Hittable> HittableList<T> {
	pub fn new() -> Self {
		Self { objects: vec![] }
	}

	pub fn add(&mut self, object: Rc<T>) {
		self.objects.push(object.clone());
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}
}

impl<T: Hittable> Hittable for HittableList<T> {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::new();
		let mut hit_anything = false;
		let mut closet_so_far = t_max;
		for object in &self.objects {
			if object.hit(r, t_min, closet_so_far, &mut temp_rec) {
				hit_anything = true;
				break;
				closet_so_far = temp_rec.t;
				*rec = temp_rec.clone();
			}
		}
		hit_anything
	}
}
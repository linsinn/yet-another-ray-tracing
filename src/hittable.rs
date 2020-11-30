use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;
use crate::material::Material;
use std::rc::Rc;


#[derive(Clone)]
pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
	pub mat_ptr: Option<Rc<dyn Material>>
}

impl HitRecord {
	pub fn new() -> Self {
		Self {
			p: Point3::new(0, 0, 0),
			normal: Vec3::new(0, 0, 0),
			t: 0.0,
			front_face: false,
			mat_ptr: None,
		}
	}

	#[inline]
	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
		self.front_face = dot(&r.direction(), outward_normal) < 0.0;
		self.normal = if self.front_face {
			*outward_normal
		} else {
			*outward_normal
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
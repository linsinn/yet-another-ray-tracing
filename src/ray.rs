use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
	origin: Point3,
	dir: Vec3,
}

impl Ray {
	pub fn new(origin: Point3, dir: Vec3) -> Self {
		Self { origin, dir }
	}

	pub fn origin(&self) -> Point3 {
		self.origin.clone()
	}

	pub fn direction(&self) -> Vec3 {
		self.dir.clone()
	}

	pub fn at(&self, t: f64) -> Point3 {
		self.origin + self.dir * t
	}
}
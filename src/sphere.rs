use crate::vec3::{Point3, dot};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
	pub center: Point3,
	pub radius: f64,
}

impl Sphere {
	pub fn new<T>(center: Point3, radius: T) -> Self
	where T: Into<f64>
	{
		let radius = radius.into();
		Self { center, radius }
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = dot(oc, r.direction());
		let c = oc.length_squared() - self.radius * self.radius;
		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return false;
		}
		let sqrt = discriminant.sqrt();

		// Find the nearest root that lies in the acceptable range.
		let mut root = (-half_b - sqrt) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrt) / a;
			if root < t_min || t_max < root {
				return false;
			}
		}
		rec.t = root;
		rec.p = r.at(root);
		let outward_normal = (rec.p - self.center) / self.radius;
		rec.set_face_normal(r, &outward_normal);
		true
	}
}
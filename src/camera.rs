use crate::vec3::{Point3, Vec3, unit_vector, cross};
use std::env::VarError;
use crate::ray::Ray;
use crate::utils::degrees_to_radians;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pub origin: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
}

impl Camera {
	pub fn new(
		look_from: Point3,
		look_at: Point3,
		v_up: Vec3,
		v_fov: f64, // vertical field-of-view in degrees
		aspect_ratio: f64
	) -> Self {
		let theta = degrees_to_radians(v_fov);
		let h = (theta / 2.0).tan();

		let viewport_height = 2.0 * h;
		let viewport_width = viewport_height * aspect_ratio;

		let w = unit_vector(look_from - look_at);
		let u = unit_vector(cross(&v_up, &w));
		let v = cross(&w, &u);

		let origin = look_from;
		let horizontal = viewport_width * u;
		let vertical = viewport_height * v;
		let lower_left_corner =  origin - horizontal / 2.0 - vertical / 2.0 - w;
		Self {
			origin,
			lower_left_corner,
			horizontal,
			vertical,
		}
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
	}
}
use crate::vec3::{Point3, Vec3, unit_vector, cross, random_in_unit_disk};
use crate::ray::Ray;
use crate::utils::degrees_to_radians;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pub origin: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub lens_radius: f64,
	pub w: Vec3,
	pub u: Vec3,
	pub v: Vec3,
}

impl Camera {
	pub fn new(
		look_from: Point3,
		look_at: Point3,
		v_up: Vec3,
		v_fov: f64, // vertical field-of-view in degrees
		aspect_ratio: f64,
		aperture: f64,
		focus_dist: f64,
	) -> Self {
		let theta = degrees_to_radians(v_fov);
		let h = (theta / 2.0).tan();

		let viewport_height = 2.0 * h;
		let viewport_width = viewport_height * aspect_ratio;

		let w = unit_vector(look_from - look_at);
		let u = unit_vector(cross(&v_up, &w));
		let v = cross(&w, &u);

		let origin = look_from;
		let horizontal = focus_dist * viewport_width * u;
		let vertical = focus_dist * viewport_height * v;
		let lower_left_corner =  origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
		let lens_radius = aperture / 2.0;
		Self {
			origin,
			lower_left_corner,
			horizontal,
			vertical,
			lens_radius,
			w, u, v,
		}
	}

	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		let rd = self.lens_radius * random_in_unit_disk();
		let offset = self.u * rd.x() + self.v * rd.y();

		Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
	}
}
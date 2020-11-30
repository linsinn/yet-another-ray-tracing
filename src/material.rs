use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color, random_unit_vector, reflect, unit_vector, dot, random_in_unit_sphere};

pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
	pub albedo: Color
}

impl Lambertian {
	pub fn new(col: Color) -> Self {
		Self {
			albedo: col
		}
	}
}

impl Material for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let mut scatter_direction = rec.normal + random_unit_vector();

		if scatter_direction.near_zero() {
			scatter_direction = rec.normal
		}

		*scattered = Ray::new(rec.p, scatter_direction);
		*attenuation = self.albedo;
		true
	}
}

pub struct Metal {
	pub albedo: Color,
	pub fuzz: f64,
}

impl Metal {
	pub fn new(col: Color, f: f64) -> Self {
		Self {
			albedo: col,
			fuzz: f.min(1.0),
		}
	}
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
		*scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
		*attenuation = self.albedo;
		dot(&scattered.direction(), &rec.normal) > 0.0
	}
}

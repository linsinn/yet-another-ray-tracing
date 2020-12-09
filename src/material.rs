use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color, random_unit_vector, reflect, unit_vector, dot, random_in_unit_sphere, refract};
use crate::utils::random_double;

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
	fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
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

pub struct Dielectric {
	pub ir: f64
}

impl Dielectric {
	pub fn new(index_of_refraction: f64) -> Self {
		Self {
			ir: index_of_refraction
		}
	}

	fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
	}
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		*attenuation = Color::new(1, 1, 1);
		let refraction_ratio = if rec.front_face {1.0 / self.ir} else {self.ir};
		let unit_direction = unit_vector(r_in.direction());

		let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

		let cannot_refract = refraction_ratio * sin_theta > 1.0;

		if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() {
			let reflected = reflect(&unit_direction, &rec.normal);
			*scattered = Ray::new(rec.p, reflected);
		} else {
			let refracted = refract(&unit_direction, &rec.normal, refraction_ratio);
			*scattered = Ray::new(rec.p, refracted);
		}
		true
	}
}
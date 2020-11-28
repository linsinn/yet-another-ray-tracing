use crate::vec3::{Color, Vec3, unit_vector, Point3, dot};
use crate::ray::Ray;
use crate::color::write_color;
use crate::hittable::{Hittable, HitRecord};
use crate::utils::INFINITY;
use crate::hittable_list::HittableList;
use std::rc::Rc;
use crate::sphere::Sphere;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
	let mut rec = HitRecord::new();
	if world.hit(r, 0.0, INFINITY, &mut rec) {
		0.5 * (rec.normal + Color::new(1, 1, 1))
	} else {
		let unit_direction = unit_vector(r.direction());
		let t = 0.5 * (unit_direction.y() + 1.0);
		(1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1.0)
	}
}


fn main() {
	// Image
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;

	// World
	let mut world = HittableList::new();
	world.add(Rc::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
	world.add(Rc::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

	// Camera
	let viewport_height = 2.0;
	let viewport_width = viewport_height * aspect_ratio;
	let focal_length = 1.0;
	let origin = Point3::new(0, 0, 0);
	let horizontal = Vec3::new(viewport_width, 0, 0);
	let vertical = Vec3::new(0, viewport_height, 0);
	let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, focal_length);

	// Render
	println!("P3\n{} {}\n255", image_width, image_height);
	for i in (0..image_height).rev() {
		eprint!("Scan lines remaining: {}\n", i);
		for j in 0..image_width {
			let u = j as f64 / (image_width - 1) as f64;
			let v = i as f64 / (image_height - 1) as f64;
			let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical + origin);
			let pixel_color = ray_color(&r, &world);
			write_color(pixel_color);
		}
	}
	eprint!("\nDone.\n");
}

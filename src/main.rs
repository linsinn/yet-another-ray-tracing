use crate::vec3::{Color, Vec3, unit_vector, Point3, dot};
use crate::ray::Ray;
use crate::color::write_color;
use crate::hittable::{Hittable, HitRecord};
use crate::utils::{INFINITY, random_double};
use crate::hittable_list::HittableList;
use std::rc::Rc;
use crate::sphere::Sphere;
use crate::camera::Camera;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod camera;

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
	let samples_per_pixel = 100;

	// World
	let mut world = HittableList::new();
	world.add(Rc::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
	world.add(Rc::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

	// Camera
	let cam = Camera::new();

	// Render
	println!("P3\n{} {}\n255", image_width, image_height);
	for i in (0..image_height).rev() {
		eprint!("Scan lines remaining: {}\r", i);
		for j in 0..image_width {
			let mut pixel_color = Color::default();
			for _ in 0..samples_per_pixel {
				let u = (j as f64 + random_double()) / (image_width - 1) as f64;
				let v = (i as f64 + random_double()) / (image_height - 1) as f64;
				let r = cam.get_ray(u, v);
				pixel_color += ray_color(&r, &world);
			}
			write_color(pixel_color, samples_per_pixel);
		}
	}
	eprint!("\nDone.\n");
}

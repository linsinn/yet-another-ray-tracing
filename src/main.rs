use crate::vec3::{Color, Vec3, unit_vector, Point3, dot, random_unit_vector, random_in_hemisphere};
use crate::ray::Ray;
use crate::color::get_pixel_color;
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

use image;
use std::cmp::max;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
	let mut rec = HitRecord::new();

	if depth <= 0 {
		return Color::default();
	}

	if world.hit(r, 0.001, INFINITY, &mut rec) {
		// let target = rec.p + rec.normal + random_unit_vector();
		let target = rec.p + random_in_hemisphere(&rec.normal);
		0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1)
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
	let max_depth = 50;

	// World
	let mut world = HittableList::new();
	world.add(Rc::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
	world.add(Rc::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

	// Camera
	let cam = Camera::new();

	// Render
	let mut buf = Vec::with_capacity((image_height * image_width * 3) as usize);
	for i in (0..image_height).rev() {
		eprint!("Scan lines remaining: {:03}\r", i);
		for j in 0..image_width {
			let mut pixel_color = Color::default();
			for _ in 0..samples_per_pixel {
				let u = (j as f64 + random_double()) / (image_width - 1) as f64;
				let v = (i as f64 + random_double()) / (image_height - 1) as f64;
				let r = cam.get_ray(u, v);
				pixel_color += ray_color(&r, &world, max_depth);
			}
			let (r, g, b) = get_pixel_color(pixel_color, samples_per_pixel);
			buf.push(r);
			buf.push(g);
			buf.push(b);
		}
	}
	image::save_buffer("image.png", &buf, image_width as u32, image_height as u32, image::ColorType::Rgb8).unwrap();
	eprint!("\nDone.\n");
}

use crate::vec3::{Color, Vec3, unit_vector, Point3, dot, random_unit_vector, random_in_hemisphere};
use crate::ray::Ray;
use crate::color::get_pixel_color;
use crate::hittable::{Hittable, HitRecord};
use crate::utils::{INFINITY, random_double, random_double_range};
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
mod material;

use image;
use std::cmp::max;
use crate::material::{Lambertian, Metal, Dielectric};
use rand::random;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
	let mut rec = HitRecord::new();

	if depth <= 0 {
		return Color::default();
	}

	if world.hit(r, 0.001, INFINITY, &mut rec) {
		let mut scattered = Ray::default();
		let mut attenuation = Color::default();
		if let Some(p) = &rec.mat_ptr {
			let p = p.clone();
			if p.scatter(r, &mut rec, &mut attenuation, &mut scattered) {
				attenuation * ray_color(&scattered, world, depth - 1)
			} else {
				Color::default()
			}
		} else {
			Color::default()
		}
	} else {
		let unit_direction = unit_vector(r.direction());
		let t = 0.5 * (unit_direction.y() + 1.0);
		(1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1.0)
	}
}


fn main() {
	// Image
	let aspect_ratio = 3.0 / 2.0;
	let image_width = 900;
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 500;
	let max_depth = 50;

	// World
	let world = random_scene();

	// Camera
	let look_from = Point3::new(13, 2, 3);
	let look_at = Point3::new(0, 0, 0);
	let v_up = Vec3::new(0, 1, 0);
	let dist_to_focus = 10.0;
	let aperture = 0.1;

	let cam = Camera::new(
		look_from,
		look_at,
		v_up,
		20.0,
		aspect_ratio,
		aperture,
		dist_to_focus
	);

	// Render
	let mut buf = Vec::with_capacity((image_height * image_width * 3) as usize);
	for i in (0..image_height).rev() {
		eprint!("Scan lines remaining: {:3}\r", i);
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

fn random_scene() -> HittableList<Sphere> {
	let mut world = HittableList::new();
	let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
	world.add(Rc::new(Sphere::new(Point3::new(0, -1000, 0), 1000.0, ground_material)));
	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = random_double();
			let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

			if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
				if choose_mat < 0.8 {
					// diffuse
					let albedo = Color::random() * Color::random();
					let sphere_material = Rc::new(Lambertian::new(albedo));
					world.add(Rc::new(Sphere::new(center.clone(), 0.2, sphere_material)));
				} else if choose_mat < 0.95 {
					// metal
					let albedo = Color::random_range(0.5, 1.0);
					let fuzz = random_double_range(0.0, 0.5);
					let sphere_material = Rc::new(Metal::new(albedo, fuzz));
					world.add(Rc::new(Sphere::new(center.clone(), 0.2, sphere_material)));
				} else {
					// glass
					let sphere_material = Rc::new(Dielectric::new(1.5));
					world.add(Rc::new(Sphere::new(center.clone(), 0.2, sphere_material)));
				}
			}
		}
	}
	let material = Rc::new(Dielectric::new(1.5));
	world.add(Rc::new(Sphere::new(Point3::new(0, 1, 0), 1.0, material)));

	let material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
	world.add(Rc::new(Sphere::new(Point3::new(-4, 1, 0), 1.0, material)));

	let material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
	world.add(Rc::new(Sphere::new(Point3::new(4, 1, 0), 1.0, material)));
	world
}
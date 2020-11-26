use crate::vec3::Color;

mod vec3;
mod color;
mod ray;

fn main() {
	let image_width = 256;
	let image_height = 256;

	println!("P3\n{} {}\n255", image_width, image_height);

	for i in (0..image_height).rev() {
		eprint!("\rScan lines remaining: {}", i);
		for j in 0..image_width {
			let color = Color::from_slice(
				&[j as f64 / (image_width - 1) as f64, i as f64 / (image_height - 1) as f64, 0.25]
			);
			color::write_color(color);
		}
	}
}

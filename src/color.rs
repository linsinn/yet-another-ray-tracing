use crate::vec3::Color;
use crate::utils::clamp;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
	let mut r = pixel_color.x();
	let mut g = pixel_color.y();
	let mut b = pixel_color.z();

	// Divide the color by the number of samples
	let scale = 1.0 / samples_per_pixel as f64;
	r *= scale;

	print!("{} {} {}\n",
					 (256.0 * clamp(r, 0.0, 0.9999)) as i32,
					 (256.0 * clamp(g, 0.0, 0.9999)) as i32,
					 (256.0 * clamp(b, 0.0, 0.9999)) as i32,);
}
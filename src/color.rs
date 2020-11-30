use crate::vec3::Color;
use crate::utils::clamp;

pub fn get_pixel_color(pixel_color: Color, samples_per_pixel: u32) -> (u8, u8, u8){
	let mut r = pixel_color.x();
	let mut g = pixel_color.y();
	let mut b = pixel_color.z();

	// Divide the color by the number of samples
	let scale = 1.0 / samples_per_pixel as f64;
	r *= scale;
	g *= scale;
	b *= scale;

	// Write the translated [0, 255] value of each color component
	((256.0 * clamp(r, 0.0, 0.9999)) as u8, (256.0 * clamp(g, 0.0, 0.9999)) as u8, (256.0 * clamp(b, 0.0, 0.9999)) as u8)
}
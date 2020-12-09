use std::ops;
use std::convert::{TryFrom, TryInto};
use crate::utils::{random_double, random_double_range};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
	pub e: [f64; 3]
}

impl Vec3 {
	pub fn new<T, R, S>(x: T, y: R, z: S) -> Self
	where
		T: Into<f64>,
		R: Into<f64>,
		S: Into<f64>,
	{
		let (x, y, z) = (x.into(), y.into(), z.into());
		Self {
			e: [x, y, z]
		}
	}

	pub fn from_slice(arr: &[f64]) -> Self {
		Self {
			e: <[f64; 3]>::try_from(arr).unwrap()
		}
	}

	pub fn x(&self) -> f64 { self.e[0] }
	pub fn y(&self) -> f64 { self.e[1] }
	pub fn z(&self) -> f64 { self.e[2] }

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.e.iter().map(|&v| v * v).sum::<f64>()
	}

	#[inline]
	pub fn random() -> Vec3 {
		Self {
			e: [random_double(), random_double(), random_double()]
		}
	}

	#[inline]
	pub fn random_range(min: f64, max: f64) -> Vec3 {
		Self {
			e: [random_double_range(min, max), random_double_range(min, max), random_double_range(min, max)]
		}
	}

	#[inline]
	pub fn near_zero(&self) -> bool {
		let eps = 1e-8;
		self.e.iter().all(|&v| v < eps)
	}
}

impl ops::Neg for Vec3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			e: self.e.iter().map(|&v| -v).collect::<Vec<f64>>().try_into().unwrap()
		}
	}
}

impl ops::Sub for Vec3 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			e: self.e.iter().zip(rhs.e.iter()).map( | ( & a, &b) | a - b).collect:: < Vec<f64> > ().try_into().unwrap()
		}
	}
}

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		for (i, &v) in rhs.e.iter().enumerate() {
			self.e[i] += v;
		}
	}
}

impl ops::Add for Vec3 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			e: self.e.iter().zip(rhs.e.iter()).map( | ( & a, &b) | a + b).collect:: < Vec<f64> > ().try_into().unwrap()
		}
	}
}

impl ops::MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		for v in self.e.iter_mut() {
			*v *= rhs;
		}
	}
}

impl ops::Mul<Vec3> for Vec3 {
	type Output = Self;
	fn mul(self, rhs: Vec3) -> Self::Output {
		Self {
			e: self.e.iter().zip(rhs.e.iter()).map( | ( & a, &b) | a * b).collect:: < Vec<f64> > ().try_into().unwrap()
		}
	}
}


impl ops::Mul<f64> for Vec3 {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		Self {
			e: self.e.iter().map( |&a| a * rhs).collect:: < Vec<f64> > ().try_into().unwrap()
		}
	}
}

impl ops::Mul<Vec3> for f64 {
	type Output = Vec3;
	fn mul(self, rhs: Vec3) -> Self::Output {
		Self::Output {
			e: rhs.e.iter().map( |&a| a * self).collect:: < Vec<f64> > ().try_into().unwrap()
		}
	}
}

impl ops::DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		for v in self.e.iter_mut() {
			*v /= rhs;
		}
	}
}

impl ops::Div<f64> for Vec3 {
	type Output = Self;
	fn div(self, rhs: f64) -> Self::Output {
		Self {
			e: self.e.iter().map( |&a| a / rhs).collect:: < Vec<f64> > ().try_into().unwrap()
		}
	}
}

#[inline]
pub fn dot(u: &Vec3, &v: &Vec3) -> f64 {
	u.e.iter().zip(v.e.iter()).map(|(&a, &b)| a * b).sum::<f64>()
}

#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	let mut res = Vec3::new(0, 0, 0);
	res.e[0] = u.e[1] * v.e[2] - u.e[2] * v.e[1];
	res.e[1] = u.e[2] * v.e[0] - u.e[0] * v.e[2];
	res.e[2] = u.e[0] * v.e[1] - u.e[1] * v.e[0];
	res
}

#[inline]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	*v - 2.0 * dot(v, n) * *n
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
	v / v.length()
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = dot(&(-*uv), n).min(1.0);
	let r_out_perpendicular = etai_over_etat * (*uv + cos_theta * *n);
	let r_out_parallel = -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt()) * *n;
	r_out_perpendicular + r_out_parallel
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::random_range(-1.0, 1.0);
		if p.length_squared() < 1.0 {
			return p;
		}
	}
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
	unit_vector(random_in_unit_sphere())
}

#[allow(dead_code)]
#[inline]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
	let in_unit_sphere = random_in_unit_sphere();
	if dot(&in_unit_sphere, normal) > 0.0 {
		in_unit_sphere
	} else {
		-in_unit_sphere
	}
}

pub fn random_in_unit_disk() -> Vec3 {
	loop {
		let p = Vec3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0);
		if p.length_squared() < 1.0 {
			return p;
		}
	}
}


pub type Point3 = Vec3;
pub type Color = Vec3;

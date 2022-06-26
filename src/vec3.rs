use std::ops::{Add, Mul, Div, Sub};
use std::cmp::PartialEq;
use super::utility;

#[derive(Debug,Copy,Clone)]
pub struct Vec3{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl PartialEq for Vec3{
	fn eq(&self, other: &Vec3) -> bool{
		(self.x==other.x)&&(self.y==other.y)&&(self.z==other.z)
	}
	
	fn ne(&self, other: &Vec3) -> bool{
		!((self.x==other.x)&&(self.y==other.y)&&(self.z==other.z))
	}
}


impl Add for Vec3{
	type Output = Vec3;
	
	fn add(self, other: Vec3) -> Vec3{
		Vec3 { x:self.x+other.x , y:self.y+other.y, z:self.z+other.z}
	}
}

impl Sub for Vec3{
	type Output = Vec3;
	
	fn sub(self, other: Vec3) -> Vec3{
		Vec3 { x:self.x-other.x , y:self.y-other.y, z:self.z-other.z}
	}
}

impl Mul<f64> for Vec3{
	type Output = Vec3;
	
	fn mul(self, other: f64) -> Vec3{
		Vec3 { x: self.x*other , y: self.y*other, z: self.z*other }
	}
}

impl Div<f64> for Vec3{
	type Output = Vec3;
	
	fn div(self, other: f64) -> Vec3{
		Vec3 { x: self.x/other, y: self.y/other, z: self.z/other }
	}
}


impl Vec3 {
	pub fn length_squared(&self) -> f64{
		self.x*self.x + self.y*self.y + self.z*self.z
	}

	pub fn length(&self) -> f64{
		self.length_squared().sqrt()
	}
	
	pub fn dot(&self, other: Vec3) -> f64{
		self.x*other.x + self.y*other.y + self.z*other.z
	}
	
	pub fn cross(&self, other: Vec3) -> Vec3{
		Vec3 { x: self.y*other.z - self.z*other.y, y: self.z*other.x - self.x*other.z, z: self.x*other.y - self.y*self.x}
	}
	
	pub fn unit_vector(&self) -> Vec3{
		let new_vector = Vec3 { x:self.x, y:self.y, z:self.z};
		new_vector / self.length()
	}
	pub fn to_color_string(&self, samples_per_pixel: i16) -> String{
		let mut r =  self.x;
		let mut g = self.y;
		let mut b = self.z;

		let scale = 1.0/(samples_per_pixel as f64);
		
		r *= scale;
		g *= scale;
		b *= scale;



		format!("{} {} {}", (256.0 * utility::clamp(r, 0.0, 0.999)) as i16, (256.0 * utility::clamp(g, 0.0, 0.999)) as i16, (256.0 * utility::clamp(b, 0.0, 0.999)) as i16)
	}
}


pub fn random_vec() -> Vec3{
	Vec3{x: utility::randomf64(), y: utility::randomf64(), z: utility::randomf64()}
}

pub fn random_vec_range(min: f64, max: f64) -> Vec3{
	Vec3{x: utility::randomf64range(min, max), y:utility::randomf64range(min, max), z: utility::randomf64range(min, max)}
}

pub fn random_in_unit_sphere() -> Vec3{
	let mut p = random_vec_range(-1.0, 1.0);
	let mut c = 0;
	while c==0 {
		p = random_vec_range(-1.0, 1.0);
		if  p.length_squared()<=1.0 {
			c = 1;
		}
	}
	p
}

#[cfg(test)]
mod tests{

    use super::Vec3;

    #[test]
	fn get_x() {
		let tv = Vec3{x:1.2,y:1.4,z:1.5};
		assert_eq!(1.2,tv.x);
	}

	#[test]
	fn add_2_vecs() {
		let v1 = Vec3 {x:1.0, y:3.0, z:4.0};
		let v2 = Vec3 {x:2.0, y:2.0, z:1.0};
		
		let vo = Vec3 {x:3.0, y:5.0, z:5.0};
		
		let vc = v1+v2;
		
		assert_eq!(vc.x,vo.x);
		assert_eq!(vc.y,vo.y);
		assert_eq!(vc.z,vo.z);
		assert_eq!(vc,vo);
	}
	
	#[test]
	fn mul_vec_by_scalar() {
		let scalar = 2.0;
		let vec = Vec3 {x:1.5, y:2.0, z:5.0};
		
		let outvec = Vec3 {x:3.0, y:4.0, z:10.0};
		
		assert_eq!(vec*scalar, outvec);
	}
	
	#[test]
	fn div_vec_by_scalar() {
		let scalar = 2.0;
		let vec = Vec3 {x:3.0, y:4.0, z:10.0};
		
		let outvec = Vec3 {x:1.5, y:2.0, z:5.0};
		
		assert_eq!(vec/scalar, outvec);
	}
	
	#[test]
	fn get_length_squared() {
		let vec = Vec3 {x:2.0, y:3.0, z:4.0};
		
		assert_eq!(vec.length_squared(), 16.0+9.0+4.0);
	}
	
	#[test]
	fn get_length(){
		let vec = Vec3 {x:4.0, y:3.0, z:0.0};
		
		assert_eq!(vec.length(), 5.0);
	}
	
	#[test]
	fn dot_product(){
		let vec1 = Vec3{ x:1.0, y:2.0, z:3.0};
		let vec2 = Vec3{ x:1.0, y:5.0, z:7.0};
		
		let out = 32.0;
		
		assert_eq!(vec1.dot(vec2), out);
	}
	
	#[test]
	fn cross_product(){
		let vec1 = Vec3{ x:1.0, y:2.0, z:3.0};
		let vec2 = Vec3{ x:1.0, y:5.0, z:7.0};
		
		let outvec = Vec3{ x:-1.0, y:-4.0, z:3.0};
		
		assert_eq!(vec1.cross(vec2), outvec);
	}
	
	#[test]
	fn unit_vector(){
		let vec = Vec3{ x:3.0, y:4.0, z:0.0};
		
		let outvec = Vec3{ x:0.6, y:0.8, z:0.0};
		
		assert_eq!(vec.unit_vector(), outvec);
	}
	
	#[test]
	fn print_color(){
		let vec = Vec3{ x:0.2, y: 0.2, z:0.2};
		
		assert_eq!(vec.to_color_string(1), "51 51 51");
	}
}

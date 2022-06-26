
use super::vec3::Vec3;
use super::ray::Ray;

pub struct HitRecord{
	pub point: Vec3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
}

impl HitRecord{
	pub fn new() -> HitRecord{
		HitRecord{point: Vec3 {x:0.0, y:0.0, z:0.0}, normal: Vec3 {x:0.0, y:0.0, z:0.0}, t: 0.0, front_face: false}
	}

	pub fn set_face_normal(& mut self, r: &Ray, outward_normal: Vec3){
		let front_face = r.direction.dot(outward_normal) < 0.0;
		if front_face{
			self.normal = outward_normal;
		} else {
			self.normal = outward_normal * -1.0;
		}
	}

	pub fn equals(& mut self, other: & HitRecord){
		self.point = other.point;
		self.normal = other.normal;
		self.t = other.t;
		self.front_face = other.front_face;
	}
}


#[cfg(test)]
mod tests{

	use super::HitRecord;
	use super::super::vec3::Vec3;

	#[test]
	fn new_hit_record(){
		let record = HitRecord::new();
		
		assert_eq!(record.point, Vec3 {x:0.0, y:0.0, z:0.0});
		assert_eq!(record.normal, Vec3 {x:0.0, y:0.0, z:0.0});
		assert_eq!(record.t, 0.0);
	}
}
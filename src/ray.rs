
use super::vec3::Vec3;
use super::vec3::random_in_unit_sphere;
use super::hittable_list::HittableList;
use super::hit_record::HitRecord;

pub struct Ray{
	pub origin: Vec3,
	pub direction: Vec3,
}

impl Ray{
	pub fn at(&self, t: f64)-> Vec3{
		let posvec = Vec3 {x: self.origin.x, y: self.origin.y, z: self.origin.z};
		let dirvec = Vec3 {x: self.direction.x, y: self.direction.y, z: self.direction.z};
		posvec + dirvec*t
	}
	
	pub fn color(&self, world: &HittableList, depth : i16) -> Vec3{
        if depth <= 0{
			return Vec3 {x: 0.0, y: 0.0, z: 0.0}
		}

		let mut rec = HitRecord::new();
        if world.hit(self, 0.0, 99999999.0, & mut rec){
			let target = rec.point + rec.normal + random_in_unit_sphere();
			let nr = Ray{origin: rec.point, direction: target - rec.point};
            return nr.color(world, depth-1)*0.5;
        }
		
		let unit_direction = self.direction.unit_vector();
		
		let t = 0.5*(unit_direction.y + 1.0);

		Vec3 {x: 1.0, y: 1.0, z: 1.0} * (1.0-t) + Vec3{x: 0.5,y: 0.7,z: 1.0} * t
	}
}

#[cfg(test)]
mod tests{

    use super::Vec3;
    use super::Ray;

	#[test]
	fn ray_at(){
		let ray = Ray{ origin: Vec3 {x:1.0, y:2.0, z:3.0}, direction: Vec3 {x:0.5, y:1.0, z:1.5}};
		let t = 2.0;
		
		let outvec = Vec3{x: 2.0, y: 4.0, z: 6.0};
		
		assert_eq!(ray.at(t), outvec);
	}
}
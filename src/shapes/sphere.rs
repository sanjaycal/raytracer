use super::super::vec3::Vec3;
use super::super::ray::Ray;
use super::super::hit_record::HitRecord;
use super::super::hit::Hit;


pub struct Sphere{
	pub center: Vec3,
	pub radius: f64,
}




impl Hit for Sphere{
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: & mut HitRecord) -> bool{
		let oc = r.origin - self.center;
		let a = r.direction.length_squared();
		let half_b = oc.dot(r.direction);
		let c = oc.length_squared() - self.radius*self.radius;
		let discriminant = half_b*half_b - a*c;
		if discriminant < 0.0 {
			return false;
		} 

		let sqrtd = discriminant.sqrt();

		let root = (- half_b - sqrtd) / a;

		if root < t_min || t_max < root{
			let root = (-half_b + sqrtd) / a;
			if root < t_min || t_max < root{
				return false;
			}
		}

		rec.t = root;
		rec.point = r.at(rec.t);
		rec.normal = (rec.point - self.center) / self.radius;
		
		let outward_normal = (rec.point - self.center) / self.radius;
		rec.set_face_normal(r, outward_normal);
		
		true

	}
}

#[cfg(test)]
mod tests{

    use super::Sphere;
    use super::super::super::vec3::Vec3;
    use super::super::super::ray::Ray;
    use super::super::super::hit::Hit;
    use super::super::super::hit_record::HitRecord;

	#[test]
	fn hit_sphere(){
		let sphere = Sphere{center: Vec3{x:5.0, y:0.0, z:0.0}, radius: 1.0};
		let mut hit_rec = HitRecord::new();
		let hit_ray = Ray{origin: Vec3{x:0.0, y:0.0, z:0.0}, direction: Vec3{x:1.0, y:0.0, z:0.0}};
		assert_eq!(sphere.hit(&hit_ray, 0.0, 10.0, & mut hit_rec), true);
		assert_eq!(hit_rec.point.x, 4.0);
		assert_eq!(hit_rec.point.y, 0.0);
		assert_eq!(hit_rec.point.z, 0.0);
		assert_eq!(hit_rec.t, 4.0);
	}
}
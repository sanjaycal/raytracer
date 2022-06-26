use super::hit::Hit;
use super::hit_record::HitRecord;
use super::ray::Ray;


pub struct HittableList{
	pub objects: Vec<Box<dyn Hit>>,
}

impl HittableList{

	pub fn new() -> HittableList{
		HittableList{objects: Vec::<Box<dyn Hit>>::new()}
	}

	pub fn add<T: 'static + Hit>(& mut self, object: T){
		self.objects.push(Box::new(object));
	}

	pub fn clear(& mut self){
		self.objects = Vec::<Box<dyn Hit>>::new();
	}

	pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: & mut HitRecord) -> bool{
		let mut temp_rec = HitRecord::new();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for object in &self.objects{
			if object.hit(r, t_min, t_max, & mut temp_rec){
				hit_anything = true;
                if temp_rec.t < closest_so_far{
				    closest_so_far = temp_rec.t; 
				    rec.equals(&temp_rec);
                }
			}
		}
		hit_anything
	}
}


#[cfg(test)]
mod tests{

    use super::super::shapes::sphere::Sphere;
    use super::super::vec3::Vec3;
    use super::super::ray::Ray;
    use super::super::hit_record::HitRecord;
    use super::HittableList;

	#[test]
	fn hit_sphere_in_hittable_list(){
		let sphere = Sphere{center: Vec3{x:5.0, y:0.0, z:0.0}, radius: 1.0};
		let mut hit_rec = HitRecord::new();
		let hit_ray = Ray{origin: Vec3{x:0.0, y:0.0, z:0.0}, direction: Vec3{x:1.0, y:0.0, z:0.0}};
		let mut hl = HittableList::new();
		hl.add(sphere);
		assert_eq!(hl.hit(&hit_ray, 0.0, 10.0, & mut hit_rec), true);
		assert_eq!(hit_rec.point.x, 4.0);
		assert_eq!(hit_rec.point.y, 0.0);
		assert_eq!(hit_rec.point.z, 0.0);
		assert_eq!(hit_rec.t, 4.0);
	}
	
}

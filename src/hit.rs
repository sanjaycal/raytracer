use super::ray::Ray;
use super::hit_record::HitRecord;

pub trait Hit{
	fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64, _rec: & mut HitRecord) -> bool{
		false
	}
}
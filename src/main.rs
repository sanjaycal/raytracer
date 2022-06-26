use raytracer::vec3::Vec3;
use raytracer::hittable_list::HittableList;
use raytracer::shapes::sphere::Sphere;
use raytracer::camera;

fn main() {
	
	let aspect_ratio = 16.0/9.0;
	
	let image_width = 400;
	let image_height  = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 100;
	let max_depth = 50;
	
	let mut world = HittableList::new();
	world.add(Sphere{center: Vec3{x:0.0, y:-100.5, z:-1.0}, radius: 100.0});
	world.add(Sphere{center: Vec3{x:0.0, y:0.0, z:-1.0}, radius: 0.5});

	let cam = camera::new_camera(aspect_ratio, 2.0, 1.0);
	
	println!("P3");
	println!("{} {}",image_width,image_height);
	println!("{}",255);
	
	for j in (0..image_height-1).rev(){
		eprintln!("\rscanlines remaining:{} ",j);
		for i in 0..image_width{
			let mut pixel_color = Vec3{x: 0.0, y: 0.0, z: 0.0};
			
			for _s in 0..samples_per_pixel{
				let u = (i as f64)/((image_width-1) as f64);
				let v = (j as f64)/((image_height-1) as f64);

				let r = cam.get_ray(u, v);
				
				pixel_color = pixel_color + r.color(&world, max_depth);
			}

			println!("{}", pixel_color.to_color_string(samples_per_pixel));
		}
	}
	
	eprintln!("Done!\n");
}

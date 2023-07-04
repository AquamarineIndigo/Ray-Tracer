// use std::intrinsics::{sqrtf64, float_to_int_unchecked};
// use std::io;
use console::style;
use image::{ImageBuffer, RgbImage};
// use indicatif::ProgressBar;
use std::{fs::File, process::exit};
// use std::num;
// use core::ops::Add;
// use core::ops::Mul;

struct Vec3 {
	x_dir: f64,
	y_dir: f64,
	z_dir: f64,
}

fn vec3_add(a: &Vec3, b: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a.x_dir + b.x_dir,
		y_dir: a.y_dir + b.y_dir,
		z_dir: a.z_dir + b.z_dir,
	}
}
fn vec3_mul(a: &f64, b: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a * b.x_dir,
		y_dir: a * b.y_dir,
		z_dir: a * b.z_dir,
	}
}
fn vec3_tri_add(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a.x_dir + b.x_dir + c.x_dir,
		y_dir: a.y_dir + b.y_dir + c.y_dir,
		z_dir: a.z_dir + b.z_dir + c.z_dir,
	}
}

impl Vec3 {
	// fn set(self, x: &f64, y: &f64, z: &f64) -> Vec3 {
	// 	Vec3 {
	// 		x_dir: *x,
	// 		y_dir: *y,
	// 		z_dir: *z,
	// 	}
	// }
	fn clone(self: &Self) -> Vec3 {
		Vec3 {
			x_dir: self.x_dir,
			y_dir: self.y_dir,
			z_dir: self.z_dir,
		}
	}
}

fn generate_unit_vector(direction: &Vec3) -> Vec3 {
	let l_sqr = direction.x_dir.powi(2) + direction.y_dir.powi(2) + direction.z_dir.powi(2);
	let l = l_sqr.sqrt();
	Vec3 {
		x_dir: direction.x_dir / l,
		y_dir: direction.y_dir / l,
		z_dir: direction.z_dir / l,
	}
}

struct Ray {
	direction: Vec3,
	origin: Vec3,
}

impl Ray {
	fn set(origin: Vec3, direction: Vec3) -> Self{
		Ray {
			direction, //: direction.clone(),
			origin, //: origin.clone(),
		}
	}

	// fn point_at_parameter(self: &Self, t: &f64) -> Vec3 {
	// 	// self.origin + (self.direction * t)
	// 	vec3_add(&self.origin, &vec3_mul(&t, &self.direction))
	// }
}

fn get_colour(r: &Ray) -> Vec3 {
	let unit_vector = generate_unit_vector(&r.direction);
	let t = 0.5 * (unit_vector.y_dir + 1.0);
	let para1: Vec3 = Vec3 {x_dir: 1.0, y_dir: 1.0, z_dir: 1.0};
	let para2: Vec3 = Vec3 {x_dir: 0.5, y_dir: 0.7, z_dir: 1.0};
	// (1.0 - t) * para1 + t * para2
	vec3_add(&vec3_mul(&(1.0 - t), &para1), &vec3_mul(&t, &para2))
}

fn main() {
	let path = "output/book1/image1-2.jpg";
	let height = 128;
	let width = 256;
	let quality = 255;
	let mut img: RgbImage = ImageBuffer::new(width, height);

	let lower_left_corner = Vec3 {x_dir: -2.0, y_dir: -1.0, z_dir: -1.0};
	let horizontal = Vec3 {x_dir: 4.0, y_dir: 0.0, z_dir: 0.0};
	let vertical = Vec3 {x_dir: 0.0, y_dir: 2.0, z_dir: 0.0};
	let origin = Vec3 {x_dir: 0.0, y_dir: 0.0, z_dir: 0.0};

	for j in 0..height { // rev?
		for i in 0..width {
			let u = (i as f64) / (width as f64);
			let v = (j as f64) / (height as f64);
			// let dir = 
			let r = Ray::set(
				origin.clone(), 
				vec3_tri_add(&lower_left_corner, &vec3_mul(&u, &horizontal), &vec3_mul(&v, &vertical)).clone(),
			);
			let pixel = img.get_pixel_mut(i, height - j - 1);
			let colour = get_colour(&r);
			let ir: f64 = 255.99 * colour.x_dir;
			let ig: f64 = 255.99 * colour.y_dir;
			let ib: f64 = 255.99 * colour.z_dir;
			*pixel = image::Rgb([ir as u8, ig as u8, ib as u8]);
			// println!("Position: (x: {i}, y: {j})");
		}
	}
	println!("Output image is in \"{}\"", style(path).green());
	let output_image = image::DynamicImage::ImageRgb8(img);
	let mut output_file = File::create(path).unwrap();
	match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
		Ok(_) => {}
		Err(_) => println!("{}", style("Output image failed").red()),
	}
	exit(0);
}
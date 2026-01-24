use myraytracing::camera::Camera;
use myraytracing::hittable::{Hittable, Sphere};
use myraytracing::hittable_list::HittableList;
use myraytracing::ray::Ray;
use myraytracing::rtweekend::{clamp, random_double, INFINITY};
use myraytracing::vec3::{Color, Point3, Vec3};
use std::sync::Arc;
use std::io::Write;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
	return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
	let target: Point3 = rec.p + rec.normal + Vec3::random_unit_vector();
	return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = Camera::new();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
	    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

	    for _ in 0..samples_per_pixel {
		let u: f64 = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
		let v: f64 = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

		let r = cam.get_ray(u, v);
		pixel_color += ray_color(&r, &world, max_depth);
	    }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}

use myraytracing::camera::Camera;
use myraytracing::hittable::{Hittable, Sphere};
use myraytracing::hittable_list::HittableList;
use myraytracing::ray::Ray;
use myraytracing::rtweekend::INFINITY;
use myraytracing::vec3::{Color, Point3};
use std::sync::Arc;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn write_color(pixel_color: Color) {
    // Write the translated [0,255] value of each color component.
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    );
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = Camera::new();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = cam.get_ray(u, v);

            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}

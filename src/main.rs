use myraytracing::ray::Ray;
use myraytracing::vec3::{Color, Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let di: Vec3 = r.direction();
    let a: f64 = di.dot(di);
    let b: f64 = 2.0 * di.dot(oc);
    let c: f64 = oc.dot(oc) - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: Ray) -> Vec3 {
    let t: f64 = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction: Vec3 = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(pixel_color: Color) {
    // Write the translated [0,255] value of each color component.
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}

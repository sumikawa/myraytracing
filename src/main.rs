use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use myraytracing::camera::Camera;
use myraytracing::config::Settings;
use myraytracing::hittable::{Hittable, Sphere};
use myraytracing::hittable_list::HittableList;
use myraytracing::material::{Dielectric, Lambertian, Metal};
use myraytracing::ray::Ray;
use myraytracing::rtweekend::random_double;
use myraytracing::texture::{CheckerTexture, SolidColor};
use myraytracing::vec3::{Color, Point3, Vec3};
use rayon::prelude::*;
use std::sync::Arc;

fn ray_color(mut r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut attenuation = Color::new(1.0, 1.0, 1.0);
    let mut current_depth = depth;

    while current_depth > 0 {
        if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
            let mut scattered = Ray::new(Point3::default(), Vec3::default());
            let mut scattered_attenuation = Color::default();
            if rec
                .mat_ptr
                .scatter(&r, &rec, &mut scattered_attenuation, &mut scattered)
            {
                attenuation = attenuation * scattered_attenuation;
                r = scattered;
            } else {
                return Color::default();
            }
        } else {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            let background_color =
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
            return attenuation * background_color;
        }
        current_depth -= 1;
    }

    Color::default() // If max_depth is reached, ray_color is black.
}

fn write_color(color: Color) -> image::Rgb<u8> {
    let r = (256.0 * color.x.sqrt().clamp(0.0, 0.9999)) as u8;
    let g = (256.0 * color.y.sqrt().clamp(0.0, 0.9999)) as u8;
    let b = (256.0 * color.z.sqrt().clamp(0.0, 0.9999)) as u8;

    image::Rgb([r, g, b])
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ))));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn myraytracing::material::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo))));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double();
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.4, 0.2, 0.1,
    )))));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0, // vfov
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    let pb = ProgressBar::new((image_width * image_height) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let pixels: Vec<(u32, u32, &mut image::Rgb<u8>)> = imgbuf.enumerate_pixels_mut().collect();

    pixels
        .into_par_iter()
        .progress_with(pb)
        .for_each(|(i, j, pixel)| {
            let mut pixel_color = Color::default();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v: f64 =
                    ((image_height - j - 1) as f64 + random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            pixel_color /= SAMPLES_PER_PIXEL as f64;
            *pixel = write_color(pixel_color);
        });

    imgbuf.save("output.png").unwrap();
}

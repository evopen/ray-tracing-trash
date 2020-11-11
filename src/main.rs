#![feature(clamp)]

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
use std::sync::{Arc, RwLock};
use vec3::Random;
use vec3::Vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList, Sphere};

type Color = Vec3;
type Point3 = Vec3;

fn write_color(pixel: &mut image::Rgb<u8>, mut pixel_color: Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f32;

    pixel_color *= scale;
    let (mut r, mut g, mut b) = pixel_color.into();
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();
    *pixel = image::Rgb([
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    ]);
}

fn ray_color(r: &ray::Ray, hittable: &dyn Hittable, depth: u32, rng: &mut ThreadRng) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();
    if let Some(rec) = hittable.hit(r, 0.001, std::f32::INFINITY) {
        let target = Vec3::from(rec.p + rec.normal + Vec3::gen_unit_vector(rng));
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), hittable, depth - 1, rng);
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 16;
    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);
    let max_depth = 50;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default();

    let start_time = std::time::Instant::now();
    log::info!("rendering started.");

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f32;
            let v =
                ((image_height - y) as f32 + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f32;
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(&r, &world, max_depth, &mut rng);
        }

        write_color(pixel, pixel_color, samples_per_pixel);
    });

    log::info!(
        "rendering finished, took {} ms",
        start_time.elapsed().as_millis()
    );

    img.save("result.png").unwrap();
}

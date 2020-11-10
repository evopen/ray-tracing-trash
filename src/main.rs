#![feature(clamp)]

mod camera;
mod hittable;
mod ray;

use glam::Vec3;
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList, Sphere};

type Color = Vec3;
type Point3 = Vec3;

fn write_color(pixel: &mut image::Rgb<u8>, mut pixel_color: Vec3, samples_per_pixel: u32) {
    pixel_color /= samples_per_pixel as f32;
    let (r, g, b) = pixel_color.into();
    *pixel = image::Rgb([
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    ]);
}

fn ray_color(r: &ray::Ray, hittable: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if hittable.hit(r, 0.0, std::f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().normalize();
    let t = unit_direction.y() + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    // World
    let mut world = Arc::new(RwLock::new(HittableList::default()));
    world
        .write()
        .unwrap()
        .add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .write()
        .unwrap()
        .add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default();

    let start_time = std::time::Instant::now();
    log::info!("rendering started.");

    let world_shared = world.clone();
    img.enumerate_pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f32;
                let v = ((image_height - y) as f32 + rng.gen_range(0.0, 1.0))
                    / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &*world_shared.read().unwrap());
            }

            write_color(pixel, pixel_color, samples_per_pixel);
        });
    // img.pixels_mut().par_bridge().into_par_iter()

    // img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
    //     let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    //     for _ in 0..samples_per_pixel {
    //         let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f32;
    //         let v =
    //             ((image_height - y) as f32 + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f32;
    //         let r = camera.get_ray(u, v);
    //         pixel_color += ray_color(&r, &world);
    //     }

    //     write_color(pixel, pixel_color, samples_per_pixel);
    // });

    log::info!(
        "rendering finished, took {} ms",
        start_time.elapsed().as_millis()
    );

    img.save("result.png").unwrap();
}

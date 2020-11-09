mod ray;
mod hittable;

use glam::Vec3 as Vec3;
use ray::Ray;
use rayon::prelude::*;

use hittable::sphere::Sphere;

type Color = Vec3;
type Point3 = Vec3;

fn write_color(pixel: &mut image::Rgb<u8>, pixel_color: &Vec3) {
    *pixel = image::Rgb([
        (255.999 * pixel_color.x()) as u8,
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8,
    ]);
}

fn ray_color(r: &ray::Ray) -> Color {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = Vec3::from(r.origin() - center.clone());
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    let start_time = std::time::Instant::now();
    log::info!("rendering started.");

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = x as f32 / (image_width - 1) as f32;
        let v = (image_height - y) as f32 / (image_height - 1) as f32;
        let r = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
        let pixel_color = ray_color(&r);
        write_color(pixel, &pixel_color);
    });

    log::info!(
        "rendering finished, took {} ms",
        start_time.elapsed().as_millis()
    );

    img.save("result.png").unwrap();
}

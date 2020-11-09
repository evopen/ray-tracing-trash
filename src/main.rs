use rayon::prelude::*;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    let start_time = std::time::Instant::now();
    log::info!("rendering started.");

    img.enumerate_pixels_mut().for_each(|(x, y, p)| {
        let r = x as f64 / (image_width - 1) as f64;
        let g = y as f64 / (image_height - 1) as f64;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *p = image::Rgb([ir, ig, ib]);
    });

    log::info!(
        "rendering finished, took {} ms",
        start_time.elapsed().as_millis()
    );

    img.save("result.png").unwrap();
}

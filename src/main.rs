use image::ImageBuffer;
use indicatif::ParallelProgressIterator;
use num::complex::Complex;
use palette::{Gradient, Pixel, Srgb};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

fn lerp(n: f64, domain: (f64, f64), range: (f64, f64)) -> f64 {
    let u = (n - domain.0) / (domain.1 - domain.0);
    u * (range.1 - range.0) + range.0
}

fn main() {
    let width = 1920;
    let height = 1080;
    let x_domain = (0.0, width as f64);
    let x_range = (-1.6, 1.6);
    let y_domain = (0.0, height as f64);
    let y_range = (-0.9, 0.9);
    let max_iter = 1000;
    let c = Complex::new(-0.8, 0.156);
    let fps = 60;
    let seconds = 23;
    let frames = fps * seconds;
    let zoom_factor: f64 = 0.99;
    let grad = Gradient::new(vec![
        Srgb::new(46.0 / 255.0, 46.0 / 255.0, 46.0 / 255.0).into_linear(),
        Srgb::new(108.0 / 255.0, 153.0 / 255.0, 187.0 / 255.0).into_linear(),
        Srgb::new(176.0 / 255.0, 82.0 / 255.0, 121.0 / 255.0).into_linear(),
        Srgb::new(158.0 / 255.0, 134.0 / 255.0, 200.0 / 255.0).into_linear(),
        Srgb::new(232.0 / 255.0, 125.0 / 255.0, 62.0 / 255.0).into_linear(),
        Srgb::new(180.0 / 255.0, 210.0 / 255.0, 115.0 / 255.0).into_linear(),
        Srgb::new(229.0 / 255.0, 181.0 / 255.0, 103.0 / 255.0).into_linear(),
        Srgb::new(214.0 / 255.0, 214.0 / 255.0, 214.0 / 255.0).into_linear(),
    ]);
    (0..frames)
        .into_par_iter()
        .progress_count(frames)
        .for_each(|frame| {
            let img = ImageBuffer::from_fn(width, height, |x, y| {
                let zoom_factor = zoom_factor.powf(frame as f64);
                let zoomed_x = (x_range.0 * zoom_factor, x_range.1 * zoom_factor);
                let zoomed_y = (y_range.0 * zoom_factor, y_range.1 * zoom_factor);
                let mut z = Complex::new(
                    lerp(x as f64, x_domain, zoomed_x),
                    lerp(y as f64, y_domain, zoomed_y),
                );
                let mut count = 0;
                while z.norm() < 2.0 && count < max_iter {
                    z = z * z + c;
                    count += 1;
                }
                let color: [u8; 3] = Srgb::from_linear(grad.get(count as f64 / max_iter as f64))
                    .into_format()
                    .into_raw();
                image::Rgb(color)
            });
            img.save(format!("frames/{}.png", frame)).unwrap();
        });
}

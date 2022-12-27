extern crate image;

use crate::vec3::Vec3;
#[cfg(test)]
use crate::ray::Ray;

use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

pub fn gen_ppm(img: Vec<Vec<Vec3>>, filename: String) -> () {

    // Time to write to image file!
    let path = Path::new(&filename);
    let display = path.display();
    let sizey = img.len() as u32;
    let sizex = img[0].len() as u32;
    let mut imgbuf = image::ImageBuffer::new(sizex, sizey);

    let bar = ProgressBar::new((img.len()) as u64);
    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining]    [writing to file]",
    ).ok().unwrap());

    for (y, row) in img.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.x() as u8, pixel.y() as u8, pixel.z() as u8]));
        }
        bar.inc(1);
    }

    let _ = image::DynamicImage::ImageRgb8(imgbuf).save(&path);
    
    bar.finish();
    println!("successfully wrote to {}", display);
}
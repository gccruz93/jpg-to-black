extern crate glob;
use glob::glob;
extern crate image;
use image::{GenericImageView, ImageBuffer, RgbaImage};

fn main() {
  for entry in glob("./*.jpg").expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        println!("{}", path.display());

        let img = image::open(&path).unwrap();
        let (width, height) = img.dimensions();
        let mut out: RgbaImage = ImageBuffer::new(width, height);

        for pixel in out.enumerate_pixels_mut() {
          *pixel.2 = image::Rgba([0, 0, 0, 255]);
        }

        out.save(path).expect("error.");

        println!("done.");
      }
      Err(e) => {
        println!("{}", e)
      }
    }
  }
}

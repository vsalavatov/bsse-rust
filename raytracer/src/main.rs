use raytracer::graphics::{color::ColorRGB, image::Image};

fn main() {
    let height: usize = 768;
    let width: usize = 1024;
    let mut image = Image::from_size(height, width);
    for row in 0..height {
        for col in 0..width {
            image[(row, col)] = ColorRGB(col as f32 / width as f32, row as f32 / height as f32, 0.0)
        }
    }

    image.save_as_ppm(String::from("img.ppm"));
}

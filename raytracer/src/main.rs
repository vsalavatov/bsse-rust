use raytracer::{
    geometry::Vec3,
    graphics::{color::ColorRGB, image::Image},
};

fn main() {
    let height: usize = 768;
    let width: usize = 1024;
    let mut image = Image::from_size(height, width);
    for row in 0..height {
        for col in 0..width {
            let v = Vec3 {
                x: row as f32 / height as f32,
                y: col as f32 / width as f32,
                z: 0.51,
            }
            .cross_product(Vec3 {
                x: 1.12453,
                y: -1.523942,
                z: 0.37648693,
            })
            .normalize();

            image[(row, col)] = ColorRGB(v.x, v.y, v.z)
        }
    }

    image.save_as_ppm(String::from("img.ppm"));
}

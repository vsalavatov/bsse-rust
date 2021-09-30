use std::f32::consts::PI;

use raytracer::{
    geometry::{Ray, Sphere, Vec3},
    graphics::{cast_ray, Image},
};

fn main() {
    let height: usize = 768;
    let width: usize = 1024;
    let fov = PI / 2.0;
    let aspect_ratio = width as f32 / height as f32;

    let mut image = Image::from_size(height, width);

    let sphere = Sphere {
        center: Vec3 {
            x: -3.0,
            y: 0.0,
            z: -16.0,
        },
        radius: 2.0,
    };

    for row in 0..height {
        for col in 0..width {
            let x =
                (2.0 * (col as f32 + 0.5) / width as f32 - 1.0) * (fov / 2.0).tan() * aspect_ratio;
            let y = -(2.0 * (row as f32 + 0.5) / height as f32 - 1.0) * (fov / 2.0).tan();
            let dir = Vec3 { x, y, z: -1.0 }.normalize();
            image[(row, col)] = cast_ray(Ray::new(Vec3::default(), dir), sphere);
        }
    }

    image.save_as_ppm(String::from("img.ppm"));
}

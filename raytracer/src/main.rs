fn main() {
    println!(
        "Hello, world! {:?}",
        raytracer::graphics::color::ColorRGB(0.5, 0.6, 0.7).quantize()
    );
}

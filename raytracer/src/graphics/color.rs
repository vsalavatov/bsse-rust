#[derive(Debug)]
pub struct ColorRGB(pub f32, pub f32, pub f32);

impl ColorRGB {
    pub fn quantize(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.) as u8,
            (self.1 * 255.) as u8,
            (self.2 * 255.) as u8,
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ColorRGB(pub f32, pub f32, pub f32);

pub const WHITE: ColorRGB = ColorRGB(1.0, 1.0, 1.0);
pub const BLACK: ColorRGB = ColorRGB(0.0, 0.0, 0.0);
pub const RED: ColorRGB = ColorRGB(1.0, 0.0, 0.0);
pub const GREEN: ColorRGB = ColorRGB(0.0, 1.0, 0.0);
pub const BLUE: ColorRGB = ColorRGB(0.0, 0.0, 1.0);

impl ColorRGB {
    pub fn quantize(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.) as u8,
            (self.1 * 255.) as u8,
            (self.2 * 255.) as u8,
        )
    }
}

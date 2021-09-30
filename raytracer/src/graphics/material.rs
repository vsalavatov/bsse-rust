use super::ColorRGB;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: ColorRGB,
}

pub const IVORY: Material = Material {
    diffuse_color: ColorRGB(0.4, 0.4, 0.3),
};
pub const RED_RUBBER: Material = Material {
    diffuse_color: ColorRGB(0.3, 0.1, 0.1),
};

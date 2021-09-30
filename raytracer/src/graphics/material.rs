use super::{ColorRGB, WHITE};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: ColorRGB,
    pub diffuse_albedo: f32,
    pub specular_exponent: f32,
    pub specular_albedo: f32,
    pub reflection_albedo: f32,
}

pub const IVORY: Material = Material {
    diffuse_color: ColorRGB(0.4, 0.4, 0.3),
    diffuse_albedo: 0.6,
    specular_exponent: 50.0,
    specular_albedo: 0.3,
    reflection_albedo: 0.1,
};
pub const RED_RUBBER: Material = Material {
    diffuse_color: ColorRGB(0.3, 0.1, 0.1),
    diffuse_albedo: 0.9,
    specular_exponent: 10.0,
    specular_albedo: 0.1,
    reflection_albedo: 0.0,
};
pub const MIRROR: Material = Material {
    diffuse_color: WHITE,
    diffuse_albedo: 0.0,
    specular_exponent: 1425.0,
    specular_albedo: 10.0,
    reflection_albedo: 0.8,
};

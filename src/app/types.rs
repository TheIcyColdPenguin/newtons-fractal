use image::{ImageBuffer, Rgba};
use piston_window::{G2dTexture, PistonWindow, TextureContext};

use crate::math::types::Polynomial;

pub struct Innards {
    pub window: PistonWindow,
    pub texture: G2dTexture,
    pub texture_context: TextureContext<
        gfx_device_gl::Factory,
        gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer,
    >,
    pub canvas: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

pub struct AppSettings {
    pub innards: Innards,
    pub zoom: i8,
    pub resolution_scale: u8,
}

pub struct MathInnards {
    pub poly: Polynomial,
    pub deriv: Polynomial,
}

pub struct App {
    pub settings: AppSettings,
    pub math: MathInnards,
}

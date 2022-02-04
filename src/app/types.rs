use image::{ImageBuffer, Rgba};
use piston_window::{keyboard::ModifierKey, G2dTexture, PistonWindow, TextureContext};

use crate::math::types::{Complex, Polynomial};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<(u32, u32)> for Point {
    fn from(point: (u32, u32)) -> Point {
        Point {
            x: point.0 as f64,
            y: point.1 as f64,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Area {
    pub start: Point,
    pub end: Point,
}

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
    pub is_panning: bool,
    pub area: Option<Area>,
    pub modifiers: ModifierKey,
}

pub struct MathInnards {
    pub poly: Polynomial,
    pub deriv: Polynomial,
    pub roots: Vec<(Complex, [u8; 3])>,
}

pub struct App {
    pub settings: AppSettings,
    pub math: MathInnards,
}

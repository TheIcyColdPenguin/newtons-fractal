use crate::app::types::{AppSettings, Innards};

use image::ImageBuffer;
use piston_window::{
    keyboard::ModifierKey, G2dTexture, PistonWindow, Size, Texture, TextureContext,
    TextureSettings, Window,
};

impl AppSettings {
    pub fn new(mut window: PistonWindow) -> AppSettings {
        let Size { width, height } = window.window.size();

        let canvas = ImageBuffer::new(width as u32, height as u32);
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };
        let texture: G2dTexture =
            Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

        AppSettings {
            resolution_scale: 10,
            zoom: 7,
            area: None,
            is_panning: false,
            modifiers: ModifierKey::NO_MODIFIER,
            innards: Innards {
                window,
                texture,
                texture_context,
                canvas,
            },
        }
    }
}

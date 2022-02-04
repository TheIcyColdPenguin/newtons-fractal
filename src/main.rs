#![windows_subsystem = "windows"]

mod app;
mod math;

use opengl_graphics::OpenGL;
use piston_window::{PistonWindow, WindowSettings};

use app::types::{App, AppSettings};

fn main() {
    let opengl = OpenGL::V4_1;

    let window: PistonWindow = WindowSettings::new("Newton's Fractal", [1000, 700])
        .exit_on_esc(true)
        .fullscreen(true)
        .vsync(true)
        .resizable(false)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let app_settings = AppSettings::new(window);

    let mut app = App::new(app_settings);

    app.run();
}

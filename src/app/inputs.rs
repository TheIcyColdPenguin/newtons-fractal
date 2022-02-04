use crate::app::{
    helpers::linear_to_log,
    types::{App, Area, Point},
};

use piston_window::{keyboard::ModifierKey, Button, ButtonArgs, ButtonState, Key, MouseButton};

impl App {
    pub fn manage_mouse_click(&mut self, button: Button) -> bool {
        if button == Button::Mouse(MouseButton::Left) {
            if self.settings.modifiers.contains(ModifierKey::SHIFT) {
                // TODO: implement moving roots around
            } else {
                self.settings.is_panning = true;
            }
            true
        } else {
            false
        }
    }

    pub fn manage_scroll(&mut self, scroll: [f64; 2]) {
        if scroll[1] > 0.0 {
            self.set_zoom(self.settings.zoom + 1);
        } else {
            self.set_zoom(self.settings.zoom - 1);
        }
    }

    pub fn manage_input(&mut self, args: ButtonArgs) -> bool {
        if args.state != ButtonState::Press {
            return false;
        }

        if let Button::Keyboard(key) = args.button {
            match key {
                x if x == Key::Plus || x == Key::Equals => self.set_zoom(self.settings.zoom + 1),
                x if x == Key::Minus || x == Key::Underscore => {
                    self.set_zoom(self.settings.zoom - 1)
                }
                x if [
                    Key::D1,
                    Key::D2,
                    Key::D3,
                    Key::D4,
                    Key::D5,
                    Key::D6,
                    Key::D7,
                    Key::D8,
                ]
                .contains(&x) =>
                {
                    // subtract 0x30 to get the actual number from the key code
                    self.settings.resolution_scale = (10 - (x as u8 - 0x30)) * 2;
                }
                Key::D9 => self.settings.resolution_scale = 1,
                _ => return false,
            }
        }

        true
    }
    pub fn manage_pan(&mut self, pos: [f64; 2]) -> bool {
        if self.settings.is_panning {
            let offset = linear_to_log(self.settings.zoom as f64);
            let movement = Point {
                x: offset * -1.39e-2 * pos[0],
                y: offset * -1.39e-2 * pos[1],
            };

            let prev_area = self.settings.area.unwrap();

            self.settings.area = Some(Area {
                start: Point {
                    x: prev_area.start.x + movement.x,
                    y: prev_area.start.y + movement.y,
                },
                end: Point {
                    x: prev_area.end.x + movement.x,
                    y: prev_area.end.y + movement.y,
                },
            });
            true
        } else {
            false
        }
    }
}

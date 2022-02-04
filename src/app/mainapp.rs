use crate::app::types::{App, AppSettings, MathInnards};
use crate::math::types::{Complex, Item, Polynomial};

use piston_window::{
    Button, ButtonEvent, Event, EventSettings, Events, MouseButton, MouseRelativeEvent,
    MouseScrollEvent, PressEvent, ReleaseEvent, RenderEvent,
};

impl App {
    pub fn new(settings: AppSettings) -> App {
        // hardcode the values for now

        let poly = Polynomial(vec![
            Item {
                pow: 3,
                coeff: 1.0.into(),
            },
            Item {
                pow: 0,
                coeff: (-1.0).into(),
            },
        ]);

        let deriv = poly.derivative();

        let roots = vec![
            (
                Complex {
                    real: 1.0,
                    imag: 0.0,
                },
                [255, 0, 0],
            ),
            (
                Complex {
                    real: -0.5,
                    imag: -0.86602540378,
                },
                [0, 255, 0],
            ),
            (
                Complex {
                    real: -0.5,
                    imag: 0.86602540378,
                },
                [0, 0, 255],
            ),
        ];

        let mut app = App {
            settings,
            math: MathInnards { poly, deriv, roots },
        };

        app.settings.area = Some(app.calc_world_size());

        app
    }

    fn render(&mut self, e: &Event) {
        use graphics::{clear, image};

        let innards = &mut self.settings.innards;

        innards
            .texture
            .update(&mut innards.texture_context, &innards.canvas)
            .unwrap();

        innards.window.draw_2d(e, |c, gl, device| {
            innards.texture_context.encoder.flush(device);

            clear([1.0; 4], gl);
            image(&innards.texture, c.transform, gl);
        });
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        self.draw();

        let mut should_update = false;

        while let Some(event) = events.next(&mut self.settings.innards.window) {
            self.settings.modifiers.event(&event);

            // render out the drawn canvas
            if let Some(_) = event.render_args() {
                self.render(&event);
            }

            if let Some(button) = event.press_args() {
                self.manage_mouse_click(button);
            };

            if let Some(button) = event.release_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    self.settings.is_panning = false;
                }
            };

            // zoom on scroll
            if let Some(scroll) = event.mouse_scroll_args() {
                self.manage_scroll(scroll);
                should_update = true;
            }

            // use inputs
            if let Some(args) = event.button_args() {
                let updated = self.manage_input(args);
                if updated {
                    should_update = true;
                }
            }
            if let Some(args) = event.mouse_relative_args() {
                let updated = self.manage_pan(args);
                if updated {
                    should_update = true;
                }
            }

            if should_update {
                self.draw();
                should_update = false;
            }
        }
    }
}

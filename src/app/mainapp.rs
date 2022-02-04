use crate::app::types::{App, AppSettings, MathInnards};
use crate::math::types::{Item, Polynomial};

use piston_window::{Event, EventSettings, Events, RenderEvent};

impl App {
    pub fn new(settings: AppSettings) -> App {
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

        App {
            settings,
            math: MathInnards { poly, deriv },
        }
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

        while let Some(event) = events.next(&mut self.settings.innards.window) {
            // render out the drawn canvas
            if let Some(_) = event.render_args() {
                self.render(&event);
            }
        }
    }
}

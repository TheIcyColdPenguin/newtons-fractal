use crate::app::types::{App, AppSettings, MathInnards};
use crate::math::types::{Complex, Item, Polynomial};

use piston_window::{Event, EventSettings, Events, RenderEvent};

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

        while let Some(event) = events.next(&mut self.settings.innards.window) {
            // render out the drawn canvas
            if let Some(_) = event.render_args() {
                self.render(&event);
            }
        }
    }
}

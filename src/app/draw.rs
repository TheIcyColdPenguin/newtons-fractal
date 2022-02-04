use crate::app::{helpers::screen_to_cart, types::App};
use crate::math::helpers::NewtonIterator;

use image::Rgba;

impl App {
    pub fn draw(&mut self) {
        let (width, height) = self.get_size();
        let (width, height) = (width as u32, height as u32);
        let res_scale = self.settings.resolution_scale as u32;

        let area = self.settings.area.unwrap_or_else(|| self.calc_world_size());
        let canvas = &mut self.settings.innards.canvas;

        for x in (0..(width - res_scale + 1)).step_by(res_scale as usize) {
            for y in (0..(height - res_scale + 1)).step_by(res_scale as usize) {
                let cart = screen_to_cart((x, y).into(), (width, height), &area);
                let mut newton_iterator =
                    NewtonIterator::new(cart, &self.math.poly, &self.math.deriv, 200);

                let mut col = [0, 0, 0];

                'outer: while let Some(z) = newton_iterator.next() {
                    for root in self.math.roots.iter() {
                        if z.close_enough(root.0) {
                            col = root.1;
                            break 'outer;
                        }
                    }
                }

                for x_off in 0..res_scale {
                    for y_off in 0..res_scale {
                        canvas.put_pixel(x + x_off, y + y_off, Rgba([col[0], col[1], col[2], 255]));
                    }
                }
            }
        }
    }
}

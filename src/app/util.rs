use crate::app::types::{App, Area, Point};

use piston_window::{Size, Window};

impl App {
    pub fn get_size(&self) -> (f64, f64) {
        let Size { width, height } = self.settings.innards.window.window.size();
        (width, height)
    }

    pub fn calc_world_size(&self) -> Area {
        // first find the smaller edge of the screen so that everything fits on screen
        let (width, height) = self.get_size();

        let smaller_edge = if width < height { width } else { height };

        // now,this edge should have a range of [-2, 2], or a size of 2, starting at -2
        // therefore, the units/pixel will be size/smaller_edge

        let units_per_pixel = 4.0 / smaller_edge;

        // finally, set the boundaries

        let half_x_size = (units_per_pixel * width) / 2.0;
        let half_y_size = (units_per_pixel * height) / 2.0;

        Area {
            start: Point {
                x: -half_x_size,
                y: -half_y_size,
            },
            end: Point {
                x: half_x_size,
                y: half_y_size,
            },
        }
    }
}

use super::log;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use geo_types::{Coord, Line, LineString};

pub trait Rasterize: Sized {
    fn rasterize(&self) -> Box<[i32]>;
}

impl Rasterize for LineString<i32> {
    fn rasterize(&self) -> Box<[i32]> {
        console_log!("self: {:#?}", self);

        let min_x = self.into_iter().map(|coord| coord.x).min().unwrap();
        let max_x = self.into_iter().map(|coord| coord.x).max().unwrap();
        let min_y = self.into_iter().map(|coord| coord.y).min().unwrap();
        let max_y = self.into_iter().map(|coord| coord.y).max().unwrap();

        let offset: Coord<i32> = (min_x, min_y).into();

        let width = max_x - min_x;
        let height = max_y - min_y;

        let self_with_offset: LineString<i32> =
            self.0.iter().map(|coord| *coord - offset).collect();

        console_log!("self with offset: {:#?}", self_with_offset);

        let mut res = vec![0; (width * height) as usize];

        for line in self_with_offset.lines() {
            console_log!("line: {:#?}", line);

            // correct_line with correct slope
            // maybe use `.slope`?
            let corrected_line =
                if (line.end.y - line.start.y).abs() < (line.end.x - line.start.x).abs() {
                    if line.start.x > line.start.x {
                        Line::new(line.end, line.start)
                        // line
                    } else {
                        line
                    }
                } else {
                    if line.start.y > line.end.y {
                        Line::new(line.end, line.start)
                    } else {
                        line
                    }
                };

            console_log!("correct line: {:#?}", corrected_line);

            let (xi, dx) = if corrected_line.dx() < 0 {
                (-1, -corrected_line.dx())
            } else {
                (1, corrected_line.dx())
            };
            let mut D = (2 * dx) - corrected_line.dy();

            let mut x = corrected_line.start.x;

            for y in corrected_line.start.y..corrected_line.end.y {
                res[(width * x + y) as usize] = 255;
                D = if D > 0 {
                    x += xi;
                    D + (2 * (dx - corrected_line.dy()))
                } else {
                    D + 2 * dx
                }
            }
        }

        res = res
            .iter()
            .map(|val| [*val, *val, *val, *val])
            .flatten()
            .collect::<Vec<_>>();

        res.insert(0, (max_x - min_x) as i32);
        res.insert(0, (max_y - min_y) as i32);
        res.into_boxed_slice()
    }
}

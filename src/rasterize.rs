use geo::GeoNum;
use geo_types::{Coord, Line, LineString};

pub trait Rasterize: Sized {
    fn rasterize(&self) -> Box<[u8]>;
}

impl Rasterize for LineString<i64> {
    fn rasterize(&self) -> Box<[u8]> {
        let min_x = self
            .into_iter()
            .map(|coord| coord.x)
            // .fold(f64::NAN, f64::min);
            .fold(i64::MAX, i64::min);
        let max_x = self
            .into_iter()
            .map(|coord| coord.x)
            .fold(i64::MIN, i64::max);
        let min_y = self
            .into_iter()
            .map(|coord| coord.y)
            .fold(i64::MAX, i64::min);
        let max_y = self
            .into_iter()
            .map(|coord| coord.y)
            .fold(i64::MIN, i64::max);

        let offset: Coord<i64> = (min_x, max_y).into();

        let mut res = vec![0; ((max_x - min_x) * (max_y - min_y)) as usize];

        for line in self.lines() {
            // line = line
            let (xi, dx) = if line.dx() < 0 {
                (-1, -line.dx())
            } else {
                (1, line.dx())
            };
            let mut D = (2 * dx) - line.dy();

            let mut x = line.start.x;

            for y in line.start.y..line.end.y {
                res[((max_x - min_x) * (x - min_x) + (y - min_y)) as usize] = 1;
                D = if D > 0 {
                    x += xi;
                    D + (2 * (dx - line.dy()))
                } else {
                    D + 2 * dx
                }
            }
        }

        res.append(&mut vec![(max_x - min_x) as u8, (max_y - min_y) as u8]);
        res.into_boxed_slice()
    }
}

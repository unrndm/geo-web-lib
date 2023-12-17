use std::iter::once;

use geo::{coords_iter::CoordsIter, winding_order::Winding, LineString, Point};
use itertools::Itertools;

use crate::{Rasterize, Rasterizer};

fn y_coordinates<'a>(
    first: &'a LineString<f64>,
    rest: &'a [LineString<f64>],
) -> impl Iterator<Item = isize> + 'a {
    once(first)
        .chain(rest)
        .flat_map(|line_string| line_string.points().map(|point| point.y().floor() as isize))
}

type PointPair = (Point<f64>, Point<f64>);

/// Iterate over all points in a polygon (both the exterior and all the holes)
/// in clockwise order and copy all succeding pairs of points into a Vec
fn into_pointpairs(first: &LineString<f64>, rest: &[LineString<f64>]) -> Vec<PointPair> {
    let num_pairs = once(first).chain(rest).map(|line| line.0.len() - 1).sum();
    let mut result = Vec::with_capacity(num_pairs);

    for ls in once(first).chain(rest) {
        if ls.is_cw() {
            result.extend(ls.points().tuple_windows::<PointPair>());
        } else {
            result.extend(ls.points().rev().tuple_windows::<PointPair>());
        }
    }

    result
}

pub fn rasterize_polygon(
    first: &LineString<f64>,
    rest: &[LineString<f64>],
    rasterizer: &mut Rasterizer,
) {
    // rasterize borders
    once(first)
        .chain(rest)
        .for_each(|ls| ls.rasterize(rasterizer));

    // rasterize insides
    assert!(first.is_closed() && rest.iter().all(|line| line.is_closed()));

    let total_points = first.coords_count()
        + rest
            .iter()
            .map(|line_string| line_string.coords_count())
            .sum::<usize>();

    let mut xs: Vec<isize> = Vec::with_capacity(total_points);

    let min_y = y_coordinates(first, rest).min().unwrap().max(0);
    let max_y = y_coordinates(first, rest)
        .max()
        .unwrap()
        .min(rasterizer.height() as isize - 1);

    let min_x = 0;
    let max_x = rasterizer.width() - 1;

    let cw_points = into_pointpairs(first, rest);

    for y in min_y..(max_y + 1) {
        let dy = 0.5 + (y as f64); // center height of line

        // iterate over ring point pairs
        for (ind1, ind2) in cw_points.iter() {
            let mut dy1 = ind1.y();
            let mut dy2 = ind2.y();

            if (dy1 < dy && dy2 < dy) || (dy1 > dy && dy2 > dy) {
                continue;
            }

            let (dx1, dx2) = if dy1 < dy2 {
                (ind1.x(), ind2.x())
            } else if dy1 > dy2 {
                std::mem::swap(&mut dy1, &mut dy2);
                (ind2.x(), ind1.x())
            } else {
                if ind1.x() > ind2.x() {
                    let horizontal_x1 = (ind2.x() + 0.5).floor() as isize;
                    let horizontal_x2 = (ind1.x() + 0.5).floor() as isize;
                    if horizontal_x1 > (max_x as isize) || horizontal_x2 <= min_x {
                        continue;
                    }
                    rasterizer.fill_horizontal_line(
                        horizontal_x1 as usize,
                        horizontal_x2 as usize,
                        y as usize,
                    );
                }
                continue;
            };

            if dy < dy2 && dy >= dy1 {
                let intersect = (dy - dy1) * (dx2 - dx1) / (dy2 - dy1) + dx1;
                xs.push((intersect + 0.5).floor() as isize);
            }
        }

        xs.sort();
        for pair in xs[..].chunks_exact(2) {
            let x_start = pair[0].max(min_x);
            let x_end = pair[1].min((max_x + 1) as isize);
            if x_start <= (max_x as isize) && x_end > min_x {
                rasterizer.fill_horizontal_line(x_start as usize, x_end as usize, y as usize);
            }
        }
        xs.clear();
    }
}

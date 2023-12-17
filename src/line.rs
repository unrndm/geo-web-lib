use geo::Line;

use crate::Rasterizer;

pub fn rasterize_line(line: &Line<f64>, rasterizer: &mut Rasterizer) {
    let width = rasterizer.width() as f64;
    let height = rasterizer.height() as f64;

    if (line.start.y < 0.0 && line.end.y < 0.0)
        || (line.start.y > height && line.end.y > height)
        || (line.start.x < 0.0 && line.end.x < 0.0)
        || (line.start.x > width && line.end.x > width)
    {
        return;
    }

    let line = if line.start.x > line.end.x {
        // todo: use mem::swap??
        Line::new(line.end, line.start)
    } else {
        *line
    };

    const LINE_THRESHOLD: f64 = 0.01;
    let is_vertical =
        (line.start.x.floor() == line.end.x.floor()) || line.dx().abs() < LINE_THRESHOLD;
    let is_horizontal =
        (line.start.y.floor() == line.end.y.floor()) || line.dy().abs() < LINE_THRESHOLD;

    if is_horizontal {
        let (x_start, x_end) = if line.start.x > line.end.x {
            (line.end.x, line.start.x)
        } else {
            (line.start.x, line.end.x)
        };
        let y = line.start.y;

        let iy = y.floor() as isize;
        if iy < 0 || iy >= (rasterizer.height() as isize) {
            return;
        }

        let x_start = (x_start.floor() as usize).clamp(0, rasterizer.width() - 1);
        let x_end = (x_end.floor() as usize).clamp(0, rasterizer.width() - 1);
        rasterizer.fill_horizontal_line(x_start, x_end, iy as usize);
    } else if is_vertical {
        let (y_start, y_end) = if line.start.y > line.end.y {
            (line.end.y, line.start.y)
        } else {
            (line.start.y, line.end.y)
        };
        let x = line.end.x;

        let ix = x.floor() as isize;
        if ix < 0 || ix >= (rasterizer.width() as isize) {
            return;
        }

        let y_start = (y_start.floor() as usize).clamp(0, rasterizer.height() - 1);
        let y_end = (y_end.floor() as usize).clamp(0, rasterizer.height() - 1);
        rasterizer.fill_vertical_line(ix as usize, y_start, y_end);
    } else {
        let (mut x_start, mut y_start) = line.start.x_y();
        let (mut x_end, mut y_end) = line.end.x_y();

        let slope = line.slope();

        // clip x
        if x_end > width {
            y_end -= (x_end - width) * slope;
            x_end = width;
        }
        if x_start < 0.0 {
            y_start += (0.0 - x_start) * slope;
            x_start = 0.0;
        }

        // clip y
        if y_end > y_start {
            if y_start < 0.0 {
                let x_diff = -y_start / slope;
                x_start += x_diff;
                y_start = 0.0;
            }
            if y_end >= height {
                x_end += (y_end - height) / slope;
                // y_end = height;
            }
        } else {
            if y_start >= height {
                let x_diff = (height - y_start) / slope;
                x_start += x_diff;
                y_start = height;
            }
            if y_end < 0.0 {
                x_end -= y_end / slope;
                // y_end = 0.0;
            }
        }

        // pixel by pixel
        while (x_start >= 0.0) && (x_start < x_end) {
            let ix = x_start.floor() as isize;
            let iy = y_start.floor() as isize;

            if iy >= 0 && ((iy as usize) < rasterizer.height()) {
                rasterizer.fill_horizontal_line(ix as usize, ix as usize, iy as usize);
            }

            let mut x_step = (x_start + 1.).floor() - x_start;
            let mut y_step = x_step * slope;

            // step to the right pixel without changing scanline?
            if ((y_start + y_step).floor() as isize) == iy {
                // skip
            } else if slope < 0.0 {
                const STEP_THRESHOLD: f64 = -0.000000001;
                y_step = ((iy as f64) - y_start).min(STEP_THRESHOLD);
                x_step = y_step / slope;
            } else {
                const STEP_THRESHOLD: f64 = 0.000000001;
                y_step = (((iy + 1) as f64) - y_start).max(STEP_THRESHOLD);
                x_step = y_step / slope;
            }
            x_start += x_step;
            y_start += y_step;
        }
    }
}

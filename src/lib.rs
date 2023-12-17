mod line;
mod macros;
mod polygon;
mod rasterize;
mod utils;

use geo::Geometry;

use ndarray::{s, Array2};
use rasterize::Rasterize;
use wasm_bindgen::prelude::*;
use wkt::TryFromWkt;

#[wasm_bindgen]
pub struct Rasterizer {
    pixels: Array2<bool>,
}

#[wasm_bindgen]
impl Rasterizer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Rasterizer {
        // utils::set_panic_hook();
        // console_log!("I can reuse macros");
        Rasterizer {
            pixels: Array2::from_elem((width, height), false),
        }
    }

    fn width(&self) -> usize {
        self.pixels.shape()[1]
    }

    fn height(&self) -> usize {
        self.pixels.shape()[0]
    }

    fn fill_pixel(&mut self, ix: usize, iy: usize) {
        let mut slice = self.pixels.slice_mut(s![iy, ix]);
        slice.fill(true);
    }

    fn fill_vertical_line(&mut self, x: usize, y_start: usize, y_end: usize) {
        for y in y_start..(y_end + 1) {
            self.fill_pixel(x, y);
        }
    }

    fn fill_horizontal_line(&mut self, x_start: usize, x_end: usize, y: usize) {
        for x in x_start..(x_end + 1) {
            self.fill_pixel(x, y);
        }
    }

    pub fn rasterize(&mut self, wkt: &str) {
        let geometry = Geometry::try_from_wkt_str(wkt).unwrap_throw();
        geometry.rasterize(self);
    }

    pub fn pixels(&self) -> Box<[u8]> {
        self.pixels
            .iter()
            .map(|&x| x as u8)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

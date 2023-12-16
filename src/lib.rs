mod macros;
mod rasterize;
mod utils;

use rasterize::Rasterize;

use geo_types::LineString;
use wasm_bindgen::prelude::*;
use wkt::TryFromWkt;

#[wasm_bindgen]
pub struct Calculator {}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        utils::set_panic_hook();
        console_log!("I can reuse macros");

        Calculator {}
    }

    pub fn rasterize_line(&self, simple_line_wkt: &str) -> Box<[i32]> {
        let line: LineString<i32> = LineString::try_from_wkt_str(simple_line_wkt).unwrap_throw();
        line.rasterize_line()
    }
}

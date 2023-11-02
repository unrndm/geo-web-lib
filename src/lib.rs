mod rasterize;
mod utils;

use rasterize::Rasterize;

use geo_types::LineString;
use wasm_bindgen::prelude::*;
use wkt::TryFromWkt;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Calculator {}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        utils::set_panic_hook();

        Calculator {}
    }

    pub fn rasterize(&self, simple_line_wkt: &str) -> Box<[i32]> {
        let line: LineString<i32> = LineString::try_from_wkt_str(simple_line_wkt).unwrap_throw();
        line.rasterize()
    }
}

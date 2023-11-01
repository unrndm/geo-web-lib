// mod geomdiff;
mod rasterize;
mod utils;

use rasterize::Rasterize;
// use std::collections::HashMap;

// use geo::algorithm::BooleanOps;
use geo_types::{Geometry, LineString};
use wasm_bindgen::prelude::*;
use wkt::{ToWkt, TryFromWkt};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// #[derive(Debug)]
// pub enum Data {
//     Vector(String),
//     // RasterF64 {
//     //     value: Box<[f64]>,
//     //     shape: Box<[i64]>,
//     // },
// }

#[wasm_bindgen]
pub struct Calculator {
    // data: HashMap<String, Data>,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        utils::set_panic_hook();

        Calculator {
        //     data: HashMap::new(),
        }
    }

    pub fn rasterize(&self, simple_line_wkt: &str) -> Box<[u8]> {
        console_log!("{:#?}", simple_line_wkt);
        let line: LineString<i64> = LineString::try_from_wkt_str(simple_line_wkt).unwrap_throw();
        line.rasterize()
    }

    // pub fn add_variable_from_wkt(&mut self, name: String, wkt: String) {
    //     self.data.insert(name, Data::Vector(wkt));
    // }

    // pub fn calculate(&self, operation: &str) -> Result<String, String> {
    //     console_log!("{:#?}", self.data);
    //     match operation {
    //         // "intersection" => match (self.data.get("A"), self.data.get("B")) {
    //         //     (Some(Data::Vector(a)), Some(Data::Vector(b))) => {
    //         //         let a_geom: Geometry<f64> = Geometry::try_from_wkt_str(a).unwrap();
    //         //         let b_geom: Geometry<f64> = Geometry::try_from_wkt_str(b).unwrap();
    //         //         Ok(a_geom.intersection(&b_geom).to_wkt().to_string())
    //         //     }
    //         //     _ => Err("not enough variable defined".to_string()),
    //         // },
    //         "geomdiff" => match (self.data.get("A"), self.data.get("B")) {
    //             (Some(Data::Vector(a)), Some(Data::Vector(b))) => {
    //                 let a_geom: Point<f64> = Point::try_from_wkt_str(a).unwrap();
    //                 let b_geom: Point<f64> = Point::try_from_wkt_str(b).unwrap();
    //                 Ok(format!("{:#?}", a_geom.diff(b_geom.into())).to_string())
    //             }
    //             _ => Err("not enough variable defined".to_string()),
    //         },
    //         _ => Err("unknown operation".to_string()),
    //     }
    // }
}

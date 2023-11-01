mod geomdiff;
mod utils;

use std::collections::HashMap;

use geo::algorithm::BooleanOps;
use geo_types::MultiPolygon;
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

#[derive(Debug)]
pub enum Data {
    Vector(String),
    // RasterF64 {
    //     value: Box<[f64]>,
    //     shape: Box<[i64]>,
    // },
}

#[wasm_bindgen]
pub struct Calculator {
    data: HashMap<String, Data>,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        utils::set_panic_hook();

        Calculator {
            data: HashMap::new(),
        }
    }

    pub fn add_variable_from_wkt(&mut self, name: String, wkt: String) {
        self.data.insert(name, Data::Vector(wkt));
    }

    pub fn calculate(&self, operation: &str) -> Result<String, String> {
        console_log!("{:#?}", self.data);
        match operation {
            "intersection" => match (self.data.get("A"), self.data.get("B")) {
                (Some(Data::Vector(A)), Some(Data::Vector(B))) => {
                    let a_geom: MultiPolygon<f64> = MultiPolygon::try_from_wkt_str(A).unwrap();
                    let b_geom: MultiPolygon<f64> = MultiPolygon::try_from_wkt_str(B).unwrap();
                    return Ok(a_geom.intersection(&b_geom).to_wkt().to_string());
                }
                _ => return Err("not enough variable defined".to_string()),
            },
            "geomdiff" => match (self.data.get("A"), self.data.get("B")) {
                (Some(Data::Vector(A)), Some(Data::Vector(B))) => {
                    // return Ok(geomdiff);
                    return Ok("".to_string());
                }
                _ => return Err("not enough variable defined".to_string()),
            },
            _ => return Err("unknown operation".to_string()),
        }
    }
}

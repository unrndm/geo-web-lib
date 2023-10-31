mod utils;

use geo::algorithm::BooleanOps;
use geo_types::MultiPolygon;
use wasm_bindgen::prelude::*;
use wkt::{ToWkt, TryFromWkt};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, geo-web-lib!");
}

#[wasm_bindgen]
pub struct Calculator {
    A: Option<String>,
    B: Option<String>,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        utils::set_panic_hook();

        Calculator { A: None, B: None }
    }

    pub fn add_variable_from_wkt(&mut self, name: String, wkt: String) -> Result<(), String> {
        if name == "A" {
            self.A = Some(wkt);
        } else if name == "B" {
            self.B = Some(wkt);
        } else {
            return Err("unknown variable name".to_string());
        }
        Ok(())
    }

    pub fn calculate(&self, operation: String) -> Result<String, String> {
        if operation == "intersection" {
            let A: MultiPolygon<f64> =
                MultiPolygon::try_from_wkt_str(&self.A.as_ref().unwrap()).unwrap();
            let B: MultiPolygon<f64> =
                MultiPolygon::try_from_wkt_str(&self.B.as_ref().unwrap()).unwrap();
            return Ok(A.intersection(&B).to_wkt().to_string());
        } else {
            return Err("unknown operation".to_string());
        }
        Ok("Point NULL".to_string())
    }
}

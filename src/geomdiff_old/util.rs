// use std::{collections::HashMap, ops::Not};

// use geo_types::{Coord, Geometry};

// struct CoordDelta {
//     x: Option<f64>,
//     y: Option<f64>,
// }

// impl CoordDelta {
//     pub fn new(x: Option<f64>, y: Option<f64>) -> Self {
//         CoordDelta { x: x, y: y }
//     }

//     pub fn patch(&self, coord: Coord) -> Coord {
//         (
//             coord.x + self.x.unwrap_or(0.0),
//             coord.y + self.y.unwrap_or(0.0),
//         )
//             .into()
//     }

//     pub fn create(
//         old_coordinate: Option<Coord>,
//         new_coordinate: Option<Coord>,
//     ) -> Result<CoordDelta, &'static str> {
//         match (old_coordinate, new_coordinate) {
//             (Some(old_coord), Some(new_coord)) => Ok(CoordDelta::new(
//                 Some(new_coord.x - old_coord.x),
//                 Some(new_coord.y - old_coord.y),
//             )),
//             (None, Some(new_coord)) => Ok(new_coord.into()),
//             (Some(old_coord), None) => Ok(old_coord.into()),
//             (None, None) => Err("Cannot create a diff from two empty coordinates!"),
//         }
//     }
// }

// impl From<Coord> for CoordDelta {
//     fn from(coord: Coord) -> Self {
//         CoordDelta {
//             x: Some(coord.x),
//             y: Some(coord.y),
//         }
//     }
// }

// impl Not for CoordDelta {
//     type Output = Self;

//     fn not(self) -> Self::Output {
//         CoordDelta {
//             x: match self.x {
//                 Some(val) => Some(-val),
//                 None => None,
//             },
//             y: match self.y {
//                 Some(val) => Some(-val),
//                 None => None,
//             },
//         }
//     }
// }

// pub trait IDiff {
//     fn index(&self) -> i32;
//     fn set_index(&mut self, index: i32);
//     fn operation(&self) -> Operation;
//     fn set_operation(&mut self, operation: Operation);
//     fn geometry_type(&self) -> &str;
//     fn apply(&self, geometry: Geometry) -> Geometry;
//     fn undo(&self, geometry: Geometry) -> Geometry;
//     fn reverse(&self, index: Option<i32>) -> dyn IDiff;
// }

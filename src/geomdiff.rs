// use geo_types::{Geometry, Point, Polygon, };

// #[derive(Debug)]
// pub struct Diff {
//     operation: Operation,
//     index: Option<i64>,
//     new_value: Option<Geometry>,
//     old_value: Option<Geometry>,
// }

// impl Diff {
//     pub fn new(op: Operation, idx: i64, old: Geometry, new: Geometry) -> Self {
//         Self {
//             operation: op,
//             index: Some(idx),
//             new_value: Some(old),
//             old_value: Some(new),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum Operation {
//     Noop,
//     Modify,
//     Insert,
//     Delete,
// }

// pub trait GeomDiff: Sized {
//     fn diff(&self, new: &Self) -> Diff;
// }

// impl GeomDiff for Point {
//     fn diff(&self, new: Self) -> Diff {
//         match (self.is_empty(), new.is_empty()) {
//             (false, false) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
//             (false, true) => Diff::new(Operation::Insert, 0, (*self).into(), new.into()),
//             (true, false) => Diff::new(Operation::Delete, 0, (*self).into(), new.into()),
//             (true, true) => Diff::new(Operation::Noop, 0, (*self).into(), new.into()),
//         }
//     }
// }

// // pub trait GeomDiff: Sized + HasDimensions where Geometry: From<Self> {
// //     fn diff(&self, new: Self) -> Diff {
// //         match (self.is_empty(), new.is_empty()) {
// //             (false, false) => self.default_modify(new),
// //             (false, true) => self.default_insert(new),
// //             (true, false) => self.default_delete(new),
// //             (true, true) => self.default_noop(new),
// //         }
// //     }

// //     fn default_modify(&self, new: Self) -> Diff;
    
// //     fn default_insert(&self, new: Self) -> Diff {
// //         Diff::new(Operation::Insert, 0, (*self).into(), new.into())
// //     }
// //     fn default_delete(&self, new: Self) -> Diff {
// //         Diff::new(Operation::Delete, 0, (*self).into(), new.into())
// //     }
// //     fn default_noop(&self, new: Self) -> Diff {
// //         Diff::new(Operation::Noop, 0, (*self).into(), new.into())
// //     }
// // }


// // impl GeomDiff for Point {
// //     // fn diff(&self, new: Self) -> Diff {
// //     //     match (self.is_empty(), new.is_empty()) {
// //     //         (false, false) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
// //     //         (false, true) => Diff::new(Operation::Insert, 0, (*self).into(), new.into()),
// //     //         (true, false) => Diff::new(Operation::Delete, 0, (*self).into(), new.into()),
// //     //         (true, true) => Diff::new(Operation::Noop, 0, (*self).into(), new.into()),
// //     //     }
// //     // }
// //     fn default_modify(&self, new: Self) -> Diff {
// //         Diff::new(Operation::Modify, 0, (*self).into(), new.into())
// //     }

// // }

// // // impl GeomDiff for MultiPoint {
// // //     fn diff(&self, new: Self) -> Diff {
// // //         match (self.is_empty(), new.is_empty()) {
// // //             (false, false) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
// // //             (false, true) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
// // //             (true, false) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
// // //             (true, true) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
// // //         }
// // //     }
// // // }

// // // impl GeomDiff for Polygon {
// // //     fn diff(&self, new: Self) -> Diff {
// // //         match (self.is_empty(), new.is_empty()) {
// // //             (false, false) => Diff::new(Operation::Modify, 0, (*self).into(), new.into()),
// // //             (false, true) => Diff::new(Operation::Insert, 0, (*self).into(), new.into()),
// // //             (true, false) => Diff::new(Operation::Delete, 0, (*self).into(), new.into()),
// // //             (true, true) => Diff::new(Operation::Noop, 0, (*self).into(), new.into()),
// // //         }
// // //     }
// // // }



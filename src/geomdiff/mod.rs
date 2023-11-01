use std::{any::type_name, collections::HashMap};

use geo_types::{
    CoordNum, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, Rect, Triangle,
};

pub struct GeometryDiffer {
    differ: Differ,
}

impl GeometryDiffer {
    pub fn diff(
        &self,
        old_geometry: Option<Geometry>,
        new_geometry: Option<Geometry>,
    ) -> Result<Option<Box<(dyn IDiff + 'static)>>, String> {
        self.differ.create_diff(old_geometry, new_geometry)
    }
    pub fn patch(&self, old_geometry: Option<Geometry>, patch: Box<dyn IDiff>) -> Option<Geometry> {
        patch.apply(old_geometry)
    }
    pub fn unpatch(
        &self,
        old_geometry: Option<Geometry>,
        patch: Box<dyn IDiff>,
    ) -> Option<Geometry> {
        patch.undo(old_geometry)
    }
}

pub enum Operation {
    Noop,
    Modify,
    Insert,
    Delete,
}

pub struct Change {
    previous_value: Option<Geometry>,
    new_value: Option<Geometry>,
    operation: Operation,
    index: i64,
}

impl Change {
    pub fn new(
        previous_value: Option<Geometry>,
        new_value: Option<Geometry>,
        operation: Operation,
        index: i64,
    ) -> Self {
        Change {
            previous_value,
            new_value,
            operation,
            index,
        }
    }
    pub fn get_value(&self) -> Option<Geometry> {
        match self.operation {
            Operation::Delete => self.new_value.to_owned(),
            _ => self.previous_value.to_owned(),
        }
    }
}
pub trait IDiffer {
    fn create_diff(&self, change: Change) -> Box<(dyn IDiff + 'static)>;
}

pub struct Differ {
    differs: HashMap<String, Box<dyn IDiffer>>,
}

fn inner_type_name<T>(geometry: Geometry<T>) -> String
where
    T: CoordNum,
{
    match geometry {
        Geometry::Point(_) => type_name::<Point<T>>(),
        Geometry::Line(_) => type_name::<Line<T>>(),
        Geometry::LineString(_) => type_name::<LineString<T>>(),
        Geometry::Polygon(_) => type_name::<Polygon<T>>(),
        Geometry::MultiPoint(_) => type_name::<MultiPoint<T>>(),
        Geometry::MultiLineString(_) => type_name::<MultiLineString<T>>(),
        Geometry::MultiPolygon(_) => type_name::<MultiPolygon<T>>(),
        Geometry::GeometryCollection(_) => type_name::<GeometryCollection<T>>(),
        Geometry::Rect(_) => type_name::<Rect<T>>(),
        Geometry::Triangle(_) => type_name::<Triangle<T>>(),
    }
}

impl Differ {
    pub fn new() -> Self {
        let mut differs = HashMap::new();
        // differs.insert("Point", Box::new(PointDiffer {}));
        // differs.insert("LineString".to_string(), Box::new(LineStringDiffer {}));
        // differs.insert("Polygon".to_string(), Box::new(PolygonDiffer {}));
        // differs.insert("MultiPoint".to_string(), Box::new(MultiPointDiffer {}));
        // differs.insert(
        //     "MultiLineString".to_string(),
        //     Box::new(MultiLineStringDiffer {}),
        // );
        // differs.insert("MultiPolygon".to_string(), Box::new(MultiPolygonDiffer {}));
        Differ { differs }
    }

    pub fn create_diff(
        &self,
        old_geometry: Option<Geometry>,
        new_geometry: Option<Geometry>,
    ) -> Result<Option<Box<(dyn IDiff + 'static)>>, String> {
        match (old_geometry, new_geometry) {
            (None, None) => Ok(None),
            (old, new) => {
                if (!Differ::geometries_equal(old, new)) {
                    Err(format!("Mismatched types: {:#?} & {:#?}", old, new).to_string())
                } else {
                    let geom_type = if (old.is_some()) {
                        inner_type_name(old.unwrap())
                    } else if (new.is_some()) {
                        inner_type_name(new.unwrap())
                    } else {
                        "none".to_string()
                    };

                    if (!self.differs.contains_key(&geom_type)) {
                        Err(format!("Geometry type not supported: {}", geom_type).to_string())
                    } else {
                        let differ = self.differs.get(&geom_type).unwrap();
                        Ok(Some(differ.create_diff(Change::new(
                            old,
                            new,
                            Differ::get_operation(old, new),
                            0,
                        ))))
                    }
                }
            }
        }
    }

    fn geometries_equal(geometry1: Option<Geometry>, geometry2: Option<Geometry>) -> bool {
        geometry1.is_none() || geometry2.is_none() || geometry1 == geometry2
    }

    pub fn get_operation(
        old_geometry: Option<Geometry>,
        new_geometry: Option<Geometry>,
    ) -> Operation {
        match (old_geometry, new_geometry) {
            (Some(_), Some(_)) => Operation::Modify,
            (None, Some(_)) => Operation::Insert,
            _ => Operation::Delete,
        }
    }
}

pub trait IDiff {
    fn apply(&self, geometry: Option<Geometry>) -> Option<Geometry>;
    fn undo(&self, geometry: Option<Geometry>) -> Option<Geometry>;

    fn index(&self) -> i32;
    fn set_index(&mut self, index: i32);
    fn operation(&self) -> Operation;
    fn set_operation(&mut self, operation: Operation);
    fn geometry_type(&self) -> &str;
    fn reverse(&self, index: Option<i32>) -> dyn IDiff;
}

pub trait IDiffWithValue<TDiffedComponent>: IDiff {
    fn value(&self) -> TDiffedComponent;
}

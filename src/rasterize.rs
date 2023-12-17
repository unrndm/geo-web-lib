use crate::line::rasterize_line;
use crate::polygon::rasterize_polygon;

use crate::Rasterizer;
use geo::{
    Geometry, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon,
};

pub trait Rasterize<T> {
    fn rasterize(&self, rasterizer: &mut Rasterizer);
}

impl Rasterize<f64> for Point<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        let x = self.x().floor() as usize;
        let y = self.y().floor() as usize;
        rasterizer.fill_pixel(x, y);
    }
}

impl Rasterize<f64> for MultiPoint<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        self.iter().for_each(|point| point.rasterize(rasterizer));
    }
}

impl Rasterize<f64> for LineString<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        self.lines().for_each(|line| {
            rasterize_line(&line, rasterizer);
        });
    }
}

impl Rasterize<f64> for MultiLineString<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        self.iter()
            .for_each(|line_string| line_string.rasterize(rasterizer));
    }
}

impl Rasterize<f64> for Polygon<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        rasterize_polygon(self.exterior(), self.interiors(), rasterizer);
    }
}

impl Rasterize<f64> for MultiPolygon<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        self.iter().for_each(|poly| poly.rasterize(rasterizer));
    }
}

impl Rasterize<f64> for Geometry<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        match self {
            Geometry::Point(point) => point.rasterize(rasterizer),
            Geometry::MultiPoint(points) => points.rasterize(rasterizer),
            Geometry::LineString(line) => line.rasterize(rasterizer),
            Geometry::MultiLineString(lines) => lines.rasterize(rasterizer),
            Geometry::Polygon(polygon) => polygon.rasterize(rasterizer),
            Geometry::MultiPolygon(polygonons) => polygonons.rasterize(rasterizer),
            Geometry::GeometryCollection(geoms) => geoms.rasterize(rasterizer),
            _ => unreachable!("This geometry type is not supported"),
        }
    }
}

impl Rasterize<f64> for GeometryCollection<f64> {
    fn rasterize(&self, rasterizer: &mut Rasterizer) {
        self.iter().for_each(|geom| geom.rasterize(rasterizer));
    }
}

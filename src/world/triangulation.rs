use bevy::prelude::*;
use spade::{DelaunayTriangulation, HasPosition, Point2};

#[derive(Resource)]
pub struct MapTriangulation {
    pub triangulation: DelaunayTriangulation<Point>
}

pub struct Point {
    pub pos: Vec2,
}

impl HasPosition for Point {
    type Scalar = f32;

    fn position(&self) -> Point2<Self::Scalar> {
        Point2::new(self.pos.x, self.pos.y)
    }
}
use bevy::math::Vec2;
use spade::{HasPosition, Point2};

pub struct Point{
    pub pos: Vec2
}

impl HasPosition for Point{
    type Scalar = f32;

    fn position(&self) -> Point2<Self::Scalar> {
        return Point2{x:self.pos.x,y:self.pos.y};
    }
}

impl Clone for Point{
    fn clone(&self) -> Self {
        return Point{
            pos: self.pos
        }
    }
}
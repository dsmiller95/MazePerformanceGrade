use godot::prelude::*;

pub trait SimpleBound {
    fn in_bounds(&self, other: Self) -> bool;
}

impl SimpleBound for Vector2i {
    fn in_bounds(&self, other: Self) -> bool {
        if other.x < 0 || other.y < 0 {
            return false;
        }

        if other.x >= self.x || other.y >= self.y {
            return false;
        }

        return true;
    }
}

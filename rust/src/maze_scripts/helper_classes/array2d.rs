use crate::maze_scripts::helper_classes::simple_bound::SimpleBound;
use godot::prelude::*;
use std::ops::{Index, IndexMut};

pub struct Array2D<T> {
    data: Box<[T]>,
    pub size: Vector2i,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    pub fn new(size: Vector2i, default: T) -> Self {
        let data_size = (size.x * size.y) as usize;
        let data = vec![default; data_size].into_boxed_slice();
        Self { data, size }
    }
}
impl<T> Array2D<T> {
    pub fn in_bounds(&self, cell: Vector2i) -> bool {
        self.size.in_bounds(cell)
    }
}

impl<T> Index<Vector2i> for Array2D<T> {
    type Output = T;

    fn index(&self, index: Vector2i) -> &Self::Output {
        if !self.in_bounds(index) {
            panic!("index bound out of range")
        }

        let index = index.x + index.y * self.size.x;
        &self.data[index as usize]
    }
}

impl<T> IndexMut<Vector2i> for Array2D<T> {
    fn index_mut(&mut self, index: Vector2i) -> &mut Self::Output {
        if !self.in_bounds(index) {
            panic!("index bound out of range")
        }

        let index = index.x + index.y * self.size.x;
        &mut self.data[index as usize]
    }
}

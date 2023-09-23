use std::ops::{Index, IndexMut};
use godot::prelude::*;

pub struct Array2D<T> {
    data: Box<[T]>,
    pub size: Vector2i,
}


impl<T> Array2D<T> where T: Clone{
    pub fn new(size: Vector2i, default: T) -> Self {
        let data_size = (size.x * size.y) as usize;
        let data = vec![default; data_size].into_boxed_slice();
        Self {
            data,
            size
        }
    }
}
impl<T> Array2D<T>{
    pub fn in_bounds(&self, cell: Vector2i) -> bool{
        if cell.x < 0 || cell.y < 0 {
            return false;
        }
        if cell.x >= self.size.x || cell.y >= self.size.x {
            return false;
        }
        return true;
    }
}

impl<T> Index<Vector2i> for Array2D<T>{
    type Output = T;

    fn index(&self, index: Vector2i) -> &Self::Output {
        if !self.in_bounds(index) {
            panic!("index bound out of range")
        }

        let index = index.x + index.y * self.size.x;
        &self.data[index as usize]
    }
}

impl<T> IndexMut<Vector2i> for Array2D<T>{
    fn index_mut(&mut self, index: Vector2i) -> &mut Self::Output {
        if !self.in_bounds(index) {
            panic!("index bound out of range")
        }

        let index = index.x + index.y * self.size.x;
        &mut self.data[index as usize]
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_iterate_size() {
        let iterated_size: Vec<Vector2i> = (Vector2i::new(0, 0)..Vector2i::new(3, 3)).collect();
        let expected_sequence = [
            Vector2i::new(0, 0),
            Vector2i::new(1, 0),
            Vector2i::new(2, 0),
            Vector2i::new(0, 1),
            Vector2i::new(1, 1),
            Vector2i::new(2, 1),
            Vector2i::new(0, 2),
            Vector2i::new(1, 2),
            Vector2i::new(2, 2),
        ];

        assert!(itertools::equal(expected_sequence, iterated_size));
    }
}
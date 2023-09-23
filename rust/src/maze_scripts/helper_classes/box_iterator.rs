use std::cmp::{max, min};
use std::ops::{Range};
use godot::prelude::*;


pub struct BoxIterator {
    base: Vector2i,
    extent: Vector2i,
    last_extent: Vector2i
}

impl Iterator for BoxIterator{
    type Item = Vector2i;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_emit = self.last_extent + Vector2i::new(1, 0);
        if next_emit.x >= self.extent.x {
            next_emit.x = 0;
            next_emit.y += 1;
        }
        if next_emit.y >= self.extent.y {
            None
        } else {
            self.last_extent = next_emit;
            Some(next_emit + self.base)
        }
    }
}


impl From<Range<Vector2i>> for BoxIterator{
    fn from(value: Range<Vector2i>) -> Self {
        let min_x = min(value.start.x, value.end.x);
        let min_y = min(value.start.y, value.end.y);
        let min_vec = Vector2i::new(min_x, min_y);
        let max_x = max(value.start.x, value.end.x);
        let max_y = max(value.start.y, value.end.y);
        let max_vec = Vector2i::new(max_x, max_y);
        BoxIterator{
            base: min_vec,
            extent: max_vec - min_vec,
            last_extent: Vector2i::new(-1, 0)
        }
    }
}

impl From<Vector2i> for BoxIterator{
    fn from(value: Vector2i) -> Self {
        BoxIterator::from(Vector2i::ZERO..value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_1_by_1() {
        let iterated_size: BoxIterator = (Vector2i::new(0, 0)..Vector2i::new(1, 1)).into();
        let iterated_collected: Vec<Vector2i> = iterated_size.collect();
        let expected_sequence = [
            Vector2i::new(0, 0),
        ];

        assert_eq!(expected_sequence.len(), iterated_collected.len());
        assert!(itertools::equal(expected_sequence, iterated_collected));
    }

    #[test]
    fn test_iterate_3_by_3() {
        let iterated_size: BoxIterator = (Vector2i::new(0, 0)..Vector2i::new(3, 3)).into();
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
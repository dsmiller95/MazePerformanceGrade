use godot::engine::RandomNumberGenerator;
use godot::prelude::*;

pub trait RngExtensions {
    fn random_vector(&mut self, size: Vector2i) -> Vector2i;
    fn shuffle_naive<T>(&mut self, arr: &mut [T]);
}

impl RngExtensions for RandomNumberGenerator {
    fn random_vector(&mut self, size: Vector2i) -> Vector2i {
        let x = self.randi_range(0, size.x - 1);
        let y = self.randi_range(0, size.y - 1);
        Vector2i { x, y }
    }

    fn shuffle_naive<T>(&mut self, arr: &mut [T]) {
        for i in 0..arr.len() {
            let swap = self.randi_range(i as i32, (arr.len() - 1) as i32) as usize;
            arr.swap(i, swap);
        }
    }
}
use godot::prelude::*;
use crate::maze_scripts::helper_classes::array2d::Array2D;
use crate::maze_scripts::helper_classes::tile_edge::TileEdge;

pub const NEIGHBORS: [Vector2i; 4] = [Vector2i::UP, Vector2i::RIGHT, Vector2i::DOWN, Vector2i::LEFT];

pub struct Reachability {
    reachable: Array2D<bool>,
    navigable_edges: Vec<TileEdge>,
}

impl Reachability{
    pub fn new(size: Vector2i) -> Self {
        Self{
            reachable: Array2D::<bool>::new(size, false),
            navigable_edges: vec![]
        }
    }

    pub fn in_bounds(&self, cell: Vector2i) -> bool {
        if cell.x < 0 || cell.y < 0 {
            return false;
        }

        if cell.x >= self.reachable.size.x || cell.y >= self.reachable.size.y {
            return false;
        }

        return true;
    }

    pub fn reachable(&self, cell: Vector2i) -> bool {
        self.reachable[cell]
    }

    pub fn reach_between(&mut self, reached: Vector2i, new_reached: Vector2i){
        if !self.reachable[reached] {
            godot_error!("trying to reach from unreached cell");
        }
        if self.reachable[new_reached] {
            godot_error!("reaching into an already reached cell");
        }

        self.reachable[new_reached] = true;
        self.navigable_edges.push(TileEdge::new(reached, new_reached));
    }

    pub fn assert_fully_reachable(&self){
        for x in 0..self.reachable.size.x {
            for y in 0..self.reachable.size.y {
                let pos = Vector2i{x, y};
                if !self.reachable[pos] {
                    godot_error!("Cell {} not reachable when assumed reachable!", pos)
                }
            }
        }
    }

    pub fn get_walls(&self) -> Vec<TileEdge> {
        let all_possible_walls = all_edges(self.reachable.size);
        let walls = all_possible_walls
            .filter(|w| !self.navigable_edges.contains(w))
            .collect();
        walls
    }
}

pub fn all_edges(size: Vector2i) -> impl Iterator<Item = TileEdge> {
    let mut edges = vec![];
    for x in 0..size.x - 1 {
        for y in 0..size.y {
            edges.push(TileEdge::new(
                Vector2i::new(x, y),
                Vector2i::new(x + 1, y)
            ))
        }
    }

    for x in 0..size.x {
        for y in 0..size.y - 1 {
            edges.push(TileEdge::new(
                Vector2i::new(x, y),
                Vector2i::new(x, y + 1)
            ))
        }
    }
    edges.into_iter()
}

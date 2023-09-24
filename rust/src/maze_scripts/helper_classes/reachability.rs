use crate::maze_scripts::helper_classes::array2d::Array2D;
use crate::maze_scripts::helper_classes::box_iterator::BoxIterator;
use crate::maze_scripts::helper_classes::simple_bound::SimpleBound;
use crate::maze_scripts::helper_classes::tile_edge::TileEdge;
use godot::prelude::*;

pub const NEIGHBORS: [Vector2i; 4] = [
    Vector2i::UP,
    Vector2i::RIGHT,
    Vector2i::DOWN,
    Vector2i::LEFT,
];

pub struct Reachability {
    reachable: Array2D<bool>,
    navigable_edges: Vec<TileEdge>,
}

impl Reachability {
    pub fn new(size: Vector2i) -> Self {
        Self {
            reachable: Array2D::<bool>::new(size, false),
            navigable_edges: vec![],
        }
    }

    pub fn size(&self) -> Vector2i {
        self.reachable.size
    }

    pub fn in_bounds(&self, cell: Vector2i) -> bool {
        return self.reachable.size.in_bounds(cell);
    }

    pub fn reachable(&self, cell: Vector2i) -> bool {
        self.reachable[cell]
    }
    pub fn traversable(&self, edge: &TileEdge) -> bool {
        self.navigable_edges.contains(edge)
    }

    pub fn reach_between(&mut self, reached: Vector2i, new_reached: Vector2i) {
        if !self.reachable[reached] {
            godot_error!("trying to reach from unreached cell");
        }
        if self.reachable[new_reached] {
            godot_error!("reaching into an already reached cell");
        }

        self.reachable[new_reached] = true;
        self.navigable_edges
            .push(TileEdge::new(reached, new_reached));
    }

    pub fn assert_fully_reachable(&self) {
        for pos in BoxIterator::from(self.reachable.size) {
            if !self.reachable[pos] {
                godot_error!("Cell {} not reachable when assumed reachable!", pos)
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
    for tile in BoxIterator::from(size - Vector2i::new(1, 0)) {
        edges.push(TileEdge::new(tile, tile + Vector2i::new(1, 0)))
    }

    for tile in BoxIterator::from(size - Vector2i::new(0, 1)) {
        edges.push(TileEdge::new(tile, tile + Vector2i::new(0, 1)))
    }
    edges.into_iter()
}

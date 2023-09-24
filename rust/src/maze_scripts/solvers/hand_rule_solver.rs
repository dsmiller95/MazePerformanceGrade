use crate::maze_scripts::helper_classes::historic_position::HistoricPosition;
use crate::maze_scripts::helper_classes::reachability::{Reachability, NEIGHBORS};
use crate::maze_scripts::helper_classes::tile_edge::TileEdge;
use godot::prelude::utilities::pow;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct HandRuleSolverRs {
    #[export]
    pub keep_right: bool,
}

#[godot_api]
impl NodeVirtual for HandRuleSolverRs {
    fn init(_: Base<Self::Base>) -> Self {
        Self { keep_right: false }
    }
}

#[godot_api]
impl HandRuleSolverRs {
    pub fn solve_maze(
        &self,
        maze: &Reachability,
        mut position: Vector2i,
        mut direction: usize,
        target: Vector2i,
    ) -> Vec<HistoricPosition> {
        let mut path = vec![];
        let delay_ms = 500;
        let mut current_step = 0 as u64;
        let size = maze.size();
        let max_steps = pow((size.x * size.y) as f64, 2.0) as u64;

        let check_dir = if self.keep_right { 1 } else { 3 };
        let search_dir = (check_dir + 2) % 4;

        path.push(HistoricPosition {
            tile: position,
            time_ms: current_step * delay_ms,
        });

        while position != target && current_step < max_steps {
            let forward_tile = position + NEIGHBORS[direction];
            let forward_edge = TileEdge::new(position, forward_tile);
            let can_move = maze.in_bounds(forward_tile) && maze.traversable(&forward_edge);

            if can_move {
                position = forward_tile;
                direction = (direction + check_dir) % 4;
                path.push(HistoricPosition {
                    tile: position,
                    time_ms: current_step * delay_ms,
                });
            } else {
                direction = (direction + search_dir) % 4;
            }

            current_step += 1;
        }

        path
    }
}

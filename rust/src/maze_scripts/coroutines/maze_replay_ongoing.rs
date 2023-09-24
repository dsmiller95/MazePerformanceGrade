use crate::assert_some::clone_some_or_log_err_none;
use crate::maze_scripts::coroutines::wait::Wait;
use crate::maze_scripts::floor_creator::FloorCreatorRs;
use crate::maze_scripts::helper_classes::historic_position::HistoricPosition;
use crate::maze_scripts::maze_config::MazeConfigRs;
use crate::maze_scripts::maze_replay::MazeReplayRs;
use godot::engine::{Material, MeshInstance3D};
use godot::prelude::*;
use std::collections::VecDeque;
use std::task::Poll;
use std::task::Poll::{Pending, Ready};

pub(crate) struct MazeReplayOngoing {
    pub highlight_material: Gd<Material>,
    pub traveled_material: Gd<Material>,
    pub maze_config: Gd<MazeConfigRs>,
    pub floors: Gd<FloorCreatorRs>,

    last_position: Option<HistoricPosition>,
    remaining_positions: VecDeque<HistoricPosition>,
    next_action_ms: u64,
    time_scale: f64,
}

impl MazeReplayOngoing {
    pub fn try_new(
        context: &mut MazeReplayRs,
        path: VecDeque<HistoricPosition>,
        current_time: u64,
        time_scale: f64,
    ) -> Option<Self> {
        clone_some_or_log_err_none!(highlight_material, context);
        clone_some_or_log_err_none!(traveled_material, context);
        clone_some_or_log_err_none!(maze_config, context);
        clone_some_or_log_err_none!(floors, context);

        Some(Self {
            traveled_material,
            highlight_material,
            maze_config,
            floors,

            last_position: None,
            remaining_positions: path,
            next_action_ms: current_time,
            time_scale,
        })
    }

    pub fn try_move(&mut self, current_ms: u64) -> Poll<()> {
        if current_ms < self.next_action_ms {
            return Pending;
        }

        match self.next_move() {
            Some(Wait::DelayMs(delay)) => {
                self.next_action_ms += delay;
                Pending
            }
            None => Ready(()),
        }
    }
    fn next_move(&mut self) -> Option<Wait> {
        let Some(current_pos) = self.remaining_positions.pop_front() else {
            return None;
        };

        self.highlight_tile(current_pos.tile, self.highlight_material.clone());

        let mut delay = 0;
        if let Some(next_pos) = self.remaining_positions.front() {
            delay = next_pos.time_ms - current_pos.time_ms;
        }

        if let Some(last_pos) = &self.last_position {
            self.highlight_tile(last_pos.tile, self.traveled_material.clone());
        }

        self.last_position = Some(current_pos);
        Some(Wait::DelayMs((delay as f64 * self.time_scale) as u64))
    }

    pub fn highlight_tile(&mut self, tile: Vector2i, material: Gd<Material>) {
        godot_print!("replaying over tile {}", tile);

        let size = self.maze_config.bind().size;
        let floors = self.floors.bind_mut();
        let floor = floors
            .floors_indexed
            .get((tile.x + tile.y * size.x) as usize);
        let mesh = floor
            .find_child("MeshInstance3D".into())
            .and_then(|x| x.try_cast::<MeshInstance3D>());
        match mesh {
            None => {
                godot_error!("Could not find mesh for floor");
            }
            Some(mut mesh) => {
                mesh.set_material_override(material);
            }
        }
    }
}

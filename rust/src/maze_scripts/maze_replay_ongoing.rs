use crate::maze_scripts::helper_classes::historic_position::HistoricPosition;
use crate::maze_scripts::maze_replay::MazeReplayRs;
use godot::prelude::*;
use std::collections::VecDeque;
use std::task::Poll;
use std::task::Poll::{Pending, Ready};

pub(crate) struct MazeReplayOngoing {
    last_position: Option<HistoricPosition>,
    remaining_positions: VecDeque<HistoricPosition>,
    next_action_ms: u64,
}

enum Wait {
    DelayMs(u64),
    Done,
}

impl MazeReplayOngoing {
    pub fn new(path: VecDeque<HistoricPosition>, current_time: u64) -> Self {
        Self {
            last_position: None,
            remaining_positions: path,
            next_action_ms: current_time,
        }
    }

    pub fn try_move(&mut self, current_ms: u64, context: &mut MazeReplayRs) -> Poll<()> {
        if current_ms < self.next_action_ms {
            return Pending;
        }

        match self.next_move(context) {
            Wait::DelayMs(delay) => {
                self.next_action_ms += delay;
                Pending
            }
            Wait::Done => Ready(()),
        }
    }
    fn next_move(&mut self, context: &mut MazeReplayRs) -> Wait {
        let Some(traveled_material) = context.traveled_material.as_ref().map(|x| x.clone()) else {
            godot_error!("{} is required!", "traveled_material");
            return Wait::Done;
        };

        let Some(highlight_material) = context.highlight_material.as_ref().map(|x| x.clone())
        else {
            godot_error!("{} is required!", "highlight_material");
            return Wait::Done;
        };

        let Some(next_pos) = self.remaining_positions.pop_front() else {
            return Wait::Done;
        };

        context.highlight_tile(next_pos.tile, highlight_material);

        let mut delay = 0;
        if let Some(last_pos) = &self.last_position {
            context.highlight_tile(last_pos.tile, traveled_material);
            delay = next_pos.time_ms - last_pos.time_ms;
        }

        self.last_position = Some(next_pos);
        Wait::DelayMs(delay)
    }
}

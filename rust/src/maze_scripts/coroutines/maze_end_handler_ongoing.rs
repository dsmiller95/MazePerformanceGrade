use crate::assert_some::clone_some_or_log_err_none;
use crate::maze_scripts::coroutines::maze_end_handler_ongoing::MazeEndHandlerState::{
    PlayerReplay, SolverReplay,
};
use crate::maze_scripts::coroutines::maze_replay_ongoing::MazeReplayOngoing;
use crate::maze_scripts::maze_end_handler::MazeEndHandlerRs;
use crate::maze_scripts::maze_replay::MazeReplayRs;
use godot::prelude::*;
use std::task::Poll;
use std::task::Poll::*;

pub(crate) struct MazeEndHandlerOngoing {
    state: MazeEndHandlerState,
    context: MazeEndHandlerOngoingContext,
}

struct MazeEndHandlerOngoingContext {
    pub maze_replay: Gd<MazeReplayRs>,
}
enum MazeEndHandlerState {
    PlayerReplay(MazeReplayOngoing),
    SolverReplay(MazeReplayOngoing, u8),
}

impl MazeEndHandlerOngoing {
    pub fn try_new(context: &mut MazeEndHandlerRs) -> Option<Self> {
        clone_some_or_log_err_none!(maze_replay, context);

        let mut inner_context = MazeEndHandlerOngoingContext { maze_replay };

        inner_context.generate_first(context).map(|x| Self {
            state: x,
            context: inner_context,
        })
    }

    pub fn try_move(&mut self, current_ms: u64, context: &mut MazeEndHandlerRs) -> Poll<()> {
        let ongoing = match &mut self.state {
            PlayerReplay(ongoing) => ongoing,
            SolverReplay(ongoing, _) => ongoing,
        };

        let moved = ongoing.try_move(current_ms);

        if moved == Pending {
            return Pending;
        }

        let next_state = self.context.generate_next(&self.state, context);

        match next_state {
            None => Ready(()),
            Some(next_state) => {
                self.state = next_state;
                Pending
            }
        }
    }
}

impl MazeEndHandlerOngoingContext {
    fn generate_first(&mut self, context: &mut MazeEndHandlerRs) -> Option<MazeEndHandlerState> {
        let player_path_history = context.player_tracker.as_ref()?.bind().path_history.clone();
        let ongoing = self.maze_replay.bind_mut().try_get_path_replay_ongoing(
            player_path_history,
            (context.replay_time_seconds * 1000.0) as f64,
        );
        ongoing.map(|x| PlayerReplay(x))
    }

    fn generate_next(
        &mut self,
        prev_state: &MazeEndHandlerState,
        context: &mut MazeEndHandlerRs,
    ) -> Option<MazeEndHandlerState> {
        match prev_state {
            PlayerReplay(_) => {
                if context.solvers.is_empty() {
                    None
                } else {
                    let player_path_history =
                        context.player_tracker.as_ref()?.bind().path_history.clone();
                    let ongoing = self.maze_replay.bind_mut().try_get_path_replay_ongoing(
                        player_path_history,
                        (context.replay_time_seconds * 1000.0) as f64,
                    );
                    ongoing.map(|x| SolverReplay(x, 0))
                }
            }
            SolverReplay(_, idx) => {
                // TODO
                None
            }
        }
    }
}

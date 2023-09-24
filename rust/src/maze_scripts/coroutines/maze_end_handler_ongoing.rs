use crate::assert_some::clone_some_or_log_err_none;
use crate::maze_scripts::coroutines::maze_end_handler_ongoing::MazeEndHandlerState::{
    PlayerReplay, SolverReplay,
};
use crate::maze_scripts::coroutines::maze_replay_ongoing::MazeReplayOngoing;
use crate::maze_scripts::maze_config::MazeConfigRs;
use crate::maze_scripts::maze_end_handler::MazeEndHandlerRs;
use crate::maze_scripts::maze_replay::MazeReplayRs;
use crate::maze_scripts::solvers::hand_rule_solver::HandRuleSolverRs;
use crate::maze_scripts::wall_creator::WallCreatorRs;
use godot::engine::RichTextLabel;
use godot::prelude::*;
use std::task::Poll;
use std::task::Poll::*;

pub(crate) struct MazeEndHandlerOngoing {
    state: MazeEndHandlerState,
    context: MazeEndHandlerOngoingContext,
}

struct MazeEndHandlerOngoingContext {
    pub maze_replay: Gd<MazeReplayRs>,
    pub wall_creator: Gd<WallCreatorRs>,
    pub maze_config: Gd<MazeConfigRs>,
    pub score_text: Gd<RichTextLabel>,
    pub solvers: Array<Gd<HandRuleSolverRs>>,
    pub replay_time_seconds: real,
}
enum MazeEndHandlerState {
    PlayerReplay(MazeReplayOngoing),
    SolverReplay(MazeReplayOngoing, u8),
}

impl MazeEndHandlerOngoing {
    pub fn try_new(context: &mut MazeEndHandlerRs) -> Option<Self> {
        clone_some_or_log_err_none!(maze_replay, context);
        clone_some_or_log_err_none!(wall_creator, context);
        clone_some_or_log_err_none!(maze_config, context);
        clone_some_or_log_err_none!(score_text, context);

        let mut inner_context = MazeEndHandlerOngoingContext {
            maze_replay,
            wall_creator,
            maze_config,
            score_text,
            solvers: context.solvers.clone(),
            replay_time_seconds: context.replay_time_seconds,
        };

        inner_context.generate_first(context).map(|x| Self {
            state: x,
            context: inner_context,
        })
    }

    pub fn try_move(&mut self, current_ms: u64) -> Poll<()> {
        let ongoing = match &mut self.state {
            PlayerReplay(ongoing) => ongoing,
            SolverReplay(ongoing, _) => ongoing,
        };

        let moved = ongoing.try_move(current_ms);

        if moved == Pending {
            return Pending;
        }

        let next_state = self.context.generate_next(&self.state);

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
        self.score_text
            .append_text(format!("player: {}\n", player_path_history.len()).into());

        self.maze_replay
            .bind_mut()
            .try_get_path_replay_ongoing(
                player_path_history,
                (context.replay_time_seconds * 1000.0) as f64,
            )
            .map(|x| PlayerReplay(x))
    }

    fn generate_next(&mut self, prev_state: &MazeEndHandlerState) -> Option<MazeEndHandlerState> {
        match prev_state {
            PlayerReplay(_) => {
                if self.solvers.is_empty() {
                    None
                } else {
                    let solver = self.solvers.get(0);
                    let ongoing = self.begin_solve(solver);
                    ongoing.map(|x| SolverReplay(x, 0))
                }
            }
            SolverReplay(_, idx) => {
                let next_index = idx + 1;
                if next_index as usize >= self.solvers.len() {
                    None
                } else {
                    let solver = self.solvers.get(next_index as usize);
                    let ongoing = self.begin_solve(solver);
                    ongoing.map(|x| SolverReplay(x, next_index))
                }
            }
        }
    }

    fn begin_solve(&mut self, solver: Gd<HandRuleSolverRs>) -> Option<MazeReplayOngoing> {
        let reachability = &self.wall_creator.bind().reachable;
        let config = self.maze_config.bind();
        let solution =
            solver
                .bind()
                .solve_maze(reachability.as_ref()?, config.entry, 0, config.exit);

        self.score_text
            .append_text(format!("{}: {}\n", solver.get_name(), solution.len()).into());
        self.maze_replay
            .bind_mut()
            .try_get_path_replay_ongoing(solution, (self.replay_time_seconds * 1000.0) as f64)
    }
}

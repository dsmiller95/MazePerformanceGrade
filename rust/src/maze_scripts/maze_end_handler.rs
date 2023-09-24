use crate::maze_scripts::coroutines::maze_end_handler_ongoing::MazeEndHandlerOngoing;
use crate::maze_scripts::maze_config::MazeConfigRs;
use crate::maze_scripts::maze_replay::MazeReplayRs;
use crate::maze_scripts::path_history::PathHistoryRs;
use crate::maze_scripts::solvers::hand_rule_solver::HandRuleSolverRs;
use crate::maze_scripts::wall_creator::WallCreatorRs;
use godot::engine::{RichTextLabel, Time};
use godot::prelude::*;
use std::task::Poll::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct MazeEndHandlerRs {
    #[export]
    maze_cam: Option<Gd<Camera3D>>,
    #[export]
    pub maze_replay: Option<Gd<MazeReplayRs>>,
    #[export]
    pub score_text: Option<Gd<RichTextLabel>>,
    #[export]
    pub maze_config: Option<Gd<MazeConfigRs>>,
    #[export]
    pub wall_creator: Option<Gd<WallCreatorRs>>,
    #[export]
    pub(crate) player_tracker: Option<Gd<PathHistoryRs>>,
    #[base]
    base: Base<Node>,

    #[export]
    pub solvers: Array<Gd<HandRuleSolverRs>>,

    #[export]
    pub(crate) replay_time_seconds: real,

    pending: Option<MazeEndHandlerOngoing>,
}

#[godot_api]
impl NodeVirtual for MazeEndHandlerRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            maze_cam: None,
            maze_replay: None,
            score_text: None,
            maze_config: None,
            wall_creator: None,
            player_tracker: None,
            base,

            solvers: array![],

            replay_time_seconds: real!(5.0),
            pending: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        let time_ms = Time::singleton().get_ticks_msec();

        let replay = self.pending.take();
        if let Some(mut replay) = replay {
            let moved = replay.try_move(time_ms);
            match moved {
                Ready(_) => {
                    self.pending = None;
                    godot_print!("maze end handling done.")
                }
                Pending => {
                    self.pending = Some(replay);
                }
            }
        }
    }
}

#[godot_api]
impl MazeEndHandlerRs {
    #[func]
    pub fn on_body_entered_end_marker(&mut self, _body: Gd<Node3D>) {
        match self.play_end_game() {
            None => {
                godot_error!("could not start end game running")
            }
            Some(_) => {}
        }
    }
}

impl MazeEndHandlerRs {
    fn play_end_game(&mut self) -> Option<()> {
        if let Some(_) = self.pending {
            godot_error!("currently playing end, cannot play another");
            return None;
        }

        self.maze_cam.as_mut()?.make_current();

        self.pending = MazeEndHandlerOngoing::try_new(self);

        Some(())
    }
}

use crate::maze_scripts::maze_config::MazeConfigRs;
use crate::maze_scripts::maze_replay::MazeReplayRs;
use crate::maze_scripts::path_history::PathHistoryRs;
use crate::maze_scripts::wall_creator::WallCreatorRs;
use godot::engine::RichTextLabel;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct MazeEndHandlerRs {
    #[export]
    maze_cam: Option<Gd<Camera3D>>,
    #[export]
    maze_replay: Option<Gd<MazeReplayRs>>,
    #[export]
    score_text: Option<Gd<RichTextLabel>>,
    #[export]
    maze_config: Option<Gd<MazeConfigRs>>,
    #[export]
    wall_creator: Option<Gd<WallCreatorRs>>,
    #[export]
    player_tracker: Option<Gd<PathHistoryRs>>,
    #[base]
    base: Base<Node>,

    #[export]
    replay_time_seconds: real,

    is_end_running: bool,
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

            replay_time_seconds: real!(5.0),
            is_end_running: false,
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
        if self.is_end_running {
            return None;
        }
        self.is_end_running = true;

        self.maze_cam.as_mut()?.make_current();

        let player_path_history = self.player_tracker.as_ref()?.bind().path_history.clone();

        self.maze_replay.as_mut()?.bind_mut().begin_path_replay(
            player_path_history,
            (self.replay_time_seconds * 1000.0) as f64,
        );

        Some(())
    }
}

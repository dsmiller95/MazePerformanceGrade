use crate::assert_some::assert_some_or_log_err;
use crate::maze_scripts::helper_classes::historic_position::HistoricPosition;
use crate::maze_scripts::helper_classes::simple_bound::SimpleBound;
use crate::maze_scripts::maze_config::MazeConfigRs;
use godot::engine::utilities::roundi;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub(crate) struct PathHistoryRs {
    #[export]
    tracked: Option<Gd<Node3D>>,
    #[export]
    maze_config: Option<Gd<MazeConfigRs>>,

    pub path_history: Vec<HistoricPosition>,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl NodeVirtual for PathHistoryRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            tracked: None,
            maze_config: None,
            path_history: vec![],
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let tracked = self.tracked.as_ref().expect("must have tracked");
        let current = self.get_current_tile_position(tracked);
        if self.path_history.is_empty() {
            self.log_next_position(current);
            return;
        }

        let last_tile = self.path_history.last().expect("must be nonempty").tile;
        if last_tile == current {
            return;
        }

        self.log_next_position(current);
    }
}

#[godot_api]
impl PathHistoryRs {
    fn get_current_tile_position(&self, tracked: &Gd<Node3D>) -> Vector2i {
        let local_position = self.base.to_local(tracked.get_global_position());
        Vector2i::new(
            roundi(local_position.x as f64) as i32,
            roundi(local_position.z as f64) as i32,
        )
    }

    fn log_next_position(&mut self, current: Vector2i) {
        assert_some_or_log_err!(maze_config, self);
        //let maze_config = self.maze_config.as_ref().expect("");
        if !maze_config.bind().size.in_bounds(current) {
            return;
        }

        self.path_history.push(HistoricPosition::new(current));
        godot_print!("logged player position: {}", current);
    }
}

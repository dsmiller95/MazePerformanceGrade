use crate::assert_some::assert_some_or_log_err;
use crate::maze_scripts::helper_classes::tile_edge::to_3d_from_2d;
use crate::maze_scripts::maze_config::MazeConfigRs;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub(crate) struct MazeFeaturesRs {
    #[export]
    maze_config: Option<Gd<MazeConfigRs>>,
    #[export]
    exit_feature: Option<Gd<Node3D>>,
    #[export]
    entry_feature: Option<Gd<Node3D>>,
    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl NodeVirtual for MazeFeaturesRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            maze_config: None,
            exit_feature: None,
            entry_feature: None,
            base,
        }
    }

    fn ready(&mut self) {
        assert_some_or_log_err!(maze_config, self);

        if let Some(exit) = &mut self.exit_feature {
            let exit_pos = self.base.to_global(to_3d_from_2d(maze_config.bind().exit));
            exit.set_global_position(exit_pos)
        }
        if let Some(entry) = &mut self.entry_feature {
            let entry_pos = self.base.to_global(to_3d_from_2d(maze_config.bind().entry));
            entry.set_global_position(entry_pos)
        }
    }
}

#[godot_api]
impl MazeFeaturesRs {}

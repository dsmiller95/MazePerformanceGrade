use crate::assert_some::assert_some_or_log_err;
use crate::maze_scripts::helper_classes::tile_edge::to_3d_from_2d;
use crate::maze_scripts::maze_config::MazeConfigRs;
use godot::prelude::*;
use std::cmp::max;

#[derive(GodotClass)]
#[class(base=Camera3D)]
pub(crate) struct MazeCameraRs {
    #[export]
    maze_config: Option<Gd<MazeConfigRs>>,
    #[base]
    base: Base<Camera3D>,
}

#[godot_api]
impl NodeVirtual for MazeCameraRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            maze_config: None,
            base,
        }
    }

    fn ready(&mut self) {
        assert_some_or_log_err!(maze_config, self);
        MazeCameraRs::position_camera_to_size(&mut self.base, maze_config.bind().size);
    }
}

#[godot_api]
impl MazeCameraRs {
    fn position_camera_to_size(base: &mut Gd<Camera3D>, size: Vector2i) {
        base.set_size((max(size.x, size.y) * 2) as real);
        base.set_position(to_3d_from_2d(size) / 2 as real + Vector3::UP * 2 as real);
    }
}

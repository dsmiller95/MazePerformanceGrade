use crate::maze_scripts::maze_config::MazeConfigRs;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub(crate) struct FloorCreatorExample {
    #[export]
    floor_prefab: Option<Gd<PackedScene>>,
    #[export]
    maze_config: Option<Gd<MazeConfigRs>>,
    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl FloorCreatorExample {}

#[godot_api]
impl Node3DVirtual for FloorCreatorExample {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            floor_prefab: None,
            maze_config: None,
            base,
        }
    }

    fn ready(&mut self) {
        let floor_prefab = self
            .floor_prefab
            .as_ref()
            .expect("floor_prefab is required!");
        let maze_config = self.maze_config.as_ref().expect("maze_config is required!");
        todo!()
    }
}

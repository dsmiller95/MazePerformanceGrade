use crate::assert_some::assert_some_or_log_err;
use crate::maze_scripts::coroutines::maze_replay_ongoing::MazeReplayOngoing;
use crate::maze_scripts::floor_creator::FloorCreatorRs;
use crate::maze_scripts::helper_classes::historic_position::HistoricPosition;
use crate::maze_scripts::maze_config::MazeConfigRs;
use godot::engine::{Material, MeshInstance3D, Time};
use godot::prelude::*;
use std::task::Poll::{Pending, Ready};

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct MazeReplayRs {
    #[export]
    pub floors: Option<Gd<FloorCreatorRs>>,
    #[export]
    pub maze_config: Option<Gd<MazeConfigRs>>,

    #[export]
    pub highlight_material: Option<Gd<Material>>,
    #[export]
    pub traveled_material: Option<Gd<Material>>,

    #[base]
    base: Base<Node>,

    pending: Option<MazeReplayOngoing>,
}

#[godot_api]
impl NodeVirtual for MazeReplayRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            floors: None,
            maze_config: None,
            highlight_material: None,
            traveled_material: None,

            base,
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
                }
                Pending => {
                    self.pending = Some(replay);
                }
            }
        }
    }
}

#[godot_api]
impl MazeReplayRs {
    // #WEIRD rust : no async? might be impossible to do coroutines. or perhaps simply not ergonomic. async is hinted at for godot 3.x
    // #WEIRD rust : forces us to implement a custom coroutine-like framework
    pub fn begin_path_replay(
        &mut self,
        path: Vec<HistoricPosition>,
        replay_time_ms: f64,
    ) -> Option<()> {
        godot_print!("trying to play path");
        if let Some(_) = self.pending {
            godot_error!("currently playing path, cannot play another");
            return None;
        }

        self.pending = self.try_get_path_replay_ongoing(path, replay_time_ms);

        Some(())
    }

    pub fn try_get_path_replay_ongoing(
        &mut self,
        path: Vec<HistoricPosition>,
        replay_time_ms: f64,
    ) -> Option<MazeReplayOngoing> {
        if path.is_empty() {
            godot_error!("cannot replay an empty path");
            return None;
        }

        let total_time_ms = path.last()?.time_ms - path.first()?.time_ms;
        let scale = replay_time_ms / total_time_ms as f64;

        let time_ms = Time::singleton().get_ticks_msec();

        MazeReplayOngoing::try_new(self, path.into(), time_ms, scale)
    }

    pub fn highlight_tile(&mut self, tile: Vector2i, material: Gd<Material>) {
        assert_some_or_log_err!(maze_config, self);
        //assert_some_or_log_err!(floors, self);
        let Some(floors) = &mut self.floors else {
            if true {
                use godot::private::class_macros::godot_error;

                godot_error!("{} is required!", stringify!(floors));
            }
            return;
        };
        godot_print!("replaying over tile {}", tile);

        let size = maze_config.bind().size;
        let floors = floors.bind_mut();
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
                mesh.set_material_overlay(material);
            }
        }
    }
}

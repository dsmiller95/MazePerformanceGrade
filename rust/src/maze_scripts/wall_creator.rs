use godot::engine::RandomNumberGenerator;
use godot::prelude::*;
use crate::assert_some::assert_some_or_log_err;
use crate::maze_scripts::helper_classes::reachability::{NEIGHBORS, Reachability};
use crate::maze_scripts::maze_config::MazeConfigRs;
use crate::maze_scripts::helper_classes::rng_extensions::RngExtensions;


#[derive(GodotClass)]
#[class(base=Node3D)]
pub(crate) struct WallCreatorRs {
    #[export]
    wall_prefab: Option<Gd<PackedScene>>,
    #[export]
    maze_config: Option<Gd<MazeConfigRs>>,

    pub reachable: Option<Reachability>,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl NodeVirtual for WallCreatorRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            wall_prefab: None,
            maze_config: None,
            reachable: None,
            base
        }
    }

    fn ready(&mut self) {
        assert_some_or_log_err!(wall_prefab, self);
        assert_some_or_log_err!(maze_config, self);

        let mut rng = RandomNumberGenerator::new();

        let reachable = randomized_dfs_walls(maze_config.bind().size, &mut rng);
        for wall in reachable.get_walls(){
            wall.spawn_wall_at_edge(wall_prefab, &mut self.base);
        }

        self.reachable = Some(reachable);
    }
}


fn randomized_dfs_walls(size: Vector2i, rng: &mut Gd<RandomNumberGenerator>) -> Reachability {
    let mut reach = Reachability::new(size);
    let origin = rng.random_vector(size);

    internal_randomize_dfs_walls(&mut reach, origin, rng);
    reach.assert_fully_reachable();

    return reach;
}

fn internal_randomize_dfs_walls(reach: &mut Reachability, cell: Vector2i, rng: &mut Gd<RandomNumberGenerator>){
    let mut indexes = [0, 1, 2, 3];
    rng.shuffle_naive(&mut indexes);

    for index in indexes {
        let next_search = cell + NEIGHBORS[index];
        if !reach.in_bounds(next_search) || reach.reachable(next_search) {
            continue;
        }
        reach.reach_between(cell, next_search);
        internal_randomize_dfs_walls(reach, next_search, rng);
    }
}

#[godot_api]
impl WallCreatorRs {
}

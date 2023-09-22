use godot::prelude::*;
use crate::maze_scripts::maze_config::MazeConfigRs;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct FloorCreatorRs {
    #[export]
    floor_prefab: Option<Gd<PackedScene>>,
    #[export]
    // #WEIRD rust : all references that go through the engine are wrapped with smart pointer Gd
    maze_config: Option<Gd<MazeConfigRs>>,

    floors_indexed: Option<Vec<Gd<Node3D>>>,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl FloorCreatorRs {}

#[godot_api]
// #WEIRD rust : be sure to
impl Node3DVirtual for FloorCreatorRs {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            floor_prefab: None,
            maze_config: None,
            floors_indexed: None,
            base
        }
    }

    fn ready(&mut self) {

        let Some(floor_prefab) = &self.floor_prefab else {
            godot_error!("Floor_prefab is required!");
            return;
        };
        let Some(maze_config) = &self.maze_config else {
            godot_error!("maze_config is required!");
            return;
        };

        // #WEIRD rust : gotta call bind() to access properties directly. Although! this enforces rusts borrow check rules nicely.
        let size = maze_config.bind().size;
        let mut floors =  Vec::new();
        floors.reserve((size.x * size.y) as usize);
        for x in 0..size.x{
            for y in 0..size.y{
                let tile_position = Vector3::new(x as real, 0.0, y as real);
                let mut new_node = floor_prefab.instantiate_as::<Node3D>();
                new_node.set_position(tile_position);

                // TODO: get this to actually index right. maybe we want a boxed slice.
                //floors[(x + y * size.x) as usize] = new_node.clone();
                self.base.add_child(new_node.upcast::<Node>());
            }
        }

        self.floors_indexed = Some(floors)
    }
}
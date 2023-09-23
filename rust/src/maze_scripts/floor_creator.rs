use crate::assert_some::assert_some_or_log_err;
use crate::maze_scripts::helper_classes::box_iterator::BoxIterator;
use crate::maze_scripts::maze_config::MazeConfigRs;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub(crate) struct FloorCreatorRs {
    #[export]
    floor_prefab: Option<Gd<PackedScene>>,
    #[export]
    // #WEIRD rust : all references that go through the engine are wrapped with smart pointer Gd
    maze_config: Option<Gd<MazeConfigRs>>,

    #[export]
    pub floors_indexed: Array<Gd<Node3D>>,

    #[base]
    base: Base<Node3D>,
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
            floors_indexed: Array::new(),
            base,
        }
    }

    fn ready(&mut self) {
        assert_some_or_log_err!(floor_prefab, self);
        assert_some_or_log_err!(maze_config, self);

        // #WEIRD rust : gotta call bind() to access properties directly. Although! this enforces rusts borrow check rules nicely.
        let size = maze_config.bind().size;
        let mut floors: Vec<Gd<Node3D>> = vec![];
        let slice_size = (size.x * size.y) as usize;
        floors.reserve(slice_size);
        for tile in BoxIterator::from(size) {
            let tile_position = Vector3::new(tile.x as real, 0.0, tile.y as real);
            let mut new_node = floor_prefab.instantiate_as::<Node3D>();
            new_node.set_position(tile_position);

            // ensure the pushes index in this order, as if it were an indexed assignment
            //floors[(x + y * size.x) as usize] = new_node.clone();
            floors.push(new_node.clone());
            self.base.add_child(new_node.upcast::<Node>());
        }

        self.floors_indexed = Array::from(floors.as_slice());
    }
}

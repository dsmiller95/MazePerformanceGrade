use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
// #WEIRD rust : naming will clash if a GDScript class has same name
pub(crate) struct MazeConfigRs {
    #[export]
    pub size: Vector2i,
    #[export]
    pub exit: Vector2i,
    #[export]
    pub entry: Vector2i,
}

#[godot_api]
impl NodeVirtual for MazeConfigRs {
    // #WEIRD rust (ish): init is required to show up in godot add node menu
    fn init(_: Base<Self::Base>) -> Self {
        Self {
            size: Vector2i::new(10, 10),
            exit: Vector2i::new(9, 9),
            entry: Vector2i::new(0, 0),
        }
    }
}

#[godot_api]
impl MazeConfigRs {
    // #WEIRD rust : this impl block is always mandatory? interesting
}

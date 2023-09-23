use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub(crate) struct MazeReplayRs {
}

#[godot_api]
impl NodeVirtual for MazeReplayRs{
    fn init(_: Base<Self::Base>) -> Self {
        Self { }
    }
}

#[godot_api]
impl MazeReplayRs {
    // #WEIRD rust : no async? might be impossible to do coroutines. or perhaps simply not ergonomic. async is hinted at for godot 3.x
}

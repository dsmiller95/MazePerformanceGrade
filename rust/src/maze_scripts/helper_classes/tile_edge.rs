use godot::prelude::*;

#[derive(PartialEq, Eq)]
pub struct TileEdge{
    pub a: Vector2i,
    pub b: Vector2i,
}

impl TileEdge{
    pub fn new(a: Vector2i, b: Vector2i) -> Self {
        let delta = b - a;
        if delta.x == 0 && delta.y == 0 {
            panic!("self-pointing edge!")
        }

        if delta.x != 0 {
            if delta.x < 0 {
                return Self{a:b, b:a};
            }
        }else{
            if delta.y <0 {
                return Self{a:b, b:a};
            }
        }
        Self{a, b}
    }


    pub fn spawn_wall_at_edge(&self, wall_prefab: &Gd<PackedScene>, parent: &mut Gd<Node3D>){
        let world_a = to_3d_from_2d(self.a);
        let world_b = to_3d_from_2d(self.b);

        let center = (world_a + world_b) / 2.0;
        let forward = (world_b - world_a).normalized();
        let mut new_wall = wall_prefab.instantiate_as::<Node3D>();
        new_wall.set_transform(Transform3D::new(
            Basis::new_looking_at(forward, Vector3::UP, false),
            center
        ));
        parent.add_child(new_wall.upcast::<Node>())
    }
}

pub fn to_3d_from_2d(flat: Vector2i) -> Vector3{
    Vector3::new(flat.x as real, 0.0, flat.y as real)
}
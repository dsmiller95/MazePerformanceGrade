use godot::prelude::*;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct RustPlayer {
    speed: f64,
    angular_speed: f64,

    #[base]
    base: Base<Sprite2D>
}

use godot::engine::Sprite2DVirtual;

#[godot_api]
impl Sprite2DVirtual for RustPlayer {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("hello, player!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base.rotate((self.angular_speed * delta) as f32);
    }
}

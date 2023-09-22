use godot::prelude::*;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct RustPlayerrrr {
    #[export]
    speed: f64,
    #[export]
    angular_speed: f64,

    #[export]
    attractor: Option<Gd<Node2D>>,
    #[export]
    attraction_force: f64,

    #[base]
    base: Base<Sprite2D>
}

use godot::engine::Sprite2DVirtual;

#[godot_api]
impl Sprite2DVirtual for RustPlayerrrr {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("hello, player!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            attractor: None,
            attraction_force: 10.0,
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base.rotate((self.angular_speed * delta) as f32);

        let rotation = self.base.get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base.translate(velocity * delta as f32);

        match &self.attractor {
            None => {}
            Some(attractor) => {
                let pos_delta = attractor.get_position() - self.base.get_position();
                let attraction = (delta * self.attraction_force) as f32 * pos_delta;
                self.base.translate(attraction)
            }
        }
    }
}

#[godot_api]
impl RustPlayerrrr {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base.emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}

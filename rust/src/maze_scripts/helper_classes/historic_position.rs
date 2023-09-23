use godot::engine::Time;
use godot::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct HistoricPosition {
    pub tile: Vector2i,
    pub time_ms: u64,
}

impl HistoricPosition {
    pub fn new(tile: Vector2i) -> Self {
        let time_ms = Time::singleton().get_ticks_msec();
        Self { tile, time_ms }
    }
}

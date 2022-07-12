pub use bevy::prelude::*;
use rand::Rng;

pub fn rand_vec2() -> Vec2 {
    return Vec2::new(
        rand::thread_rng().gen_range(-1000.0..1000.0),
        rand::thread_rng().gen_range(-1000.0..1000.0),
    );
}

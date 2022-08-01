use std::time::Duration;

use glam::vec3;

use crate::*;

#[derive(Component, Default)]
pub struct Map {}

#[derive(Bundle, Default)]
pub struct MapBundle {
    map: Map,

    #[bundle]
    sprite: SpriteBundle,
}

pub const MAP_DIMS: Vec2 = vec2(1200.0, 800.0);

impl MapBundle {
    pub fn new(pos: Vec2, tex: Handle<Image>) -> Self {
        return Self {
            map: Map { ..default() },
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    custom_size: Some(MAP_DIMS),
                    ..default()
                },
                transform: Transform {
                    translation: vec3(0.0, 0.0, 10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        };
    }
}

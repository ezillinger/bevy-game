use bevy_rapier2d::na::clamp;

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

pub fn clamp_position(pos: &Vec2) -> Vec2 {
    return vec2(
        clamp(pos.x, -MAP_DIMS.x / 2.2, MAP_DIMS.x / 2.2),
        clamp(pos.y, -MAP_DIMS.y / 2.5, MAP_DIMS.y / 2.3),
    );
}

impl MapBundle {
    pub fn new(pos: Vec2, tex: Handle<Image>) -> Self {
        return Self {
            map: Map { ..default() },
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    custom_size: Some(MAP_DIMS),
                    color: Color::hsla(0.0, 0.0, 0.5, 0.5),
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        };
    }
}

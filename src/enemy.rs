use std::time::Duration;

use benimator::Play;
use bevy::{core::Stopwatch, math::vec3};

use crate::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub position: Vec2,
    pub direction: Vec2,

    pub radius: f32,
    pub damage: f32,

    pub health: i32,
    pub max_health: i32,

    pub hit_interval: Duration,
    pub point_value: i32,
    pub speed: f32,

    pub hit_timer: Stopwatch,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    collider: Collider,
    sensor: Sensor,
    animation: Handle<SpriteSheetAnimation>,
    play: Play,

    #[bundle]
    sprite: SpriteSheetBundle,
}

impl EnemyBundle {
    pub fn new(
        pos: Vec2,
        tex: Handle<TextureAtlas>,
        animation: Handle<SpriteSheetAnimation>,
    ) -> Self {
        return Self {
            enemy: Enemy {
                position: pos,
                direction: vec2(1.0, 0.0),
                radius: 1.0,
                damage: 10.0,
                point_value: 100,
                health: 100,
                hit_interval: Duration::from_millis(300),
                speed: 10.0,
                ..default()
            },
            collider: Collider::capsule_y(50.0, 50.0),
            sprite: SpriteSheetBundle {
                texture_atlas: tex,
                sprite: TextureAtlasSprite {
                    custom_size: Some(vec2(200.0, 200.0)),
                    ..default()
                },
                ..default()
            },
            animation: animation,
            ..default()
        };
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(
        Entity,
        &mut Enemy,
        &mut Transform,
        &Collider,
        &mut TextureAtlasSprite,
    )>,
    mut game: ResMut<Game>,
    rapier_ctx: Res<RapierContext>,
) {
    for (entity, mut enemy, mut transform, collider, mut sprite) in enemies.iter_mut() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }

        enemy.hit_timer.tick(time.delta());

        let player_dir = (game.player.position - enemy.position).normalize();
        let new_pos = enemy.position + rand_vec2() / 100.0 + player_dir * enemy.speed;

        enemy.direction = player_dir;
        enemy.position = new_pos;

        sprite.flip_x = enemy.direction.x < 0.0;

        *transform = Transform {
            translation: Vec3::new(
                enemy.position.x,
                enemy.position.y,
                z_from_y(enemy.position.y),
            ),
            ..default()
        };

        if enemy.hit_timer.elapsed() > enemy.hit_interval {
            rapier_ctx.intersections_with_shape(
                transform.translation.truncate(),
                0.0,
                collider,
                QueryFilter::default(),
                |entity| {
                    if entity.id() == game.player.id.unwrap().id() {
                        game.player.health -= enemy.damage as i32;
                        enemy.hit_timer.reset();
                        return false;
                    }
                    return true;
                },
            );
        }
    }
}

use std::time::Duration;

use bevy::core::Stopwatch;

use crate::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub position: Vec2,
    pub radius: f32,
    pub damage: f32,

    pub health: i32,
    pub max_health: i32,

    pub hit_interval: Duration,
    pub point_value: i32,

    pub hit_timer: Stopwatch,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    collider: Collider,
    sensor: Sensor,

    #[bundle]
    sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(pos: Vec2, tex: Handle<Image>) -> Self {
        return Self {
            enemy: Enemy {
                position: pos,
                radius: 1.0,
                damage: 10.0,
                point_value: 100,
                health: 100,
                hit_interval: Duration::from_millis(300),
                ..default()
            },
            collider: Collider::capsule_y(50.0, 50.0),
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(200.0, 220.0)),
                    ..default()
                },
                ..default()
            },
            ..default()
        };
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Enemy, &mut Transform, &Collider)>,
    mut game: ResMut<Game>,
    rapier_ctx: Res<RapierContext>,
) {
    for (entity, mut enemy, mut transform, collider) in enemies.iter_mut() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }

        enemy.hit_timer.tick(time.delta());

        enemy.position += rand_vec2() / 100.0f32;
        *transform = Transform {
            translation: Vec3::new(enemy.position.x, enemy.position.y, 32.0f32),
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

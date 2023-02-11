use std::time::Duration;

use bevy::time::Stopwatch;

use crate::*;

const ENEMY_DIMS: Vec2 = vec2(70.0, 90.0);

#[derive(Component, Default)]
pub struct Enemy {
    pub position: Vec2,
    pub direction: Vec2,

    pub radius: f32,
    pub damage: f32,

    pub health: f32,
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

    #[bundle]
    sprite: SpriteSheetBundle,
}

impl EnemyBundle {
    pub fn new(
        pos: Vec2,
        tex: Handle<TextureAtlas>,
    ) -> Self {
        return Self {
            enemy: Enemy {
                position: pos,
                direction: vec2(1.0, 0.0),
                radius: 1.0,
                damage: 10.0,
                point_value: 100,
                health: 100.0,
                max_health: 100,
                hit_interval: Duration::from_millis(300),
                speed: 5.0,
                ..default()
            },
            collider: Collider::capsule_y(ENEMY_DIMS.y / 5.0, ENEMY_DIMS.x / 4.0),
            sprite: SpriteSheetBundle {
                texture_atlas: tex,
                sprite: TextureAtlasSprite {
                    custom_size: Some(ENEMY_DIMS),
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

    let mut enemy_push_away = Vec::new();
    for (entity, mut enemy, mut transform, collider, mut sprite) in enemies.iter() {
        let mut push = Vec2::ZERO;
        rapier_ctx.intersections_with_shape(
                transform.translation.truncate(),
                0.0,
                collider,
                QueryFilter::default(),
                |intersecting_entity| {
                    if intersecting_entity != entity && intersecting_entity != game.player.id.unwrap() {
                        if let Ok(other) = enemies.get(intersecting_entity) {
                            let dir = (enemy.position - other.1.position).normalize();
                            push += 10.0 * dir;
                        }
                    }
                    return true;
                },
            );
            enemy_push_away.push(push);
    }

    let mut i = 0;
    for (entity, mut enemy, mut transform, collider, mut sprite) in enemies.iter_mut() {

        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }

        enemy.hit_timer.tick(time.delta());

        let player_dir = (game.player.position - enemy.position).normalize();
        let player_dist = (game.player.position - enemy.position).length();
        enemy.direction = (25.5 * enemy.direction
            + (player_dist / 700.0) * 10.5 * rand_norm_vec2()
            + (1.0 - player_dist / 700.0) * 5.25 * player_dir
            + *enemy_push_away.get(i).expect("oob"))
            .normalize();
        let new_pos = enemy.position + enemy.direction * enemy.speed;

        enemy.position = clamp_position(&new_pos);

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
                    if entity == game.player.id.unwrap() {
                        game.player.health -= enemy.damage;
                        enemy.hit_timer.reset();
                        return false;
                    }
                    return true;
                },
            );
        }
        i += 1;
    }
}

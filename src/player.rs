use std::time::Duration;

use bevy::core::Stopwatch;

use crate::*;

#[derive(Default, Component)]
pub struct Player {
    pub position: Vec2,
    pub direction: Vec2,
    pub radius: f32,
    pub id: Option<Entity>,
    pub bullets: Vec<Entity>,
    pub shot_clock: Stopwatch,
}

impl Player {
    fn tick(self: &mut Self, delta: Duration) {
        self.shot_clock.tick(delta);
    }
}

pub fn tick(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut transform: Query<&mut Transform>,
) {
    const PLAYER_SPEED: f32 = 200.0;
    const BULLET_SPEED: f32 = 800.0;
    const FIRE_INTERVAL: f32 = 0.25;

    game.player.tick(time.delta());

    if input.pressed(KeyCode::Left) {
        game.player.direction = Vec2::new(-1.0, 0.0);
        game.player.position.x -= PLAYER_SPEED * time.delta().as_secs_f32();
    } else if input.pressed(KeyCode::Right) {
        game.player.direction = Vec2::new(1.0, 0.0);
        game.player.position.x += PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if input.pressed(KeyCode::Up) {
        game.player.direction = Vec2::new(0.0, 1.0);
        game.player.position.y += PLAYER_SPEED * time.delta().as_secs_f32();
    } else if input.pressed(KeyCode::Down) {
        game.player.direction = Vec2::new(0.0, -1.0);
        game.player.position.y -= PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if input.pressed(KeyCode::Space) && game.player.shot_clock.elapsed_secs() >= FIRE_INTERVAL {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform {
                    translation: game.player.position.extend(32.0f32),
                    ..default()
                },
                ..default()
            })
            .insert(Bullet {
                shooter: game.player.id.unwrap(),
                position: game.player.position,
                hits_player: false,
                velocity: BULLET_SPEED * game.player.direction,
            })
            .insert(Collider::cuboid(25.0, 25.0))
            .insert(Sensor);

        game.player.shot_clock.reset();
    }

    *transform.get_mut(game.player.id.unwrap()).unwrap() = Transform {
        translation: Vec3::new(game.player.position.x, game.player.position.y, 32.0f32),
        ..default()
    };
}

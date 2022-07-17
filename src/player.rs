use std::time::Duration;

use bevy::core::Stopwatch;

use crate::*;

#[derive(Component)]
pub struct Player {
    pub position: Vec2,
    pub direction: Vec2,
    pub radius: f32,
    pub id: Option<Entity>,
    pub bullets: Vec<Entity>,
    pub shot_clock: Stopwatch,
    pub health: i32,
    pub max_health: i32,
    pub score: i32,
}

impl Default for Player {
    fn default() -> Self {
        return Player {
            health: 100,
            max_health: 100,
            position: Vec2::ZERO,
            radius: 1.0,
            bullets: Vec::<Entity>::new(),
            shot_clock: Stopwatch::new(),
            direction: Vec2::new(1.0, 0.0),
            score: 0,
            id: None,
        };
    }
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    collider: Collider,
    sensor: Sensor,

    #[bundle]
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(tex: Handle<Image>) -> PlayerBundle {
        return PlayerBundle {
            player: Player::default(),
            collider: Collider::cuboid(100.0, 100.0),
            sensor: Sensor,
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    //color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(150.0, 160.0)),
                    ..default()
                },
                ..default()
            },
        };
    }
}

impl Player {
    fn tick_cooldowns(self: &mut Self, delta: Duration) {
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

    if input.pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Menu));
        return;
    }

    if game.player.health <= 0 {
        commands.insert_resource(NextState(GameState::GameOver));
        return;
    }

    game.player.tick_cooldowns(time.delta());

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

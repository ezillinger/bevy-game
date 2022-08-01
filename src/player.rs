use std::time::Duration;

use crate::*;
use bevy::time::Stopwatch;

const PLAYER_DIMS: Vec2 = vec2(40.0, 45.0);

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
            collider: Collider::capsule_y(PLAYER_DIMS.y / 4.0, PLAYER_DIMS.x / 4.0),
            sensor: Sensor,
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    custom_size: Some(PLAYER_DIMS),
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
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut player: Query<(&mut Sprite, &mut Transform), With<Player>>,
) {
    const PLAYER_SPEED: f32 = 400.0;
    const BULLET_SPEED: f32 = 400.0;
    const BULLET_SIZE: f32 = 10.0;
    const FIRE_INTERVAL: f32 = 0.25;

    if input.pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Paused));
        return;
    }

    if game.player.health <= 0 {
        commands.insert_resource(NextState(GameState::GameOver));
        return;
    }

    game.player.tick_cooldowns(time.delta());

    if let Some(window) = windows.get_primary() {
        if let Some(mouse_pos) = window.cursor_position() {
            game.mouse_rel_pos = mouse_pos / Vec2::new(window.width(), window.height());

            let aspect_ratio = window.width() / window.height();
            let world_pos = (game.mouse_rel_pos - 0.5) * Vec2::new(5000.0, aspect_ratio / 5000.0);
            game.player.direction = (world_pos - game.player.position).normalize();
            game.mouse_world_pos = world_pos;
        }
    }

    if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
        game.player.position.x -= PLAYER_SPEED * time.delta().as_secs_f32();
    } else if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
        game.player.position.x += PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
        game.player.position.y += PLAYER_SPEED * time.delta().as_secs_f32();
    } else if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
        game.player.position.y -= PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if (input.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left))
        && game.player.shot_clock.elapsed_secs() >= FIRE_INTERVAL
    {
        commands.spawn_bundle(BulletBundle::new(Bullet {
            shooter: Some(game.player.id.unwrap()),
            position: game.player.position,
            hits_player: false,
            velocity: BULLET_SPEED * game.player.direction,
            damage: 150,
            radius: BULLET_SIZE,
        }));

        game.player.shot_clock.reset();
    }

    if let Ok((mut sprite, mut transform)) = player.get_single_mut() {
        *transform = Transform {
            translation: Vec3::new(
                game.player.position.x,
                game.player.position.y,
                z_from_y(game.player.position.y),
            ),
            ..default()
        };
        sprite.flip_x = game.player.direction.x < 0.0;
    }
}

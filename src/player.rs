use std::{time::Duration, f32::consts::PI};

use crate::*;
use bevy::{time::Stopwatch, sprite::Mesh2dHandle};
use bevy_rapier2d::na::Rotation2;
use map::clamp_position;
use physics_sprite::PhysicsSpriteBundle;

const PLAYER_DIMS: Vec2 = vec2(40.0, 45.0);

pub struct Stat {
    pub base: f32,

    pub multiply: f32,
    pub add: f32,
}

impl Stat {
    fn new(base: f32) -> Stat {
        Stat {
            base: base,
            add: 0.0,
            multiply: 1.0,
        }
    }

    pub fn value(&self) -> f32 {
        self.base * self.multiply + self.add
    }
}

#[derive(Component)]
pub struct Stats {
    pub max_health: Stat,
    pub shot_duration: Stat,
    pub shot_speed: Stat,
    pub shot_size: Stat,
    pub fire_interval: Stat,
    pub damage: Stat,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            damage: Stat::new(60.0),
            max_health: Stat::new(100.0),
            fire_interval: Stat::new(0.5),
            shot_speed: Stat::new(500.0),
            shot_duration: Stat::new(1.0),
            shot_size: Stat::new(10.0),
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub position: Vec2,
    pub direction: Vec2,
    pub id: Option<Entity>,
    pub shot_clock: Stopwatch,
    pub score: i32,
    pub health: f32,

    pub stats: Stats,
}

impl Default for Player {
    fn default() -> Self {
        return Player {
            position: Vec2::ZERO,
            shot_clock: Stopwatch::new(),
            direction: Vec2::new(1.0, 0.0),
            health: 100.0,
            score: 0,
            id: None,
            stats: Stats::new(),
        };
    }
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    player: Player,

    #[bundle]
    sprite: PhysicsSpriteBundle,
}

#[derive(Default, Component)]
pub struct Cursor {
    screen_pos: Vec2,
    world_pos: Vec2,
}

#[derive(Default, Bundle)]
pub struct CursorBundle {
    pub cursor: Cursor,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(material: Handle<ColorMaterial>, mesh: Mesh2dHandle) -> PlayerBundle {
        return PlayerBundle {
            player: Player::default(),
            sprite: PhysicsSpriteBundle::new(&PLAYER_DIMS, &Vec2::ZERO, material, mesh),
        };
    }
}

impl Player {
    fn tick_cooldowns(self: &mut Self, delta: Duration) {
        self.shot_clock.tick(delta);
    }
}

pub fn tick_cursor(
    mut game: ResMut<Game>,
    windows: Res<Windows>,
    camera: Query<&OrthographicProjection, (With<Camera>, Without<Cursor>)>,
    mut cursor: Query<(&mut Transform, &mut Cursor)>,
) {
    if let Ok(projection) = camera.get_single() {
        if let Some(window) = windows.get_primary() {
            game.window_size = vec2(window.width(), window.height());
            if let Some(mouse_pos) = window.cursor_position() {
                if let Ok((mut transform, mut cursor)) = cursor.get_single_mut() {
                    cursor.screen_pos = mouse_pos;
                    let world_pos = vec2(
                        lerp(projection.left, projection.right, game.mouse_rel_pos.x),
                        lerp(projection.bottom, projection.top, game.mouse_rel_pos.y),
                    );
                    game.mouse_rel_pos = cursor.screen_pos / game.window_size;
                    cursor.world_pos = world_pos;
                    transform.translation = world_pos.extend(33.0);
                }
            }
        }
    }
}

pub fn make_mesh() -> Mesh {
    return shape::RegularPolygon::new(30.0, 3).into();
}

pub fn tick(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut player: Query<(&mut Mesh2dHandle, &mut Transform), With<Player>>,
    cursor: Query<&Cursor>,
) {
    const PLAYER_SPEED: f32 = 400.0;

    if input.pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Paused));
        return;
    }

    if game.player.health <= 0.0 {
        commands.insert_resource(NextState(GameState::GameOver));
        return;
    }

    game.player.tick_cooldowns(time.delta());

    if let Ok(cursor) = cursor.get_single() {
        game.player.direction = (cursor.world_pos - game.player.position).normalize();
        game.mouse_world_pos = cursor.world_pos;
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

    game.player.position = clamp_position(&game.player.position);

    if (input.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left))
        && game.player.shot_clock.elapsed_secs() >= game.player.stats.fire_interval.value()
    {
        let velocity = if input.pressed(KeyCode::LShift) { game.player.stats.shot_speed.value() * 10.0 } else { game.player.stats.shot_speed.value() };
        commands.spawn(BulletBundle::new(Bullet {
            shooter: Some(game.player.id.unwrap()),
            position: game.player.position,
            hits_player: false,
            velocity: velocity * game.player.direction,
            damage: game.player.stats.damage.value(),
            radius: game.player.stats.shot_size.value(),
        }, game.handles.bullet_mesh.clone()));

        game.player.shot_clock.reset();
    }

    let angle = game.player.direction.y.atan2(game.player.direction.x);
    if let Ok((mut sprite, mut transform)) = player.get_single_mut() {
        *transform = Transform {
            translation: Vec3::new(
                game.player.position.x,
                game.player.position.y,
                z_from_y(game.player.position.y),
            ),
            rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle - PI/2.0),
            ..default()
        };
        // sprite.flip_x = game.player.direction.x < 0.0;
    }
}

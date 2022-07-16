mod bullet;
mod enemy;
mod player;
mod prelude;

use player::*;
use prelude::*;

use bullet::Bullet;
use enemy::{Enemy, EnemyBundle};

#[derive(Default)]
pub struct Game {
    player: Player,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(player::tick)
        .add_system(enemy::tick)
        .add_system(bullet::tick)
        .run();
}

fn setup(mut commands: Commands, mut game: ResMut<Game>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    game.player.id = Some(
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("../../../assets/player.png"),
                sprite: Sprite {
                    //color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(150.0, 160.0)),
                    ..default()
                },
                ..default()
            })
            .insert(Collider::cuboid(75.0, 80.0))
            .insert(Sensor)
            .insert(Player::default())
            .id(),
    );

    let tex: Handle<Image> = asset_server.load("../../../assets/enemy.png");
    for _ in 0..10 {
        commands.spawn_bundle(EnemyBundle::new(rand_vec2(), tex.clone()));
    }
}

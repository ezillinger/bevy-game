mod bullet;
mod enemy;
mod player;
mod prelude;

use bevy::asset;
use player::*;
use prelude::*;

use rand::Rng;

use bullet::{tick_bullets, Bullet};
use enemy::{tick_enemies, Enemy};

#[derive(Default)]
pub struct Game {
    player: Player,
}

fn main() {

    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player::handle_input)
        .add_system(tick_enemies)
        .add_system(tick_bullets)
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
            .insert(Player::default())
            .id(),
    );

    let tex: Handle<Image> = asset_server.load("../../../assets/enemy.png");
    for _ in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: tex.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(200.0, 220.0)),
                    ..default()
                },
                ..default()
            })
            .insert(Enemy{ position: rand_vec2(), radius: 1.0f32 });
    }
}

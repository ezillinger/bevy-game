mod bullet;
mod enemy;
mod player;
mod prelude;
mod ui;

use bevy::{
    render::{
        camera::{self, CameraProjection, ScalingMode},
        primitives::Frustum,
    },
    transform,
};
use player::*;
use prelude::*;

use bevy_egui::{EguiPlugin, EguiSettings};

use bullet::Bullet;
use enemy::{Enemy, EnemyBundle};
use iyes_loopless::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Gameplay,
    Paused,
    GameOver,
    Reset,
}

#[derive(Default)]
pub struct Game {
    player: Player,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_loopless_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(ui::draw_hud)
        .add_system(ui::draw_main_menu.run_in_state(GameState::Menu))
        .add_system(ui::draw_game_over.run_in_state(GameState::GameOver))
        .add_system(ui::draw_pause_menu.run_in_state(GameState::Paused))
        .add_system(reset.run_in_state(GameState::Reset))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .with_system(player::tick)
                .with_system(enemy::tick)
                .with_system(bullet::tick)
                .into(),
        )
        .run();
}

fn reset(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, With<Enemy>)>,
) {
    game.player = Player {
        id: game.player.id,
        ..default()
    };

    //enemies
    for e in enemies.iter() {
        commands.entity(e.0).despawn();
    }

    let tex: Handle<Image> = asset_server.load("../../../assets/enemy.png");
    for _ in 0..10 {
        commands.spawn_bundle(EnemyBundle::new(rand_vec2(), tex.clone()));
    }

    commands.insert_resource(NextState(GameState::Gameplay));
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    // gui
    egui_settings.as_mut().scale_factor = 2.0;

    // camera
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
    commands.spawn_bundle(camera_bundle);

    //player
    let tex = asset_server.load("../../../assets/player.png");
    game.player.id = Some(commands.spawn_bundle(PlayerBundle::new(tex.clone())).id());
}

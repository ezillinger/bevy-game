mod bullet;
mod enemy;
mod map;
mod player;
mod prelude;
mod ui;

use std::time::Duration;

use bevy::{
    asset::{self, LoadState},
    core::Zeroable,
    math::vec2,
    render::{
        camera::{CameraProjection, ScalingMode},
        texture::ImageSettings,
    },
};

use benimator::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use iyes_loopless::prelude::*;

use bullet::*;
use enemy::*;
use map::*;
use player::*;
use prelude::*;

#[derive(Default)]
struct Handles {
    player_tex: Handle<Image>,

    map_tex: Handle<Image>,

    enemy_tex: Handle<Image>,
    enemy_atlas: Handle<TextureAtlas>,
    enemy_animation: Handle<SpriteSheetAnimation>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Init,
    Menu,
    Gameplay,
    Paused,
    GameOver,
    Reset,
}

#[derive(Default)]
pub struct Game {
    player: Player,
    map: Map,
    handles: Handles,
    mouse_world_pos: Vec2,
    mouse_rel_pos: Vec2,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ImageSettings::default_nearest())
        .add_loopless_state(GameState::Init)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_startup_system(setup)
        .add_system(wait_for_assets.run_in_state(GameState::Init))
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

fn wait_for_assets(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    asset_server: ResMut<AssetServer>,
) {
    println!("Waiting for assets");
    if LoadState::Loaded == asset_server.get_load_state(&game.handles.enemy_tex)
        && LoadState::Loaded == asset_server.get_load_state(&game.handles.player_tex)
        && LoadState::Loaded == asset_server.get_load_state(&game.handles.map_tex)
    {
        println!("Textures loaded. Building texture atlases");
        {
            // enemy
            let image_size = images.get(&game.handles.enemy_tex).unwrap().size();
            let atlas = TextureAtlas::from_grid(
                game.handles.enemy_tex.clone(),
                vec2(image_size.x / 4.0, image_size.y),
                4,
                1,
            );
            game.handles.enemy_atlas = atlases.add(atlas);
        }

        println!("Atlas Building Complete");
        commands.insert_resource(NextState(GameState::Menu));
    }
}

fn reset(mut commands: Commands, mut game: ResMut<Game>, enemies: Query<(Entity, With<Enemy>)>) {
    game.player = Player {
        id: game.player.id,
        ..default()
    };

    //enemies
    for e in enemies.iter() {
        commands.entity(e.0).despawn();
    }

    for _ in 0..10 {
        commands.spawn_bundle(EnemyBundle::new(
            rand_vec2(),
            game.handles.enemy_atlas.clone(),
            game.handles.enemy_animation.clone(),
        ));
    }

    commands.insert_resource(NextState(GameState::Gameplay));
}

fn make_camera() -> Camera2dBundle {
    // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
    // the camera's translation by far and use a right handed coordinate system
    let far = 1000.0;
    let orthographic_projection = OrthographicProjection {
        far,
        depth_calculation: bevy::render::camera::DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedVertical(MAP_DIMS.y),
        scale: 1.0,
        ..Default::default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
    let view_projection =
        orthographic_projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = bevy::render::primitives::Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        orthographic_projection.far,
    );
    Camera2dBundle {
        camera: Camera { ..default() },
        frustum: frustum,
        projection: orthographic_projection,
        transform: transform,
        ..default()
    }
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut egui_settings: ResMut<EguiSettings>,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    // gui
    egui_settings.as_mut().scale_factor = 2.0;

    // camera
    commands.spawn_bundle(make_camera());

    // load assets
    game.handles.map_tex = asset_server.load("../../../assets/map.png");
    game.handles.enemy_tex = asset_server.load("../../../assets/creature-sheet.png");
    game.handles.player_tex = asset_server.load("../../../assets/player.png");

    game.handles.enemy_animation = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(250),
    ));

    // map
    commands.spawn_bundle(MapBundle::new(Vec2::ZERO, game.handles.map_tex.clone()));

    //player
    game.player.id = Some(
        commands
            .spawn_bundle(PlayerBundle::new(game.handles.player_tex.clone()))
            .id(),
    );
}

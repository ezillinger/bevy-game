mod bullet;
mod enemy;
mod game;
mod map;
mod physics_sprite;
mod pickup;
mod player;
mod prelude;
mod ui;

use std::time::Duration;

use bevy::{
    asset::LoadState,
    math::vec2,
    render::{
        camera::{CameraProjection, CameraRenderGraph, DepthCalculation, ScalingMode},
        primitives::Frustum,
        texture::ImageSettings,
        view::VisibleEntities,
    },
};

use benimator::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use iyes_loopless::prelude::*;

use bullet::*;
use enemy::*;
use map::*;
use pickup::*;
use player::*;
use prelude::*;

#[derive(Default)]
struct Handles {
    player_tex: Handle<Image>,
    map_tex: Handle<Image>,
    pickup_tex: Handle<Image>,

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
    handles: Handles,
    mouse_world_pos: Vec2,
    mouse_rel_pos: Vec2,
    window_size: Vec2,
    wave: i32,
    kills: i32,
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
                .with_system(player::tick_cursor)
                .with_system(player::tick)
                .with_system(pickup::tick)
                .with_system(enemy::tick)
                .with_system(bullet::tick)
                .with_system(game::spawn_waves)
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
        && LoadState::Loaded == asset_server.get_load_state(&game.handles.pickup_tex)
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

    game.wave = 0;
    game.kills = 0;
    game.player.score = 0;

    //enemies
    for e in enemies.iter() {
        commands.entity(e.0).despawn();
    }

    commands.insert_resource(NextState(GameState::Gameplay));
}

fn make_camera() -> Camera2dBundle {
    let far = 1000.0;
    // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
    // the camera's translation by far and use a right handed coordinate system
    let mut projection = OrthographicProjection {
        far,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedVertical(MAP_DIMS.y),
        ..Default::default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far(),
    );
    Camera2dBundle {
        camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::NAME),
        projection,
        visible_entities: VisibleEntities::default(),
        frustum,
        transform,
        global_transform: Default::default(),
        camera: Camera::default(),
        camera_2d: Camera2d::default(),
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
    game.handles.pickup_tex = asset_server.load("../../../assets/pickup.png");
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

    commands.spawn_bundle(CursorBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

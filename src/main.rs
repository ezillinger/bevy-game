mod bullet;
mod enemy;
mod game;
mod map;
mod physics_sprite;
mod pickup;
mod player;
mod prelude;
mod ui;

use bevy::{
    asset::LoadState,
    core_pipeline::{
        bloom::BloomSettings, clear_color::ClearColorConfig,
    },
    math::vec2,
    render::{
        camera::{CameraProjection, ScalingMode},
        primitives::Frustum,
    },
    sprite::Mesh2dHandle,
};

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
    player_mesh: Mesh2dHandle,

    pickup_tex: Handle<Image>,
    pickup_mesh: Mesh2dHandle,
    bullet_mesh: Mesh2dHandle,

    enemy_tex: Handle<Image>,
    enemy_atlas: Handle<TextureAtlas>,
    enemy_mesh: Mesh2dHandle,

    map_tex: Handle<Image>,
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

#[derive(Default, Resource)]
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
        .insert_resource(Msaa{samples: 1})
        .init_resource::<Game>()
        .add_loopless_state(GameState::Init)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor { title: "Hello".into(), ..default() },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(EguiPlugin)
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
                None,
                None,
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
    let projection = OrthographicProjection {
        far,
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
        projection,
        frustum,
        transform,
        camera_2d: Camera2d{ clear_color: ClearColorConfig::Custom(Color::BLACK)},
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    }
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut egui_settings: ResMut<EguiSettings>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // gui
    egui_settings.as_mut().scale_factor = 2.0;

    // camera
    commands.spawn((
        make_camera(),
        BloomSettings {
            intensity: 0.3,
            threshold: 0.6,
            knee: 0.1,
            ..default()
        },
    ));

    // load assets
    game.handles.map_tex = asset_server.load("map.png");
    game.handles.pickup_tex = asset_server.load("pickup.png");
    game.handles.enemy_tex = asset_server.load("creature-sheet.png");
    game.handles.player_tex = asset_server.load("player.png");

    game.handles.player_mesh = meshes.add(make_mesh()).into();
    game.handles.enemy_mesh = meshes.add(shape::Circle::new(10.0).into()).into();
    game.handles.pickup_mesh = meshes.add(shape::Box::new(10.0, 10.0, 10.0).into()).into();
    game.handles.bullet_mesh = meshes.add(shape::Circle::new(10.0).into()).into();

    // map
    commands.spawn(MapBundle::new(Vec2::ZERO, game.handles.map_tex.clone()));

    //player
    game.player.id = Some(
        commands
            .spawn(PlayerBundle::new(
                materials.add(ColorMaterial {
                    color: Color::hsla(130.0, 1.0, 0.5, 1.0),
                    texture: None,
                }),
                game.handles.player_mesh.clone(),
            ))
            .id(),
    );

    commands.spawn(CursorBundle {
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

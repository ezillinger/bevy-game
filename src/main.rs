mod bullet;
mod enemy;
mod player;
mod prelude;

use player::*;
use prelude::*;

use bullet::Bullet;
use enemy::{Enemy, EnemyBundle};
use iyes_loopless::prelude::*;

use bevy_egui::{
    egui::{self},
    EguiContext, EguiPlugin, EguiSettings,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Gameplay,
    Paused,
    GameOver,
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
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(draw_hud)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Menu)
                .with_system(draw_main_menu)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::GameOver)
                .with_system(draw_game_over)
                .into(),
        )
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

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    // gui
    egui_settings.as_mut().scale_factor = 2.0;

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //player

    let tex = asset_server.load("../../../assets/player.png");
    game.player.id = Some(commands.spawn_bundle(PlayerBundle::new(tex.clone())).id());

    //enemies
    let tex: Handle<Image> = asset_server.load("../../../assets/enemy.png");
    for _ in 0..10 {
        commands.spawn_bundle(EnemyBundle::new(rand_vec2(), tex.clone()));
    }
}

fn draw_main_menu(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<CurrentState<GameState>>,
) {
    egui::Area::new("Main Menu")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            let visuals = &mut ui.style_mut().visuals;
            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
            ui.label("Main Menu");
            if ui.button("Start").clicked() {
                commands.insert_resource(NextState(GameState::Gameplay));
            }
        });
}

fn draw_game_over(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<Game>,
) {
    egui::Area::new("Game Over")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().spacing.item_spacing.y = 30.0;
            let visuals = &mut ui.style_mut().visuals;
            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
            ui.label("You Suck");
            ui.label(format!("Your Score: {}", state.player.score));
            if ui.button("Restart").clicked() {
                commands.insert_resource(NextState(GameState::Gameplay));
            }
        });
}

fn draw_hud(mut egui_context: ResMut<EguiContext>, game: Res<Game>) {
    egui::Area::new("hud")
        .anchor(egui::Align2::LEFT_TOP, [10.0, 10.0])
        .show(egui_context.ctx_mut(), |ui| {
            let visuals = &mut ui.style_mut().visuals;
            visuals.extreme_bg_color = egui::Color32::DARK_GRAY;
            visuals.faint_bg_color = egui::Color32::RED;
            visuals.widgets.noninteractive.bg_fill = egui::Color32::RED;
            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
            visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::RED;
            visuals.selection.bg_fill = egui::Color32::RED;
            ui.label(format!("Score: {:?}", game.player.score));
            ui.add(
                egui::ProgressBar::new(game.player.health as f32 / game.player.max_health as f32)
                    .text(format!("{}/{}", game.player.health, game.player.max_health))
                    .desired_width(100.0)
                    .animate(false),
            )
        });
}

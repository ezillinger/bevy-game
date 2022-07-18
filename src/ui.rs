use super::*;

use crate::prelude::*;

use bevy::app::AppExit;
use bevy_egui::{
    egui::{self},
    EguiContext,
};

pub fn draw_main_menu(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut exit: EventWriter<AppExit>,
) {
    egui::Area::new("Main Menu")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            let visuals = &mut ui.style_mut().visuals;
            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
            ui.label("Main Menu");
            if ui.button("New Game").clicked() {
                commands.insert_resource(NextState(GameState::Reset));
            }
            if ui.button("Quit to Desktop").clicked() {
                exit.send(AppExit);
            }
        });
}

pub fn draw_pause_menu(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut exit: EventWriter<AppExit>,
) {
    egui::Area::new("Pause Menu")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            let visuals = &mut ui.style_mut().visuals;
            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
            ui.label("Paused");
            if ui.button("Resume").clicked() {
                commands.insert_resource(NextState(GameState::Gameplay));
            }
            if ui.button("Main Menu").clicked() {
                commands.insert_resource(NextState(GameState::Menu));
            }
            if ui.button("Restart").clicked() {
                commands.insert_resource(NextState(GameState::Reset));
            }
            if ui.button("Quit to Desktop").clicked() {
                exit.send(AppExit);
            }
        });
}

pub fn draw_game_over(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    state: Res<Game>,
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
                commands.insert_resource(NextState(GameState::Reset));
            }
        });
}

pub fn draw_hud(mut egui_context: ResMut<EguiContext>, game: Res<Game>, windows: Res<Windows>) {
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
            );

            let debug = true;
            if debug {
                if let Some(pos) = windows.get_primary().unwrap().cursor_position() {
                    ui.label(format!("Mouse: {:?}", pos));
                    ui.label(format!("Mouse World: {:?}", game.mouse_world_pos));
                }
                ui.label(format!("Player: {:?}", game.player.position));
            }
        });
}

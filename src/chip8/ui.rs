use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext, egui,
};

use super::Chip8;
use super::Config;
use super::ResetFlag;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(Startup, setup_system)
            .add_systems(EguiPrimaryContextPass, ui_example_system);
    }
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    chip8: Res<Chip8>,
    mut reset_flag: ResMut<ResetFlag>,
    config: Res<Config>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let _debug_panel = egui::SidePanel::right("debug")
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Debug Information");
            ui.separator();
            ui.label(format!("Theme: {}", config.theme));
            ui.label(format!("ROM: {}", config.rom));
            ui.separator();
            ui.label(format!("Program Counter: {}", chip8.program_counter));
            ui.label(format!("Index Register: {}", chip8.i));
            ui.label(format!("Stack Pointer: {}", chip8.stack_pointer));
            ui.label(format!(
                "Value at Stack Pointer: {}",
                chip8.stack[chip8.stack_pointer]
            ));
            ui.separator();
            for i in 0..16 {
                ui.label(format!("Register V{}: {}", i, chip8.registers[i]));
            }
            ui.separator();
            ui.label(format!("Delay Timer: {}", chip8.delay_timer));
            ui.label(format!("Sound Timer: {}", chip8.sound_timer));
            ui.separator();
            if ui.button("Reset Interpreter").clicked() {
                reset_flag.reset = true;
            };
        });

    camera.viewport = Some(Viewport {
        physical_position: UVec2 { x: 0, y: 0 },
        physical_size: UVec2 {
            x: window.physical_width(),
            y: window.physical_height(),
        },
        ..default()
    });

    Ok(())
}

fn setup_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    egui_global_settings.auto_create_primary_context = false;
    // Egui camera.
    commands.spawn((
        // The `PrimaryEguiContext` component requires everything needed to render a primary context.
        PrimaryEguiContext,
        Camera2d,
        // Setting RenderLayers to none makes sure we won't render anything apart from the UI.
        RenderLayers::none(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}

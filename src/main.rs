//! This is a open world RPG game where the skills, classes, magic and everything else are defined
//! using game-ingested YAML files at new game, and from then on the saved-game is its own world
//! which can be influenced by the player character and other characters in the game.
//!
//! Its goal is to be very open-ended with the experience it provides, letting the player make
//! not only decisions in game but before the game even begins to have an interesting experience.

#![warn(
    missing_docs,
    unreachable_code,
    unreachable_patterns,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![deny(unsafe_code)]

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    winit::WinitSettings,
};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui, WorldInspectorPlugin};
use systems::ingest_file_util::load_ingest_file_dir;

mod systems;

fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    App::new()
        // add logging, but filter out some noisy logs
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter:
                "info,untitled_bevy_game=debug,wgpu_core=warn,wgpu_hal=warn,wgpu_hal::auxil=error"
                    .into(),
            level: bevy::log::Level::DEBUG,
        }))
        // default plugins
        .add_plugins(DefaultPlugins)
        // fps diagnostic plugin
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // egui inspector
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(configure_visuals)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        // Testing the ingest system is main priority
        .add_startup_system(system_load)
        // Load the basic ui system
        .add_startup_system(basic_ui)
        // FPS text
        .add_system(text_update_system)
        .run();
}

fn system_load() {
    log::debug!("Test");
    load_ingest_file_dir("ingest_test");
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    let mut fonts = egui::FontDefinitions::default();
    // install your own font (.ttf and .otf supported)
    fonts.font_data.insert(
        "agavenf".to_string(),
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/agave regular Nerd Font Complete.ttf"
        )),
    );
    // insert it at the beginning for highest priority
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "agavenf".to_owned());
    egui_ctx.ctx_mut().set_fonts(fonts);
}

fn basic_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // See https://github.com/BorisBoutillier/Kataster/blob/main/src/ui.rs for some ideas

    // My custom text to spawn the title
    commands.spawn(
        TextBundle::from_section(
            "untitled bevy game",
            TextStyle {
                font: asset_server.load("fonts/FredokaOne-Regular.ttf"),
                font_size: 64.0,
                color: Color::ANTIQUE_WHITE,
            },
        )
        .with_text_alignment(TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        })
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Percent(20.0),
                left: Val::Percent(50.0),
                ..default()
            },
            size: Size::new(Val::Percent(100.0), Val::Px(64.0)),
            ..default()
        }),
    );

    // Text with multiple sections (has the 'FPS' label and the 'FPS' display)
    commands
        .spawn(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FredokaOne-Regular.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FredokaOne-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(FpsText);
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}

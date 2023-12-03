use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub const FPS_DISPLAY_X: f32 = -760.;
pub const FPS_DISPLAY_Y: f32 = 470.;

pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, spawn_fps_display)
            .add_systems(Update, update_fps_display)
            .add_systems(Update, toggle_display);
    }
}

#[derive(Component)]
struct FpsDisplay;

fn update_fps_display(
    mut score_text_query: Query<&mut Text, With<FpsDisplay>>,
    diagnostic: Res<DiagnosticsStore>,
) {
    for mut text in &mut score_text_query {
        if let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn toggle_display(
    mut visibility_query: Query<&mut Visibility, With<FpsDisplay>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F8) {
        let mut visible = visibility_query.single_mut();
        match *visible {
            Visibility::Visible => *visible = Visibility::Hidden,
            Visibility::Hidden => *visible = Visibility::Visible,
            Visibility::Inherited => *visible = Visibility::Hidden,
        }
    }
}

fn spawn_fps_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Black.ttf"),
                        font_size: 60.,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Roboto-Black.ttf"),
                    font_size: 60.,
                    ..default()
                }),
            ]),
            transform: Transform {
                translation: Vec3 {
                    x: FPS_DISPLAY_X,
                    y: FPS_DISPLAY_Y,
                    z: 1.,
                },

                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        FpsDisplay,
        Name::new("fps_display"),
    ));
}

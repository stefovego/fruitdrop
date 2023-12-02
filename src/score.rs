use bevy::prelude::*;



pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PlayerScore{ value: 0 })
        .add_systems(Startup, spawn_scoreboard)
        .add_systems(Update, update_scoreboard);
    }
}

#[derive(Component)]
struct Score;

#[derive(Resource)]
pub struct PlayerScore {
    pub value: u32 
}

fn update_scoreboard(
    mut score_text_query: Query<&mut Text, With<Score>>,
    player_score: Res<PlayerScore>
) {
    if player_score.is_changed(){
        for mut text in &mut score_text_query {
            let value = player_score.value;
            text.sections[0].value = format!("{value:.2}");
        }
    }
}


fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_score: Res<PlayerScore>
    ) {
        commands.spawn((
        TextBundle::from_section(player_score.value.to_string(), 
            TextStyle { 
                font: asset_server.load("fonts/Roboto-Black.ttf"), 
                font_size: 60., 
                ..default()
            }),
        Score,
        Name::new("scoreboard")
    ));
}


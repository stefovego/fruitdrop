use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ActionState<Action>>()
            .insert_resource(
                InputMap::default()
                    .insert(Action::MoveLeft, KeyCode::ArrowLeft)
                    .insert(Action::MoveLeft, KeyCode::KeyA)
                    .insert(Action::MoveRight, KeyCode::ArrowRight)
                    .insert(Action::MoveRight, KeyCode::KeyD)
                    .insert(Action::DropBall, KeyCode::Space)
                    .insert(Action::DropBall, MouseButton::Left)
                    .insert(Action::ToggleTune, KeyCode::KeyT)
                    .insert(Action::ToggleFps, KeyCode::KeyF)
                    .build(),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    MoveLeft,
    MoveRight,
    DropBall,
    ToggleTune,
    ToggleFps,
}

#[derive(Component)]
struct Controls;

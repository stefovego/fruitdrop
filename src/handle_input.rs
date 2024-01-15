use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ActionState<Action>>()
            .insert_resource(
                InputMap::default()
                    .insert(KeyCode::Left, Action::MoveLeft)
                    .insert(KeyCode::A, Action::MoveLeft)
                    .insert(KeyCode::Right, Action::MoveRight)
                    .insert(KeyCode::D, Action::MoveRight)
                    .insert(KeyCode::Space, Action::DropBall)
                    .insert(MouseButton::Left, Action::DropBall)
                    .insert(KeyCode::T, Action::ToggleTune)
                    .insert(KeyCode::F, Action::ToggleFps)
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

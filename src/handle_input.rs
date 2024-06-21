use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ActionState<Action>>()
            .insert_resource(Action::mk_input_map());
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

impl Action {
    fn mk_input_map() -> InputMap<Self> {
        InputMap::new([(Self::MoveLeft, KeyCode::ArrowLeft)])
            .with(Self::MoveRight, KeyCode::ArrowRight)
            .with(Self::DropBall, KeyCode::Space)
            .with(Self::ToggleTune, KeyCode::KeyT)
            .with(Self::ToggleFps, KeyCode::KeyF)
    }
}

#[derive(Component)]
struct Controls;

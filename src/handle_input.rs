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
    Pause,
}

impl Action {
    fn mk_input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::MoveLeft, KeyCode::ArrowLeft),
            (Self::MoveRight, KeyCode::ArrowRight),
            (Self::DropBall, KeyCode::Space),
            //(Self::DropBall, MouseButton::Left),
            (Self::ToggleTune, KeyCode::KeyT),
            (Self::ToggleFps, KeyCode::KeyF),
            (Self::Pause, KeyCode::Escape),
        ])
    }
}

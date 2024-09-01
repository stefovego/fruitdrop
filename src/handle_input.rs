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
            (Self::MoveLeft, InputKind::PhysicalKey(KeyCode::ArrowLeft)),
            (Self::MoveRight, InputKind::PhysicalKey(KeyCode::ArrowRight)),
            (Self::DropBall, InputKind::PhysicalKey(KeyCode::Space)),
            (Self::DropBall, InputKind::Mouse(MouseButton::Left)),
            (Self::ToggleTune, InputKind::PhysicalKey(KeyCode::KeyT)),
            (Self::ToggleFps, InputKind::PhysicalKey(KeyCode::KeyF)),
            (Self::Pause, InputKind::PhysicalKey(KeyCode::Escape)),
        ])
    }
}

#[derive(Component)]
struct Controls;

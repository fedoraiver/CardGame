pub mod components;
pub mod event;
pub mod systems;
pub mod util;

use crate::game_play::components::*;
use crate::game_play::event::*;
use crate::game_play::systems::hovering::*;
use crate::game_play::systems::movement::*;
use crate::game_play::systems::selection::*;
use crate::game_play::systems::setup::*;
use crate::game_play::systems::tilting::*;
use crate::resources::*;
use crate::states::AppState;

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Card>();
        app.register_type::<Hoverable>();
        app.register_type::<IsHovering>();
        app.register_type::<Tiltable>();
        app.register_type::<IsTilting>();
        app.register_type::<Selectable>();
        app.register_type::<IsSelected>();
        app.register_type::<Movable>();
        app.register_type::<MovableByCursor>();
        app.register_type::<IsMoving>();
        app.register_type::<MoveBasePosition>();
        app.register_type::<HoverBasePosition>();
        app.register_type::<CardShadow>();
        app.add_plugins(MeshPickingPlugin);
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
        app.add_plugins(Material2dPlugin::<MyTextureAtlasMaterial>::default());
        app.add_message::<SelectItem>();
        app.add_message::<UnSelectItem>();
        app.add_message::<MoveItem>();
        app.init_resource::<CursorPressedAtItem>();
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            OnTransition {
                exited: AppState::MainMenu,
                entered: AppState::InGame,
            },
            setup_background,
        );
        app.add_systems(
            Update,
            (tilt_card, move_card, hover_card, select_card).run_if(in_state(AppState::InGame)),
        );
    }
}

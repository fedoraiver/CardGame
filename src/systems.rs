use crate::{game_play::systems::mouse_input_handle::*, resources::*, util::*};

use bevy::prelude::*;

use crate::states::AppState;

pub fn toggle_pause_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state_current_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        match app_state_current_state.get() {
            AppState::InGame => app_state_next_state.set(AppState::Paused),
            AppState::Paused => app_state_next_state.set(AppState::InGame),
            _ => {}
        }
    }
}

pub fn quit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn register_my_observers(mut cmd: Commands) {
    cmd.spawn_batch([
        (
            Observer::new(cursor_over_at_hoverable_item),
            Name::new("cursor_over_at_hoverable_item_observer"),
        ),
        (
            Observer::new(mock_cursor_over_at_hoverable_item),
            Name::new("mock_cursor_over_at_hoverable_item_observer"),
        ),
        (
            Observer::new(cursor_over_at_tiltable_item),
            Name::new("cursor_over_at_tiltable_item"),
        ),
        (
            Observer::new(mock_cursor_over_at_tiltable_item),
            Name::new("mock_cursor_over_at_tiltable_item"),
        ),
        (
            Observer::new(cursor_move_at_tiltable_item),
            Name::new("cursor_move_at_hoverable_item"),
        ),
        (
            Observer::new(cursor_out_at_hoverable_item),
            Name::new("cursor_out_at_hoverable_item_observer"),
        ),
        (
            Observer::new(mock_cursor_out_at_hoverable_item),
            Name::new("mock_cursor_out_at_hoverable_item_observer"),
        ),
        (
            Observer::new(cursor_out_at_tiltable_item),
            Name::new("cursor_out_at_tiltable_item"),
        ),
        // (
        //     Observer::new(mock_cursor_out_at_tiltable_item),
        //     Name::new("mock_cursor_out_at_tiltable_item"),
        // ),
        (
            Observer::new(cursor_pressed_at_item),
            Name::new("cursor_pressed_at_item"),
        ),
        (
            Observer::new(cursor_click_at_selectable_item),
            Name::new("cursor_click_at_selectable_item_observer"),
        ),
        (
            Observer::new(mock_cursor_click_at_selectable_item),
            Name::new("mock_cursor_click_at_selectable_item_observer"),
        ),
        (
            Observer::new(cursor_drag_start_at_movable_by_cursor_item),
            Name::new("cursor_drag_start_at_movable_by_cursor_item_observer"),
        ),
        (
            Observer::new(cursor_drag_at_movable_by_cursor_item),
            Name::new("cursor_drag_at_movable_by_cursor_item_observer"),
        ),
        (
            Observer::new(cursor_drag_end_at_movable_by_cursor_item),
            Name::new("cursor_drag_end_at_movable_by_cursor_item_observer"),
        ),
    ]);
}

pub fn register_cards_aseprite_metadata(mut cmd: Commands) {
    cmd.insert_resource(CardsAsePriteMetadata::from(
        load_aseprite_metadata_from_json("assets/metadata/aseprite_cards.json"),
    ));
}

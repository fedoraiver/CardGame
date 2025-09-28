use crate::{game_play::systems::mouse_input_handle::*, resources::*, util::*};

use bevy::prelude::*;

use crate::states::AppState;

pub fn toggle_pause_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state_current_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state_current_state.get() {
            AppState::InGame => app_state_next_state.set(AppState::Paused),
            AppState::Paused => app_state_next_state.set(AppState::InGame),
            _ => {}
        }
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

#[cfg(feature = "bevy_mod_debugdump_plugin")]
pub fn output_render_graph(app: &mut App) {
    use bevy_mod_debugdump::*;
    let dot = render_graph_dot(app, &render_graph::Settings::default());
    if let Err(err) = std::fs::write("graph/render_graph.dot", dot) {
        error!("Failed to write render graph: {}", err);
    } else {
        info!("Render graph written to render_graph.dot");
    }
}

#[cfg(feature = "bevy_mod_debugdump_plugin")]
pub fn output_schedule_graph<L: bevy::ecs::schedule::ScheduleLabel>(
    app: &mut App,
    schedule_label: L,
) {
    use bevy_mod_debugdump::*;
    let dot = schedule_graph_dot(app, schedule_label, &schedule_graph::Settings::default());
    if let Err(err) = std::fs::create_dir_all("graph")
        .and_then(|_| std::fs::write("graph/schedule_graph.dot", dot))
    {
        error!("Failed to write schedule graph: {}", err);
    } else {
        info!("Schedule graph written to graph/schedule_graph.dot");
    }
}

pub fn register_cards_aseprite_metadata(mut cmd: Commands) {
    cmd.insert_resource(CardsAsePriteMetadata::from(
        load_aseprite_metadata_from_json("assets/metadata/aseprite_cards.json"),
    ));
}


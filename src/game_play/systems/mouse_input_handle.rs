use crate::{
    game_play::{components::*, event::*, util::window_to_world_position},
    resources::*,
};

use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct MockPointerOut {
    entity: Entity,
}
impl MockPointerOut {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
#[derive(EntityEvent)]
pub struct MockPointerOver {
    entity: Entity,
}
impl MockPointerOver {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(EntityEvent)]
pub struct MockPointerClick {
    entity: Entity,
}

pub fn cursor_over_at_hoverable_item(
    trigger: On<Pointer<Over>>,
    mut query: Query<(Entity, &mut Transform), With<Hoverable>>,
    mut cmd: Commands,
    mut z_index_manager: ResMut<ZIndexManager>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        return;
    }
    if let Ok((entity, mut transform)) = query.get_mut(trigger.event().event_target()) {
        cmd.entity(entity).insert((
            IsHovering,
            HoverBasePosition {
                position: transform.translation,
            },
        ));
        transform.translation.z = z_index_manager.next(&mut cmd);
        debug!(
            "Hovering over entity at position: ({}, {})",
            transform.translation.x, transform.translation.y
        );
    }
}

pub fn mock_cursor_over_at_hoverable_item(
    trigger: On<MockPointerOver>,
    mut query: Query<(Entity, &mut Transform), With<Hoverable>>,
    mut cmd: Commands,
    mut z_index_manager: ResMut<ZIndexManager>,
) {
    if let Ok((entity, mut transform)) = query.get_mut(trigger.event().event_target()) {
        cmd.entity(entity).insert((
            IsHovering,
            HoverBasePosition {
                position: transform.translation,
            },
        ));
        transform.translation.z = z_index_manager.next(&mut cmd);
        debug!(
            "Hovering over entity at position: ({}, {})",
            transform.translation.x, transform.translation.y
        );
    }
}

pub fn cursor_over_at_tiltable_item(
    trigger: On<Pointer<Over>>,
    query: Query<Entity, With<Tiltable>>,
    mut cmd: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        return;
    }
    if let Ok(entity) = query.get(trigger.event().event_target()) {
        cmd.entity(entity).insert(IsTilting::default());
    }
}

pub fn mock_cursor_over_at_tiltable_item(
    trigger: On<MockPointerOver>,
    query: Query<Entity, With<Tiltable>>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get(trigger.event().event_target()) {
        cmd.entity(entity).insert(IsTilting::default());
    }
}

pub fn cursor_move_at_tiltable_item(
    trigger: On<Pointer<Move>>,
    mut query: Query<(&mut IsTilting, &Transform), (With<Tiltable>, Without<IsMoving>)>,
) {
    if let Ok((mut is_tilting, transform)) = query.get_mut(trigger.event().event_target()) {
        if let Some(cursor_position) = trigger.event().event.hit.position {
            is_tilting.cursor_from_item_position =
                cursor_position.xy() - transform.translation.xy();
        }
    }
}

pub fn cursor_out_at_hoverable_item(
    trigger: On<Pointer<Out>>,
    mut query: Query<(Entity, &mut Transform, &HoverBasePosition), With<IsHovering>>,
    mut cmd: Commands,
) {
    if let Ok((entity, mut transform, base_position)) =
        query.get_mut(trigger.event().event_target())
    {
        transform.translation.x = base_position.position.x;
        transform.translation.y = base_position.position.y;
        cmd.entity(entity).remove::<IsHovering>();
        cmd.entity(entity).remove::<HoverBasePosition>();
    }
}

pub fn mock_cursor_out_at_hoverable_item(
    trigger: On<MockPointerOut>,
    mut query: Query<(Entity, &mut Transform, &HoverBasePosition), With<IsHovering>>,
    mut cmd: Commands,
) {
    if let Ok((entity, mut transform, base_position)) =
        query.get_mut(trigger.event().event_target())
    {
        transform.translation.x = base_position.position.x;
        transform.translation.y = base_position.position.y;
        cmd.entity(entity).remove::<IsHovering>();
        cmd.entity(entity).remove::<HoverBasePosition>();
    }
}

pub fn cursor_out_at_tiltable_item(
    trigger: On<Pointer<Out>>,
    query: Query<Entity, With<IsTilting>>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get(trigger.event().event_target()) {
        cmd.entity(entity).remove::<IsTilting>();
    }
}

pub fn cursor_pressed_at_item(
    trigger: On<Pointer<Press>>,
    mut cursor_pressed_at_item: ResMut<CursorPressedAtItem>,
) {
    cursor_pressed_at_item.position = trigger.event().pointer_location.position;
    debug!(
        "Cursor pressed at position: {:?}",
        cursor_pressed_at_item.position
    );
}

pub fn cursor_click_at_selectable_item(
    trigger: On<Pointer<Click>>,
    query: Query<Entity, With<Selectable>>,
    selected_query: Query<&IsSelected>,
    mut cmd: Commands,
    mut select_event_writer: MessageWriter<SelectItem>,
    mut unselect_event_writer: MessageWriter<UnSelectItem>,
    cursor_pressed_at_item: Res<CursorPressedAtItem>,
) {
    if let Ok(entity) = query.get(trigger.event().event_target()) {
        if trigger.event().pointer_location.position != cursor_pressed_at_item.position {
            debug!("Click position does not match pressed position, ignoring click.");
            return;
        }
        if selected_query.get(entity).is_ok() {
            cmd.entity(entity).remove::<IsSelected>();
            unselect_event_writer.write(UnSelectItem::new(entity));
            debug!("Entity deselected: {:?}", entity);
            debug!("{:?}", trigger);
        } else {
            cmd.entity(entity).insert(IsSelected);
            select_event_writer.write(SelectItem::new(entity));
            debug!("Entity selected: {:?}", entity);
            debug!("{:?}", trigger);
        }
    }
}

pub fn mock_cursor_click_at_selectable_item(
    trigger: On<MockPointerClick>,
    query: Query<Entity, With<Selectable>>,
    selected_query: Query<&IsSelected>,
    mut cmd: Commands,
    mut select_event_writer: MessageWriter<SelectItem>,
    mut unselect_event_writer: MessageWriter<UnSelectItem>,
) {
    if let Ok(entity) = query.get(trigger.event().event_target()) {
        if selected_query.get(entity).is_ok() {
            cmd.entity(entity).remove::<IsSelected>();
            unselect_event_writer.write(UnSelectItem::new(entity));
            debug!("Entity deselected: {:?}", entity);
        } else {
            cmd.entity(entity).insert(IsSelected);
            select_event_writer.write(SelectItem::new(entity));
            debug!("Entity selected: {:?}", entity);
        }
    }
}

pub fn cursor_drag_start_at_movable_by_cursor_item(
    trigger: On<Pointer<DragStart>>,
    mut query: Query<(Entity, &Transform), With<MovableByCursor>>,
    mut cmd: Commands,
) {
    if let Ok((entity, transform)) = query.get_mut(trigger.event().event_target()) {
        cmd.trigger(MockPointerOut::new(entity));
        cmd.entity(entity).insert(IsMoving);
        cmd.entity(entity)
            .insert(MoveBasePosition::new(Vec3::from(transform.translation)));
        debug!("Started dragging entity: {:?}", entity);
    }
}

pub fn cursor_drag_at_movable_by_cursor_item(
    trigger: On<Pointer<Drag>>,
    query: Query<Entity, With<IsMoving>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window>,
    mut move_event_writer: MessageWriter<MoveItem>,
) {
    if let Ok(entity) = query.get(trigger.event().event_target()) {
        move_event_writer.write(MoveItem {
            entity: (entity),
            delta_transform: (Transform::from_translation(
                window_to_world_position(trigger.event().delta, &q_camera, &q_window).extend(0.0),
            )),
        });
        trace!("move event: {:?}", trigger.event);
    }
}

pub fn cursor_drag_end_at_movable_by_cursor_item(
    trigger: On<Pointer<DragEnd>>,
    mut query: Query<Entity, With<IsMoving>>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get_mut(trigger.event().event_target()) {
        cmd.entity(entity).remove::<IsMoving>();
        cmd.entity(entity).remove::<MoveBasePosition>();
        cmd.trigger(MockPointerOver::new(entity));
        debug!("Stopped dragging entity: {:?}", entity);
    }
}

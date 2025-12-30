use bevy::prelude::*;

#[derive(Message)]
pub struct SelectItem {
    pub entity: Entity,
}

impl SelectItem {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Message)]
pub struct UnSelectItem {
    pub entity: Entity,
}

impl UnSelectItem {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Message)]
pub struct MoveItem {
    pub entity: Entity,
    pub delta_transform: Transform,
}

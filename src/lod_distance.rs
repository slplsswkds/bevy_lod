use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct LODDistances {
    pub l1: f32,
    pub l2: f32,
    pub l3: f32,
}

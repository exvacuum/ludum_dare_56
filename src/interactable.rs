use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Interactable;

#[derive(Event, Debug)]
pub struct InteractEvent(pub Entity);


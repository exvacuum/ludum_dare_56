use bevy::{asset::embedded_asset, prelude::*};

pub fn embed_assets(app: &mut App) {
    embedded_asset!(app, "embedded_assets", "./models/world.glb");
}

use bevy::{asset::embedded_asset, prelude::*};

pub fn embed_assets(app: &mut App) {
    embedded_asset!(app, "embedded_assets", "./models/world.glb");
    embedded_asset!(app, "embedded_assets", "./dialog/dialog.yarn");
    embedded_asset!(app, "embedded_assets", "./textures/pillbug.png");
    embedded_asset!(app, "embedded_assets", "./textures/ant.png");
}

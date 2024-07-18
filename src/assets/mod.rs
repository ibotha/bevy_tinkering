use bevy::prelude::*;
use texture_atlas::{TextureAndLayout, TextureAtlasLayoutLoader};
pub mod texture_atlas;

pub fn plugin(app: &mut App) {
    app.init_asset::<TextureAndLayout>();
    app.register_asset_loader(TextureAtlasLayoutLoader);
}

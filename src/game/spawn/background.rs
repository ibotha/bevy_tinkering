//! Spawn the background

use bevy::prelude::*;

use crate::{assets::texture_atlas::TextureAndLayout, screen::Screen};

use super::Depths;

#[derive(Resource, Default)]
struct TextureAtlases {
    building_atlas: Handle<TextureAndLayout>,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<TextureAtlases>();
    app.add_systems(Startup, load_sprites);
    app.observe(spawn_background);
}

#[derive(Event, Debug)]
pub struct SpawnBackground;

fn load_sprites(asset_server: Res<AssetServer>, mut handle: ResMut<TextureAtlases>) {
    handle.as_mut().building_atlas =
        asset_server.load::<TextureAndLayout>("images/spritesheet_default.atl");
}

fn spawn_background(
    _trigger: Trigger<SpawnBackground>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<TextureAtlases>,
    texture_atlas_assets: Res<Assets<TextureAndLayout>>,
    mut commands: Commands,
) {
    // Spawn in a background element, perhaps we can eventually have this
    // tied into the camera movement, spawning in instances to cover the
    // visible range. Making the background infinite.
    if let Some(texure_and_atlas) = texture_atlas_assets.get(&(texture_atlases.building_atlas)) {
        commands.spawn((
            Name::new("Buildings"),
            SpriteBundle {
                texture: texure_and_atlas.texture.clone(),
                transform: Transform {
                    translation: Vec3::new(0f32, 0f32, Depths::Buildings.into()),
                    ..default()
                },
                ..default()
            },
            StateScoped(Screen::Playing),
            TextureAtlas {
                layout: texure_and_atlas.layout.clone(),
                index: 80,
            },
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(1024f32 * 200f32)),
                    ..default()
                },
                texture: asset_server.load("images/parchmentBasic.png"),
                transform: Transform {
                    translation: Vec3::new(0f32, 0f32, Depths::Background.into()),
                    ..default()
                },
                ..default()
            },
            StateScoped(Screen::Playing),
            ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1f32,
            },
        ));
    } else {
        panic!("AAAAH it didn't exist!!!!")
    }
}

//! Spawn the background

use bevy::prelude::*;

use crate::{assets::texture_atlas::TextureAndLayout, screen::Screen};

use super::Depths;

struct TextureAtlases {
    building_atlas: Handle<TextureAtlasLayout>,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HoldTheHandle>();
    app.add_systems(Startup, load_sprites);
    app.observe(spawn_background);
}

#[derive(Event, Debug)]
pub struct SpawnBackground;

#[derive(Resource, Default)]
struct HoldTheHandle(Handle<TextureAndLayout>);

fn load_sprites(asset_server: Res<AssetServer>, mut handle: ResMut<HoldTheHandle>) {
    handle.as_mut().0 = asset_server.load::<TextureAndLayout>("images/spritesheet_default.atl");
}

fn spawn_background(
    _trigger: Trigger<SpawnBackground>,
    asset_server: Res<AssetServer>,
    texture_atlas_assets: Res<Assets<TextureAndLayout>>,
    mut commands: Commands,
) {
    // Spawn in a background element, perhaps we can eventually have this
    // tied into the camera movement, spawning in instances to cover the
    // visible range. Making the background infinite.
    /*
    commands
        .spawn((
            Name::new("Background"),
            Transform::default(),
            StateScoped(Screen::Playing),
        ))
        .with_children(|commands| {
    */
    let atlas_handle: Handle<TextureAndLayout> =
        asset_server.load("images/spritesheet_default.atl");
    if let Some(texure_and_atlas) = texture_atlas_assets.get(&atlas_handle) {
        commands.spawn((
            Name::new("Buildings"),
            SpriteBundle {
                sprite: Sprite {
                    //custom_size: Some(Vec2::splat(100f32)),
                    ..default()
                },
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
    //  });
}

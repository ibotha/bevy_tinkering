//! Spawn the background

use bevy::prelude::*;

use crate::screen::Screen;

use super::Depths;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_background);
}

#[derive(Event, Debug)]
pub struct SpawnBackground;

fn spawn_background(
    _trigger: Trigger<SpawnBackground>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // Spawn in a background element, perhaps we can eventually have this
    // tied into the camera movement, spawning in instances to cover the
    // visible range. Making the background infinite.
    commands
        .spawn((
            Name::new("Background"),
            Transform::default(),
            StateScoped(Screen::Playing),
        ))
        .with_children(|a| {
            a.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(1024f32 * 200f32)),
                        ..default()
                    },
                    texture: asset_server.load("images/parchmentBasic.png"),
                    transform: Transform {
                        translation: Vec3::new(0f32, 0f32, Depths::Background.into()),
                        scale: Vec3::splat(1f32),
                        ..default()
                    },
                    ..default()
                },
                ImageScaleMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 1f32,
                },
            ));
        });

    commands.spawn((
        Name::new("Buildings"),
        SpriteBundle {
            texture: asset_server.load("images/spritesheet_retina.png"),
            transform: Transform {
                translation: Vec3::new(0f32, 0f32, Depths::Buildings.into()),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

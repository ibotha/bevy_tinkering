//! Spawn the player.

use bevy::prelude::*;

use crate::screen::Screen;

use super::Depths;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: asset_server.load("images/ducky.png"),
            transform: Transform::from_translation(Vec3::new(
                0f32,
                0f32,
                Depths::MovingEntities.into(),
            )),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}

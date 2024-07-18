use bevy::prelude::*;
use crate::extensions::camera::shakies::{Shakies, shakies_plugin, ShakeEvent};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
       .add_systems(Update, test_camera_shake)
       .add_plugins(shakies_plugin);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        Shakies::default(),
        IsDefaultUiCamera,
    ));
}




// New test system
pub fn test_camera_shake(
    input: Res<ButtonInput<KeyCode>>,
    mut ev_shake: EventWriter<ShakeEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        ev_shake.send(ShakeEvent(0.5));
        info!("Test: Camera shake triggered!");
    }
}
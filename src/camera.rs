//! Camera setup.

use bevy::prelude::*;

use crate::camera_movement::{self, Movement, MovementController};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera_movement::plugin);
    app.add_systems(Startup, spawn_cameras);
    app.observe(observe_camera_swap);
}

#[derive(Event, PartialEq, Eq)]
pub enum CameraTypes {
    Menu,
    Game,
}

#[derive(Component, Deref)]
struct CameraType(CameraTypes);

fn spawn_cameras(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.near = -1000.0;
    camera_bundle.projection.far = 1000.0;
    camera_bundle.camera.clear_color = ClearColorConfig::Custom(Color::hsva(0.0, 0.0, 1.0, 1.0));

    commands.spawn((
        Name::new("Camera"),
        camera_bundle.clone(),
        IsDefaultUiCamera,
        CameraType(CameraTypes::Menu),
    ));

    camera_bundle.camera.is_active = false;

    commands.spawn((
        Name::new("Camera"),
        camera_bundle,
        Movement { speed: 420f32 },
        MovementController::default(),
        CameraType(CameraTypes::Game),
    ));
}

fn observe_camera_swap(
    trigger: Trigger<CameraTypes>,
    mut query: Query<(Entity, &mut Camera, &mut Transform, &CameraType)>,
    mut commands: Commands,
) {
    for (entity, mut camera, mut transform, cam_type) in &mut query {
        if cam_type.0 == *trigger.event() {
            commands.entity(entity).try_insert((IsDefaultUiCamera,));
            transform.translation = Vec3::default();
            camera.is_active = true;
        } else {
            commands.entity(entity).remove::<IsDefaultUiCamera>();
            camera.is_active = false;
        }
    }
}

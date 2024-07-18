//! Camera setup.

use bevy::{ecs::reflect::ReflectCommandExt, prelude::*};

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
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle { ..default() },
        IsDefaultUiCamera,
        CameraType(CameraTypes::Menu),
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
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

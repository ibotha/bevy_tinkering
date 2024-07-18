use bevy::{prelude::*, log::info};
use noise::{NoiseFn, Perlin};
use rand::random;

// Reference 
// Camera Shake for unity by Roystan Honks
// Url: https://roystan.net/articles/camera-shake/
#[derive(Resource)]
pub struct NoiseResource {
    perlin: Perlin,
}

impl Default for NoiseResource {
    fn default() -> Self {
        Self {
            perlin: Perlin::new(random()),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Shakies {
    pub maximum_translation_shake: Vec2,
    pub frequency: f32,
    pub shakies_exponent: f32,
    pub recovery_speed: f32,
    pub shakies: f32,
    pub original_position: Vec3,
}

impl Default for Shakies {
    fn default() -> Self {
        Self {
            maximum_translation_shake: Vec2::new(100.0, 100.0),
            frequency: 25.0,
            shakies_exponent: 2.0,
            recovery_speed: 1.0,
            shakies: 0.0,
            original_position: Vec3::ZERO,
        }
    }
}

impl Shakies {
    pub fn add_shakies(&mut self, amount: f32) {
        self.shakies = (self.shakies + amount).clamp(0.0, 1.0);
    }
}

#[derive(Event)]
pub struct ShakeEvent(pub f32);

pub fn trigger_camera_shake(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Shakies>,
) {
    if input.just_pressed(KeyCode::ShiftLeft) || input.just_pressed(KeyCode::ShiftRight) {
        info!("Shift key pressed! Attempting to trigger camera shake.");
        if let Ok(mut shakies) = camera_query.get_single_mut() {
            shakies.add_shakies(0.5);
            info!("Camera shake triggered. New shakies value: {}", shakies.shakies);
        } else {
            info!("Failed to get Shakies component. Is it attached to the camera entity?");
        }
    }
}

pub fn update_camera_shake(
    time: Res<Time>,
    noise: Res<NoiseResource>,
    mut query: Query<(&mut Transform, &mut Shakies)>,
) {
    for (mut transform, mut shake) in query.iter_mut() {
        let shake_amount = shake.shakies.powf(shake.shakies_exponent);
        
        let noise_fn = |offset: f64| {
            noise.perlin.get([
                offset, 
                time.elapsed_seconds() as f64 * shake.frequency as f64
            ]) as f32
        };

        let translation = Vec3::new(
            shake.maximum_translation_shake.x * noise_fn(0.0),
            shake.maximum_translation_shake.y * noise_fn(1.0),
            0.0
        ) * shake_amount;

        // Apply shake relative to the original position
        transform.translation = shake.original_position + translation;

        shake.shakies = (shake.shakies - shake.recovery_speed * time.delta_seconds()).max(0.0);

        if shake.shakies > 0.0 {
            info!("Applying codename 'shakies'. Shake: {}, Translation: {:?}", 
                  shake.shakies, translation);
        }
    }
}

pub fn handle_shake_events(
    mut events: EventReader<ShakeEvent>,
    mut camera_query: Query<&mut Shakies>,
) {
    for event in events.read() {
        if let Ok(mut shakies) = camera_query.get_single_mut() {
            shakies.add_shakies(event.0);
            info!("Camera shake triggered by event. New shakies value: {}", shakies.shakies);
        } else {
            info!("Failed to get Shakies component. Is it attached to the camera entity?");
        }
    }
}

pub fn shakies_plugin(app: &mut App) {
    app.init_resource::<NoiseResource>()
       .register_type::<Shakies>()
       .add_event::<ShakeEvent>()
       .add_systems(Update, (update_camera_shake, trigger_camera_shake, handle_shake_events));
}
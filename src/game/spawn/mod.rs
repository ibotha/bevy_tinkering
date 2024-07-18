//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod background;
pub mod level;
pub mod player;

// This enum is a single place for us to store all the standard z-depths
// for our game, lower down is closer to the camera.
#[repr(i32)]
enum Depths {
    Background = 1,
    Terrain,
    Buildings,
    MovingEntities,
}

impl Into<f32> for Depths {
    fn into(self) -> f32 {
        self as i32 as f32
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, background::plugin));
}

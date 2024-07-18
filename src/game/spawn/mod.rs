//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod background;
pub mod level;

// This enum is a single place for us to store all the standard z-depths
// for our game, lower down on the list is closer to the camera.
#[repr(i32)]
enum Depths {
    Base = 0,
    Background,
    Terrain,
    Buildings,
    MovingEntities,
}

impl From<Depths> for f32 {
    fn from(value: Depths) -> Self {
        value as i32 as f32
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, background::plugin));
}

// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use map_maker::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}

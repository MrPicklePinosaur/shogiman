mod board;

use bevy::{prelude::*, sprite::Material2dPlugin};
pub use board::*;

// TODO create plugin for all materials

pub struct MyMaterialPlugin;

impl Plugin for MyMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Material2dPlugin::<BoardMaterial>::default()));
    }
}

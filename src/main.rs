mod board;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, log::LogPlugin, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use board::BoardPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(LogPlugin {
            filter: "shogiman=debug".into(),
            level: bevy::log::Level::WARN,
        }),))
        .add_plugins((
            bevy_svg::prelude::SvgPlugin,
            bevy_mod_picking::prelude::DefaultPickingPlugins,
        ))
        .add_plugins((BoardPlugin))
        .add_systems(Startup, (setup_cam))
        // .add_systems(Update, ())
        .run();
}

fn setup_cam(mut cmd: Commands) {
    cmd.spawn((Camera2dBundle::default(), RaycastPickCamera::default()));
}

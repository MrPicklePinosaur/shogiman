mod board;
mod materials;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, log::LogPlugin, prelude::*, sprite::MaterialMesh2dBundle};
use board::BoardPlugin;
use materials::{BoardMaterial, MyMaterialPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(AssetPlugin {
                asset_folder: "assets".into(),
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            })
            .set(LogPlugin {
                filter: "shogiman=debug".into(),
                level: bevy::log::Level::WARN,
            }),))
        .add_plugins((bevy_svg::prelude::SvgPlugin))
        .add_plugins((BoardPlugin))
        .add_systems(Startup, (setup_cam))
        .add_plugins((MyMaterialPlugin))
        // .add_systems(Update, ())
        .run();
}

fn setup_cam(mut cmd: Commands) {
    cmd.spawn((Camera2dBundle::default()));
}

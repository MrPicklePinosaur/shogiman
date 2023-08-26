mod materials;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use materials::{BoardMaterial, MyMaterialPlugin};
use shogi::{bitboard::Factory, Move, Position};

#[derive(Resource, Deref, DerefMut)]
pub struct Board(pub Position);

impl Board {
    pub fn new() -> Self {
        let mut pos = Position::new();
        pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
            .unwrap();
        Board(pos)
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                asset_folder: "assets".into(),
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            }),
            DefaultPickingPlugins,
        ))
        .add_systems(Startup, (setup, setup_cam, render_game_board))
        .add_plugins((MyMaterialPlugin))
        // .add_systems(Update, ())
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.insert_resource(Board::new());
}

fn setup_cam(mut cmd: Commands) {
    cmd.spawn((Camera2dBundle::default(),));
}

fn render_game_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BoardMaterial>>,
) {
    let mesh_handle = meshes.add(Mesh::from(shape::Quad { ..default() }));

    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            transform: Transform::default().with_scale(Vec3::splat(256.)),
            material: materials.add(BoardMaterial {
                base_color: Color::BEIGE,
                grid_color: Color::BLUE,
                rows: 9,
                columns: 9,
            }),
            ..default()
        },
        PickableBundle::default(),
    ));
}

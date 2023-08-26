use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use shogi::{bitboard::Factory, Move, Position, Square};

use crate::materials::BoardMaterial;

#[derive(Resource, Deref, DerefMut)]
pub struct Board(pub Position);

impl Default for Board {
    fn default() -> Self {
        let mut pos = Position::new();
        pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
            .unwrap();
        Board(pos)
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_systems(Startup, (render_game_board, render_game_pieces));
    }
}

fn render_game_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BoardMaterial>>,
) {
    let mesh_handle = meshes.add(Mesh::from(shape::Quad { ..default() }));

    cmd.spawn((MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::default().with_scale(Vec3::splat(256.)),
        material: materials.add(BoardMaterial {
            base_color: Color::BEIGE,
            grid_color: Color::BLUE,
            rows: 9,
            columns: 9,
        }),
        ..default()
    },));
}

fn render_game_pieces(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, board: Res<Board>) {
    for square in Square::iter() {
        debug!("square {square:?}");
    }
}

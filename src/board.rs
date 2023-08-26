use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_svg::prelude::*;
use shogi::{bitboard::Factory, Move, Piece, PieceType, Position, Square};

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

fn render_game_pieces(mut cmd: Commands, board: Res<Board>, server: Res<AssetServer>) {
    // TODO make board resource that gets us position from file and rank
    let board_scale = 256.;

    for square in Square::iter() {
        if let Some(piece) = board.piece_at(square) {
            let handle = server.load(format!("sprites/{}", piece_to_sprite(piece)));

            let x_pos = (9. - square.file() as f32) / 9. * board_scale - board_scale / 2.;
            let y_pos = (9. - square.rank() as f32) / 9. * board_scale - board_scale;
            cmd.spawn(Svg2dBundle {
                svg: handle,
                origin: Origin::BottomLeft,
                transform: Transform::default()
                    // TODO proper 2d render order
                    .with_translation(Vec3::new(x_pos, y_pos, 1.0))
                    .with_scale(Vec3::splat(1.0)),
                ..default()
            });
        }
    }
}

/// Map the piece to the correct sprite to use
fn piece_to_sprite(piece: &Piece) -> String {
    // TODO: config for 王将 and 玉将
    let piece_type = match piece.piece_type {
        PieceType::King => "OU",
        PieceType::Rook => "HI",
        PieceType::Bishop => "KA",
        PieceType::Gold => "KI",
        PieceType::Silver => "GI",
        PieceType::Knight => "KE",
        PieceType::Lance => "KY",
        PieceType::Pawn => "FU",
        PieceType::ProRook => "RY",
        PieceType::ProBishop => "UM",
        PieceType::ProSilver => "NG",
        PieceType::ProKnight => "NK",
        PieceType::ProLance => "NY",
        PieceType::ProPawn => "TO",
    };

    let color = match piece.color {
        shogi::Color::Black => "0",
        shogi::Color::White => "1",
    };

    format!("{color}{piece_type}.svg")
}

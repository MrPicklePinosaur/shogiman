use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use bevy_svg::prelude::*;
use shogi::{bitboard::Factory, Piece, PieceType, Position, Square};

use crate::materials::BoardMaterial;

#[derive(Resource)]
pub struct Board {
    pub state: Position,
    pub scale: f32,
}

impl Default for Board {
    fn default() -> Self {
        let mut pos = Position::new();
        pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
            .unwrap();
        Board {
            state: pos,
            scale: 32.,
        }
    }
}

impl Board {
    /// Get the world position of a given square
    pub fn cell_transform(&self, square: &Square) -> Vec2 {
        let cell_size = self.scale / 2.;

        let x = (8. - square.file() as f32) * self.scale - self.scale * 9. / 2. + cell_size;
        let y = (8. - square.rank() as f32) * self.scale - self.scale * 9. / 2. + cell_size;
        Vec2::new(x, y)
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_systems(Startup, (render_game_board, render_game_pieces))
            .add_systems(Update, (board_gizmo));
    }
}

fn render_game_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BoardMaterial>>,
    board: Res<Board>,
) {
    let mesh_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::splat(board.scale * 9.),
        ..default()
    }));

    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: materials.add(BoardMaterial {
                base_color: Color::BEIGE,
                grid_color: Color::BLUE,
                rows: 9,
                columns: 9,
            }),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
        On::<Pointer<Move>>::run(on_hover),
        On::<Pointer<Click>>::run(on_click),
    ));
}

fn on_hover(evt: Listener<Pointer<Move>>, q: Query<(Entity, &Transform)>, board: Res<Board>) {
    if let Some(pos) = evt.hit.position {
        if let Ok(transform) = q.get_component::<Transform>(evt.target) {
            // hit position in local space
            let local_trans = transform.compute_matrix().inverse().transform_point3(pos);

            // hit position with bottom right as handle
            let offset_trans = local_trans.truncate() + Vec2::splat(board.scale * 9. / 2.);

            debug!("local {offset_trans:?}");

            // find out which grid square cursor is on
            // let raw_pos = (transform.translation - pos).truncate() - Vec2::splat(board.scale * 9. / 2.);
            // debug!("raw pos {raw_pos:?}");
        }
    }
}

fn on_click(evt: Listener<Pointer<Click>>) {
    // debug!("on hover event {evt:?}");
}

fn render_game_pieces(mut cmd: Commands, board: Res<Board>, server: Res<AssetServer>) {
    // TODO make board resource that gets us position from file and rank

    for square in Square::iter() {
        if let Some(piece) = board.state.piece_at(square) {
            let handle = server.load(format!("sprites/{}", piece_to_sprite(piece)));

            cmd.spawn(Svg2dBundle {
                svg: handle,
                origin: Origin::TopLeft,
                transform: Transform::default()
                    // TODO proper 2d render order
                    .with_translation(board.cell_transform(&square).extend(1.0)),
                ..default()
            });
        }
    }
}

fn board_gizmo(mut gizmos: Gizmos, board: Res<Board>) {
    for square in Square::iter() {
        gizmos.circle_2d(board.cell_transform(&square), 10., Color::RED);
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

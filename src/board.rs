use bevy::{
    input::keyboard::KeyboardInput, prelude::*, render::extract_resource::ExtractResource,
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::prelude::*;
use bevy_svg::prelude::*;
use shogi::{bitboard::Factory, Piece, PieceType, Position, Square};

use crate::materials::BoardMaterial;

/// Use to decide what color the cell should be
#[derive(Debug, Component, Default)]
pub struct CellHighlighter {
    /// Is the mouse hovering over the cell
    pub is_hovered: bool,
    /// Is the cell a potential position to move to
    pub is_move_target: bool,
}

#[derive(Debug, Resource)]
pub struct Board {
    pub state: Position,
    pub scale: f32,
}

#[derive(Debug, Resource, Default)]
pub struct ColorPalette {
    pub base: Handle<ColorMaterial>,
    pub hover: Handle<ColorMaterial>,
    pub move_target: Handle<ColorMaterial>,
}

/// Wrapper component for shogi square
#[derive(Debug, Component, Deref, DerefMut)]
pub struct BoardSquare(pub Square);

impl Default for Board {
    fn default() -> Self {
        // TODO might be issue if this gets called twice?
        Factory::init();

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

/// Highlight cells in the grid
// #[derive(ExtractResource, Clone)]
// pub struct BoardHighlight {
//     pub cells: [u32; 81]
// }

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<ColorPalette>()
            .add_systems(
                Startup,
                (init_materials, render_game_board, render_game_pieces),
            )
            .add_systems(
                Update,
                (
                    board_gizmo,
                    highlight_board
                        .run_if(|input: Res<Input<KeyCode>>| input.just_pressed(KeyCode::P)),
                    cell_highlighter,
                ),
            );
    }
}

fn init_materials(
    mut color_palette: ResMut<ColorPalette>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    color_palette.base = materials.add(ColorMaterial {
        color: Color::BEIGE,
        ..default()
    });

    color_palette.hover = materials.add(ColorMaterial {
        color: Color::BLUE,
        ..default()
    });

    color_palette.move_target = materials.add(ColorMaterial {
        color: Color::RED,
        ..default()
    });
}

fn render_game_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    color_palette: Res<ColorPalette>,
    board: Res<Board>,
) {
    let mesh_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::splat(board.scale),
        ..default()
    }));

    // TODO would be cool to batch this
    for square in Square::iter() {
        let square_entity = cmd
            .spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: color_palette.base.clone(),
                    transform: Transform::from_translation(
                        board.cell_transform(&square).extend(0.),
                    ),
                    ..default()
                },
                PickableBundle::default(),
                RaycastPickTarget::default(),
                On::<Pointer<Over>>::run(on_move_in),
                On::<Pointer<Out>>::run(on_move_out),
                On::<Pointer<Click>>::run(on_click),
                BoardSquare(square),
                CellHighlighter::default(),
            ))
            .id();
    }
}

fn highlight_board(
    mut q: Query<&mut Handle<BoardMaterial>>,
    mut materials: ResMut<Assets<BoardMaterial>>,
) {
    debug!("P pressed");
    for mut mat_handle in q.iter_mut() {
        if let Some(mat) = materials.get_mut(&mut *mat_handle) {
            mat.base_color = Color::RED;
        }
    }
}

fn on_move_in(evt: Listener<Pointer<Over>>, mut q: Query<(Entity, &mut CellHighlighter)>) {
    let mut hl = q.get_component_mut::<CellHighlighter>(evt.target).unwrap();
    hl.is_hovered = true;
}

fn on_move_out(evt: Listener<Pointer<Out>>, mut q: Query<(Entity, &mut CellHighlighter)>) {
    let mut hl = q.get_component_mut::<CellHighlighter>(evt.target).unwrap();
    hl.is_hovered = false;
}

fn cell_highlighter(
    mut cmd: Commands,
    q: Query<(Entity, &CellHighlighter), (Changed<CellHighlighter>)>,
    color_palette: Res<ColorPalette>,
) {
    for (entity, hl) in &q {
        if hl.is_move_target {
            cmd.entity(entity).insert(color_palette.move_target.clone());
        } else if hl.is_hovered {
            cmd.entity(entity).insert(color_palette.hover.clone());
        } else {
            cmd.entity(entity).insert(color_palette.base.clone());
        }
    }
}

fn on_click(evt: Listener<Pointer<Click>>, q: Query<(Entity, &BoardSquare)>, board: Res<Board>) {
    debug!("on hover event {evt:?}");

    // fetch the piece that is in the square
    // TODO failable systems would be nice
    let board_square = q.get_component::<BoardSquare>(evt.target).unwrap();
    if let Some(piece) = board.state.piece_at(**board_square) {
        debug!("clicked on {piece:?}");

        debug!("the side to move is {:?}", board.state.side_to_move());

        debug!("sqaure {board_square:?} piece {piece:?}");

        // draw an indictor for where the piece is allowed to move
        let moves = board.state.move_candidates(**board_square, *piece);

        for square in moves.into_iter() {
            debug!("square {square:?}");
        }
    }
}

fn render_game_pieces(mut cmd: Commands, board: Res<Board>, server: Res<AssetServer>) {
    // TODO make board resource that gets us position from file and rank

    for square in Square::iter() {
        if let Some(piece) = board.state.piece_at(square) {
            let handle = server.load(format!("sprites/{}", piece_to_sprite(piece)));

            cmd.spawn((
                Svg2dBundle {
                    svg: handle,
                    origin: Origin::TopLeft,
                    transform: Transform::default()
                        // TODO proper 2d render order
                        .with_translation(
                            board.cell_transform(&square).extend(1.0)
                                + Vec3::new(-board.scale / 2., board.scale / 2., 0.),
                        ),
                    ..default()
                },
                PickableBundle::default(),
                RaycastPickTarget::default(),
                // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                // }),
            ));
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

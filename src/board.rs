use std::{collections::HashMap, time::Duration};

use bevy::{
    ecs::query::Has, input::keyboard::KeyboardInput, prelude::*,
    render::extract_resource::ExtractResource, sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::prelude::*;
use bevy_svg::prelude::*;
use shogi::{bitboard::Factory, Piece, PieceType, Position, Square};

const BOARD_SCALE: f32 = 32.;

#[derive(Debug, Event)]
pub struct PieceMoveEvent {
    pub piece_id: PieceId,
    pub to: Square,
}

impl PieceMoveEvent {
    pub fn to(&self) -> Square {
        self.to
    }
    pub fn from(&self) -> Square {
        self.piece_id.square
    }
}

/// Event when a player gets to start their turn
#[derive(Debug, Event, Deref, DerefMut, PartialEq, Eq)]
pub struct TurnChangedEvent(pub shogi::Color);

/// Use to decide what color the cell should be
#[derive(Debug, Component, Default)]
pub struct CellHighlighter {
    /// Is the mouse hovering over the cell
    pub is_hovered: bool,
    /// Is the cell a potential position to move to
    pub is_move_target: bool,
}

/// Identifier for a piece
#[derive(Debug, Component, PartialEq, Eq)]
pub struct PieceId {
    pub piece: Piece,
    pub square: Square,
}

impl PieceId {
    pub fn new(piece: Piece, square: Square) -> Self {
        PieceId { piece, square }
    }
}

/// Represents which piece the player is currently considering on placing onto the board as well as
/// where the piece is
#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct Hand(pub Option<PieceId>);

#[derive(Debug, Resource)]
pub struct Board {
    pub state: Position,
    /// Access cell entity given sqaure index
    pub index_to_cell_entity: HashMap<usize, Entity>,
    /// Access a piece entity given a square index
    pub index_to_piece_entity: HashMap<usize, Entity>,
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
            index_to_cell_entity: HashMap::default(),
            index_to_piece_entity: HashMap::default(),
        }
    }
}

impl Board {
    /// Get the world position of a given square
    pub fn cell_transform(&self, square: &Square) -> Vec2 {
        let cell_size = BOARD_SCALE / 2.;

        let x = (8. - square.file() as f32) * BOARD_SCALE - BOARD_SCALE * 9. / 2. + cell_size;
        let y = (8. - square.rank() as f32) * BOARD_SCALE - BOARD_SCALE * 9. / 2. + cell_size;
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
            .init_resource::<Hand>()
            .add_event::<PieceMoveEvent>()
            .add_event::<TurnChangedEvent>()
            .add_systems(Startup, (init_materials, init_game_board, init_game_pieces))
            .add_systems(
                Update,
                (
                    // board_gizmo,
                    cell_highlighter,
                    piece_move_animator,
                    computer_move,
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

fn init_game_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    color_palette: Res<ColorPalette>,
    mut board: ResMut<Board>,
) {
    let mesh_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::splat(BOARD_SCALE),
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

        board
            .index_to_cell_entity
            .insert(square.index(), square_entity);
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

fn on_click(
    evt: Listener<Pointer<Click>>,
    q: Query<(Entity, &BoardSquare)>,
    mut q_hl: Query<(Entity, &mut CellHighlighter)>,
    mut board: ResMut<Board>,
    mut hand: ResMut<Hand>,
    mut evw_piece_move: EventWriter<PieceMoveEvent>,
    mut evw_turn_changed: EventWriter<TurnChangedEvent>,
) {
    // fetch the piece that is in the square
    // TODO failable systems would be nice
    let board_square = q.get_component::<BoardSquare>(evt.target).unwrap();

    // clear highlighting on all square first
    for (_, mut hl) in &mut q_hl {
        hl.is_move_target = false;
    }

    // if we have a piece in our hand and we clicked on a valid move spot, move the piece
    if let Some(PieceId { piece, square }) = **hand {
        // TODO maybe check if it's the players turn

        let next_move = shogi::Move::Normal {
            from: square,
            to: **board_square,
            promote: false,
        };
        if let Err(err) = board.state.make_move(next_move) {
            warn!("move error {err:?}");
        }

        **hand = None;

        // send event that we moved this piece
        evw_piece_move.send(PieceMoveEvent {
            piece_id: PieceId { piece, square },
            to: **board_square,
        });

        // TODO hardcoded for now
        evw_turn_changed.send(TurnChangedEvent(shogi::Color::White));
    } else {
        // otherwise place the piece in our hand and display potential moves
        if let Some(piece) = board.state.piece_at(**board_square) {
            debug!("clicked on {piece:?}");

            debug!("the side to move is {:?}", board.state.side_to_move());

            debug!("sqaure {board_square:?} piece {piece:?}");

            // draw an indictor for where the piece is allowed to move
            let moves = board.state.move_candidates(**board_square, *piece);

            // highlight squares
            for square in moves.into_iter() {
                let square_entity = board.index_to_cell_entity.get(&square.index()).unwrap();
                let mut target_square = q_hl
                    .get_component_mut::<CellHighlighter>(*square_entity)
                    .unwrap();
                target_square.is_move_target = true;
            }

            // Pick up piece to move
            // TODO hardcode player to be black player for now
            if board.state.side_to_move() == shogi::Color::Black {
                *hand = Hand(Some(PieceId::new(*piece, **board_square)));
            }
        }
    }
}

// TODO make board resource that gets us position from file and rank
fn init_game_pieces(mut cmd: Commands, mut board: ResMut<Board>, server: Res<AssetServer>) {
    for square in Square::iter() {
        if let Some(piece) = board.state.piece_at(square) {
            let handle = server.load(format!("sprites/{}", piece_to_sprite(piece)));

            let piece_id = cmd
                .spawn((
                    // TODO define actual layers
                    TransformBundle::from_transform(
                        Transform::default()
                            .with_translation(board.cell_transform(&square).extend(1.)),
                    ),
                    VisibilityBundle::default(),
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                    PieceId::new(*piece, square),
                    // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                    // }),
                ))
                .with_children(|parent| {
                    parent.spawn((Svg2dBundle {
                        svg: handle,
                        transform: Transform::from_xyz(-BOARD_SCALE / 2., BOARD_SCALE / 2., 0.),
                        origin: Origin::TopLeft,
                        ..default()
                    },));
                })
                .id();

            board.index_to_piece_entity.insert(square.index(), piece_id);
        }
    }
}

/// Animate the piece moves
fn piece_move_animator(
    mut cmd: Commands,
    mut evr_piece_move: EventReader<PieceMoveEvent>,
    q: Query<(Entity, &PieceId)>,
    board: Res<Board>,
) {
    use bevy_tweening::{lens::*, *};

    for ev in evr_piece_move.iter() {
        for (entity, piece_id) in &q {
            if *piece_id == ev.piece_id {
                debug!("move event for {entity:?}");

                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs(1),
                    TransformPositionLens {
                        start: board.cell_transform(&ev.from()).extend(1.),
                        end: board.cell_transform(&ev.to()).extend(1.),
                    },
                )
                .with_repeat_count(1);

                cmd.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

fn computer_move(
    mut board: ResMut<Board>,
    mut evr_turn_changed: EventReader<TurnChangedEvent>,
    mut evw_piece_move: EventWriter<PieceMoveEvent>,
) {
    for ev in &mut evr_turn_changed {
        if **ev == shogi::Color::White {
            debug!("computer's move");

            // Do fancy stuff to decide the next move

            // for now we do a random algorithm

            // choose a random piece that is able to move

            use rand::seq::SliceRandom;

            let bb = board.state.player_bb(shogi::Color::White);
            let mut shuffled_pieces = bb.into_iter().collect::<Vec<_>>();
            shuffled_pieces.shuffle(&mut rand::thread_rng());

            let Some((piece, from, to)) = shuffled_pieces.iter().find_map(|square| {
                let piece = board.state.piece_at(*square).unwrap();
                let mut moves = board
                    .state
                    .move_candidates(*square, piece)
                    .into_iter()
                    .collect::<Vec<_>>();
                if !moves.is_empty() {
                    // select a random move to make
                    moves.shuffle(&mut rand::thread_rng());
                    let random_move = moves.iter().next().unwrap().clone();

                    Some((piece, *square, random_move))
                } else {
                    None
                }
            }) else {
                error!("no moves for computer to make");
                return;
            };
            debug!("computer's move {piece:?} {from:?} {to:?}");

            // TODO this code is pretty duplicate
            let next_move = shogi::Move::Normal {
                from,
                to,
                promote: false,
            };
            if let Err(err) = board.state.make_move(next_move) {
                warn!("move error {err:?}");
            }

            // send event that we moved this piece
            evw_piece_move.send(PieceMoveEvent {
                piece_id: PieceId {
                    piece,
                    square: from,
                },
                to,
            });

            // evw_turn_changed.send(TurnChangedEvent(shogi::Color::Black));
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

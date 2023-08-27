//! Material for a shogi board
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "0f48c817-71aa-4377-a73c-69a1e16c486d"]
pub struct BoardMaterial {
    /// Main color of the board
    #[uniform(0)]
    pub base_color: Color,

    /// Color of the board grid lines
    #[uniform(0)]
    pub grid_color: Color,

    #[uniform(0)]
    pub rows: u32,

    #[uniform(0)]
    pub columns: u32,
    // /// Individual cell colors
    // /// first bit: unhighlight
    // /// second bit: highlight
    // /// NOTE: using array length of 84 to ensure stride length is multiple of 16
    // #[uniform(0)]
    // pub cell_colors: [u32; 81],
}

impl Material2d for BoardMaterial {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/board_material.wgsl".into()
    }
}

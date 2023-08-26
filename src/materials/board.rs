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
}

impl Material2d for BoardMaterial {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/board_material.wgsl".into()
    }
}

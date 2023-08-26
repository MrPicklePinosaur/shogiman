#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct BoardMaterial {
    board_color: vec4<f32>,
    grid_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: BoardMaterial;


@fragment
fn fragment(
    mesh: MeshVertexOutput
) -> @location(0) vec4<f32> {
    return material.board_color;
}

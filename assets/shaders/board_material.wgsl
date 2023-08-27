#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct BoardMaterial {
    board_color: vec4<f32>,
    grid_color: vec4<f32>,
    rows: u32,
    columns: u32,
    // cell_colors: array<u32, 81>,
};

@group(1) @binding(0)
var<uniform> material: BoardMaterial;


@fragment
fn fragment(
    mesh: MeshVertexOutput
) -> @location(0) vec4<f32> {
    var scale = f32(material.columns * material.rows);
    var offset = vec2<f32>(0.5, 0.5);

    var x_sample = fract(abs(mesh.uv.x * scale + offset.x) / f32(material.columns) ) * f32(material.columns);
    var y_sample = fract(abs(mesh.uv.y * scale + offset.y) / f32(material.rows) ) * f32(material.rows);

    if (x_sample < 1.0 || y_sample < 1.0) {
        return material.grid_color;
    } else {
        return material.board_color;
    }
}

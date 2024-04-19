#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> elapsed_time: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    //mesh.uv
    // return mesh.uv;
    // return material_color;

    let v = pow(cos(elapsed_time), 2.0);

    return vec4<f32>(mesh.uv.x, mesh.uv.y, v, 1.0);
}
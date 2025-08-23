// #import bevy_sprite::mesh2d_vertex_output::VertexOutput
struct VertexOutput {
    // this is `clip position` when the struct is used as a vertex stage output 
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
}

struct texture_atlas_layout {
    offset: vec2<f32>,
    size: vec2<f32>,
    texture_size: vec2<f32>,
}

@group(2) @binding(0)
var pattern_texture: texture_2d<f32>;
@group(2) @binding(1)
var pattern_sampler: sampler;
@group(2) @binding(2) var<uniform> texture_atlas_layout_info:texture_atlas_layout;

fn calc_uv_in_texture_atlas(in_uv: vec2f) -> vec2<f32> {
    return (texture_atlas_layout_info.offset + (texture_atlas_layout_info.size) * in_uv) / texture_atlas_layout_info.texture_size;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = calc_uv_in_texture_atlas(in.uv);
    return textureSample(pattern_texture, pattern_sampler, uv);
}

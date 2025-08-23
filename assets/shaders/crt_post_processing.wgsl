// #import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
struct FullscreenVertexOutput {
    @builtin(position)
    position: vec4<f32>,
    @location(0)
    uv: vec2<f32>,
};

// #import bevy_render::globals::Globals
struct Globals {
    // The time since startup in seconds
    // Wraps to 0 after 1 hour.
    time: f32,
    // The delta time since the previous frame in seconds
    delta_time: f32,
    // Frame count since the start of the app.
    // It wraps to zero when it reaches the maximum value of a u32.
    frame_count: u32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: f32
#endif
};

struct PostProcessSettings {
    saturation: f32,
    contrast: f32,
    gamma: f32,
    brightness: f32,
};

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var<uniform> globals: Globals;

fn saturate_color(color: vec3<f32>, saturation: f32) -> vec3<f32> {
    let gray = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    return mix(vec3<f32>(gray), color, saturation);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let q = in.uv;
    let uv = 0.5 + (q - vec2f(0.5)) * (0.99 + 0.01 * sin(0.2 * globals.time));

    let oricolor = textureSample(screen_texture, texture_sampler, vec2(q.x, q.y));
    var color = vec3f(0.0);

    color.r = textureSample(screen_texture, texture_sampler, vec2(uv.x + 0.001 * sin(0.4 * globals.time), uv.y)).r;
    color.g = textureSample(screen_texture, texture_sampler, vec2(uv.x + 0.001 * sin(0.4 * globals.time), uv.y)).g;
    color.b = textureSample(screen_texture, texture_sampler, vec2(uv.x + 0.001 * sin(0.4 * globals.time), uv.y)).b;

    color = clamp(color * 0.5 + 0.5 * color * color * 1.2, vec3f(0.0), vec3f(1.0));
    color *= 0.2 + 0.8 * 16.0 * uv.x * uv.y * (1.0 - uv.x) * (1.0 - uv.y);
    color *= vec3(0.95, 1.1, 0.95);
    color *= 0.9 + 0.1 * sin(10.0 * globals.time - uv.y * 1000.0);
    let blend = 1.5 * pow(length(q - vec2f(0.5)), 4.0);
    color = mix(color, oricolor.rgb, vec3f(blend));

    color = saturate_color(color, settings.saturation);
    color = (color - vec3f(0.5)) * settings.contrast + vec3f(0.5);
    color = pow(color, vec3<f32>(settings.gamma));
    color = color + vec3<f32>(settings.brightness);

    return vec4f(color, oricolor.a);
}
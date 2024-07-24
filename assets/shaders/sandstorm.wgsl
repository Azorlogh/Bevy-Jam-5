#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::globals::Globals

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
strength: f32,
blowout_factor: f32,
distort_strength: f32,
xspd: f32,
yspd: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>,
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var<uniform> globals: Globals;

fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }

fn noise2(n: vec2f) -> f32 {
    let d = vec2f(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2f(0.), vec2f(1.), fract(n));
    return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let strength = settings.strength;
    let blowout_factor = settings.blowout_factor;
    let distort_strength = settings.distort_strength;
    let xspd = settings.xspd;
    let yspd = settings.yspd;
    let time = globals.time;
    
    //setup texture scaling and movement
    let tex_x = ((in.uv.x * 4.0) + (time * xspd)) % 1.0 ;
    let tex_y = ((in.uv.y * 4.0) + (time * yspd)) % 1.0;
    
    let tex1_x = ((in.uv.x * 2.0) + (time * xspd)) % 1.0;
    let tex1_y = ((in.uv.y * 2.0) + (time * yspd)) % 1.0;
    
    let tex2_x = ((in.uv.x * 1.0) + (time * xspd)) % 1.0;
    let tex2_y = ((in.uv.y * 1.0) + (time * yspd)) % 1.0;
    
    //get 3 layers of texture
    let texColor = noise2(vec2(tex_x,tex_y) * 128);
    let texColor1 = noise2(vec2(tex1_x,tex1_y) * 128);
    let texColor2 = noise2(vec2(tex2_x,tex2_y) * 128);
    
    //mix texture layers  
    var t = max(max(texColor * 0.28, texColor1 * 0.34), texColor2 * 0.34);
  	t *= strength;
    
    //set storm intensity as distort
    let noise = smoothstep(0.11,0.45,t);
    let sceneNoise = in.uv + (vec2(noise*xspd,noise*yspd)*distort_strength);
    
    //get scene
    let sceneColor = textureSample(screen_texture, texture_sampler, sceneNoise).rgb;
    
    t *= blowout_factor;
    
    //mix colorized textures to scene
    return vec4<f32>(mix(sceneColor, vec3(0.65, 0.52, 0.44), t), 1.0);
}

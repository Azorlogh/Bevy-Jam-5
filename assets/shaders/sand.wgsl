#import bevy_pbr::pbr_types
#import bevy_pbr::pbr_functions::alpha_discard
#import bevy_pbr::pbr_functions
#import bevy_pbr::pbr_bindings
#import bevy_pbr::pbr_fragment
#import bevy_pbr::{
	mesh_view_bindings as view_bindings,
	mesh_view_types,
	mesh_types,
	shadows,
	lighting,
};
#import noise::simplex_vec3f::noise_simplex_vec3f

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct CustomMaterial {
    color: vec4<f32>,
};

const SAND_LIGHT: vec3f = vec3f(1.00, 0.75, 0.07);
const SAND_DARK: vec3f = vec3f(0.76,0.58,0.07);
// const ROCK_COLOR: vec3f = vec3f(0.12, 0.13, 0.13)*2.0;

const scale: f32 = 1.0+0.5+0.25+0.15+0.075;

fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }

fn mod289(x: vec4f) -> vec4f { return x - floor(x * (1. / 289.)) * 289.; }
fn perm4(x: vec4f) -> vec4f { return mod289(((x * 34.) + 1.) * x); }

fn noise3(p: vec3f) -> f32 {
    let a = floor(p);
    var d: vec3f = p - a;
    d = d * d * (3. - 2. * d);

    let b = a.xxyy + vec4f(0., 1., 0., 1.);
    let k1 = perm4(b.xyxy);
    let k2 = perm4(k1.xyxy + b.zzww);

    let c = k2 + a.zzzz;
    let k3 = perm4(c);
    let k4 = perm4(c + 1.);

    let o1 = fract(k3 * (1. / 41.));
    let o2 = fract(k4 * (1. / 41.));

    let o3 = o2 * d.z + o1 * (1. - d.z);
    let o4 = o3.yw * d.x + o3.xz * (1. - d.x);

    return o4.y * d.y + o4.x * (1. - d.y);
}

@fragment
fn fragment(
    in: VertexOutput,
	@builtin(front_facing) is_front: bool,
) -> FragmentOutput {

	var pbr_input = pbr_fragment::pbr_input_from_standard_material(in, is_front);

  let t = view_bindings::globals.time;

	// pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

	let pos = in.world_position.xyz*100.0;
	// let noise = (
	// 	noise_simplex_vec3f(pos) +
	// 	noise_simplex_vec3f(pos*3.0)*0.5 +
	// 	noise_simplex_vec3f(pos*9.0)*0.25 +
	// 	noise_simplex_vec3f(pos*27.0)*0.15 +
	// 	noise_simplex_vec3f(pos*81.0)*0.075)/scale*0.5+0.5;
	

	// let rock = ROCK_COLOR*smoothstep(0.1, 0.9, noise);
	// let grass = GRASS_COLOR*noise;
	// let rock = vec3f(1);
	// let grass = vec3f(0);

	// pbr_input.material.base_color = vec4f(mix(rock, grass, vec3f(smoothstep(0.5, 0.7, in.world_normal.y))), 1.0);

	let noise_col = noise3(pos);

	let noise_normal = vec2f(noise3(pos + vec3f(100.0, 0.0, 0.0)), noise3(pos + vec3f(200.0, 0.0, 0.0)));


	// let Nt = noise_normal;

	// let TBN = pbr_functions::calculate_tbn_mikktspace(pbr_input.world_normal, in.world_tangent);

	// pbr_input.N = pbr_functions::apply_normal_mapping(
	// 	pbr_bindings::material.flags,
	// 	TBN,
	// 	double_sided,
	// 	is_front,
	// 	Nt,
	// );

	let waves = noise_simplex_vec3f(in.world_position.xyz*0.04*vec3f(6.0, 1.0, 1.0) + vec3f(0.2131, 0.31234, 0.52141) * t * 0.2)*0.5+0.5;

	pbr_input.N = normalize(pbr_input.N + vec3f(noise_normal.x, 0.0, noise_normal.y)*0.2+vec3f(0, -0.5, 0)*waves);



	pbr_input.material.base_color = vec4f(mix(SAND_LIGHT, SAND_DARK, noise3(pos)), 1.0);
	pbr_input.material.perceptual_roughness = (noise3(pos + vec3f(300.0, 0.0, 0.0))*0.5+0.5);

#ifdef PREPASS_PIPELINE
	let out = deferred_output(in, pbr_input);
#else
	var out: FragmentOutput;
	out.color = apply_pbr_lighting(pbr_input);
	out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif
	return out;
}



// let out_color = pbr_functions::apply_pbr_lighting(pbr_input);

// return pbr_functions::main_pass_post_lighting_processing(pbr_input, out_color);

// return vec4<f32>(1.0, 0.0, 0.0, 1.0);
// return vec4f(vec3f(noise_simplex_vec3f(in.world_position.xyz/10.0)), 1.0);


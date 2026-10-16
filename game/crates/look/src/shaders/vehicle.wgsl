// The vehicles' paint. The baked models' textures are dark and clean, and seen from a drone a kilometre off they
// were dark blobs that merged with the road. Here the texture gives only its panel lines and shading (its
// luminance against its own mean), and the colour is painted in the hull's frame: each side's camouflage for the
// season, the running gear dark, mud thrown up the lower hull, the decks pale with dust or frost and worn to the
// steel where the crew climb. A burned-out wreck is rust, soot and pale ash instead, so it reads apart from the living.

#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    mesh_functions::get_world_from_local,
    mesh_bindings::mesh,
}

struct Livery {
    // x: the scheme (0 Russian green, 1 NATO three-tone, 2 Ukrainian green, 3 an aircraft's own paint),
    // y: whitewash, z: 1 in June, w: a seed.
    scheme: vec4<f32>,
    // x: the hull's half width, y: its height, z: this mesh's frame above the hull's (a turret's ring), w: 1 burned
    // out.
    frame: vec4<f32>,
    // x: 0 paint, 1 bare steel and rubber, 2 keeps its colour; y: the texture's mean luminance (0 untextured);
    // z: 1 vertex-coloured (built in code).
    stuff: vec4<f32>,
}

// Bindless where the standard material is (each material's livery is a slot in one array), a plain uniform
// where it is not.
#ifdef BINDLESS
struct LiveryIndices {
    livery: u32,
}
@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<storage> livery_indices: array<LiveryIndices>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var<storage> liveries: array<Livery>;
#else
@group(#{MATERIAL_BIND_GROUP}) @binding(50) var<uniform> livery: Livery;
#endif

// Paint colours are given as sRGB, as one would pick them, and lit linear.
fn lin(c: vec3<f32>) -> vec3<f32> {
    return pow(c, vec3<f32>(2.2));
}

fn hash3(p: vec3<f32>) -> f32 {
    var q = fract(p * 0.1031);
    q += dot(q, q.yzx + 33.33);
    return fract((q.x + q.y) * q.z);
}

// Value noise in the hull's frame, 0..1: patches that wrap round the hull's faces as paint does.
fn vnoise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = mix(hash3(i), hash3(i + vec3<f32>(1.0, 0.0, 0.0)), u.x);
    let b = mix(hash3(i + vec3<f32>(0.0, 1.0, 0.0)), hash3(i + vec3<f32>(1.0, 1.0, 0.0)), u.x);
    let c = mix(hash3(i + vec3<f32>(0.0, 0.0, 1.0)), hash3(i + vec3<f32>(1.0, 0.0, 1.0)), u.x);
    let d = mix(hash3(i + vec3<f32>(0.0, 1.0, 1.0)), hash3(i + vec3<f32>(1.0, 1.0, 1.0)), u.x);
    return mix(mix(a, b, u.y), mix(c, d, u.y), u.z);
}

fn fbm(p: vec3<f32>) -> f32 {
    return vnoise(p) * 0.62 + vnoise(p * 2.07 + 5.1) * 0.28 + vnoise(p * 4.3 + 9.7) * 0.1;
}

// A hard paint edge at `t` that still does not alias when the patch is a few pixels across.
fn edge(t: f32, v: f32) -> f32 {
    let w = 0.012 + fwidth(v);
    return smoothstep(t - w, t + w, v);
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    let base = pbr_input.material.base_color;
#ifdef BINDLESS
    let slot = mesh[in.instance_index].material_and_lightmap_bind_group_slot & 0xffffu;
    let l = liveries[livery_indices[slot].livery];
#else
    let l = livery;
#endif
    let scheme = l.scheme;
    let frame = l.frame;
    let stuff = l.stuff;

    // The fragment in the hull's frame: metres, y up from the ground under the tracks, the front towards -z.
    let m = get_world_from_local(in.instance_index);
    let ax = m[0].xyz;
    let ay = m[1].xyz;
    let az = m[2].xyz;
    let rel = in.world_position.xyz - m[3].xyz;
    var p = vec3<f32>(dot(rel, ax) / dot(ax, ax), dot(rel, ay) / dot(ay, ay), dot(rel, az) / dot(az, az));
    p.y += frame.z;
    let wn = pbr_input.world_normal;
    let n = normalize(vec3<f32>(dot(wn, ax), dot(wn, ay), dot(wn, az)));
    let up = smoothstep(0.45, 0.85, n.y);
    let seed = vec3<f32>(scheme.w, scheme.w * 0.37, scheme.w * 0.71);
    let summer = scheme.z > 0.5;
    let h = p.y;

    // What this surface is: paint, steel and rubber, or something that keeps its own colour. The parts built in
    // code are vertex-coloured; their green is paint, their grey steel.
    var kind = stuff.x;
    if stuff.z > 0.5 && !(base.g > base.r && base.g > base.b * 1.25) {
        kind = 1.0;
    }
    // The texture's own shading and panel lines, as a factor about its mean.
    var detail = 1.0;
    if stuff.y > 0.0 {
        detail = clamp(dot(base.rgb, vec3<f32>(0.2126, 0.7152, 0.0722)) / stuff.y, 0.35, 1.7);
        detail = mix(1.0, detail, 0.85);
    }

    var c = base.rgb;
    var rough = 0.55;
    if frame.w > 0.5 {
        // Burned out: the paint gone to rust where the fire was hottest, sooted above the hatches and the engine
        // deck, the decks gone to pale ash, the tracks and the mud below black.
        let r = fbm(p * 0.9 + seed);
        c = mix(lin(vec3<f32>(0.15, 0.10, 0.07)), lin(vec3<f32>(0.36, 0.21, 0.12)), smoothstep(0.35, 0.75, r));
        let soot = smoothstep(0.36, 0.6, fbm(p * vec3<f32>(1.3, 0.35, 1.3) + seed + 11.0));
        c = mix(c, lin(vec3<f32>(0.07, 0.065, 0.06)), soot * 0.9);
        let ash = up * smoothstep(0.42, 0.66, fbm(p * 1.6 + seed + 23.0));
        c = mix(c, lin(vec3<f32>(0.47, 0.45, 0.42)), ash * 0.5);
        c = mix(c, lin(vec3<f32>(0.09, 0.08, 0.07)), 1.0 - smoothstep(0.45, 0.85, h + (vnoise(p * 2.0) - 0.5) * 0.3));
        if kind > 1.5 {
            c = lin(vec3<f32>(0.08, 0.075, 0.07));
        }
        c *= detail;
        rough = 0.95;
    } else {
        if kind < 0.5 {
            if scheme.x < 0.5 {
                // Russian 4BO green, faded in patches to a dull olive, stained darker in others; at Vuhledar
                // what is left of a winter whitewash, brushed on in streaks and worn through.
                let mott = fbm(p * 0.7 + seed);
                c = mix(lin(vec3<f32>(0.25, 0.27, 0.18)), lin(vec3<f32>(0.37, 0.39, 0.26)), smoothstep(0.25, 0.5, mott));
                c = mix(c, lin(vec3<f32>(0.47, 0.47, 0.35)), smoothstep(0.55, 0.8, mott) * 0.7);
                let wash = fbm(p * vec3<f32>(0.42, 0.6, 0.42) + seed + 3.0);
                let streak = vnoise(p * vec3<f32>(2.4, 4.5, 2.4) + seed);
                let ww = edge(0.6, wash) * (0.45 + 0.55 * streak) * scheme.y;
                c = mix(c, lin(vec3<f32>(0.74, 0.74, 0.70)), ww * 0.9);
            } else if scheme.x < 1.5 {
                // NATO three-tone: a green field with large hard-edged patches of brown and black.
                let a = fbm(p * 0.42 + seed);
                let b = fbm(p * 0.48 + seed + 31.7);
                c = lin(vec3<f32>(0.30, 0.35, 0.20));
                c = mix(c, lin(vec3<f32>(0.38, 0.28, 0.18)), edge(0.55, a));
                c = mix(c, lin(vec3<f32>(0.11, 0.11, 0.10)), edge(0.6, b));
            } else if scheme.x < 2.5 {
                // Ukrainian green, plain and dull, mottled with wear.
                let mott = fbm(p * 0.7 + seed);
                c = mix(lin(vec3<f32>(0.24, 0.28, 0.18)), lin(vec3<f32>(0.35, 0.39, 0.26)), smoothstep(0.25, 0.55, mott));
            }
            if stuff.z > 0.5 || scheme.x > 2.5 {
                // An aircraft keeps its own paint; so do the parts built in code, but under the same wear.
                c = select(c, base.rgb * 1.3, scheme.x > 2.5);
                c = select(c, mix(base.rgb, c, 0.6), stuff.z > 0.5);
            }
            c *= detail;
            // Paint worn through to the primer and the steel on the decks, where the crew climb.
            let fine = 1.0 - smoothstep(0.015, 0.05, fwidth(p.x));
            let wear = step(0.78, vnoise(p * 11.0 + seed)) * up * fine;
            c = mix(c, lin(vec3<f32>(0.46, 0.45, 0.41)), wear * 0.5);
        } else if kind < 1.5 {
            // Steel and rubber: the running gear, the guns and the grilles, dark and a little glossy.
            c = base.rgb * 1.1;
            rough = 0.45;
        }
        if scheme.x < 2.5 {
            // The running gear in the shadow of the hull, grimed dark.
            let gear = step(frame.x - 0.6, abs(p.x)) * (1.0 - smoothstep(0.6, 1.0, h));
            c *= mix(1.0, 0.6, gear);
            // The decks catch the sky, and the dust or the frost settles on them; in June the steppe's dust coats
            // everything a little.
            let settle = up * (0.5 + 0.7 * fbm(p * 1.3 + seed + 41.0));
            if summer {
                let dust = lin(vec3<f32>(0.62, 0.56, 0.45));
                c = mix(c, dust, clamp(0.03 + settle * 0.14, 0.0, 1.0));
                rough = mix(rough, 0.85, settle * 0.5);
            } else {
                c = mix(c, lin(vec3<f32>(0.68, 0.69, 0.68)), clamp(settle * 0.12, 0.0, 1.0));
            }
            // Mud: thick on the tracks and the lower hull, a ragged line above them, and thrown up the sides and
            // the glacis in splashes. Wet and dark in February with paler dried crusts; dry earth and dust in June.
            let ragged = h + (fbm(p * vec3<f32>(2.0, 0.8, 2.0) + seed) - 0.5) * 0.55;
            let splash = smoothstep(0.62, 0.74, vnoise(p * 3.2 + seed + 7.0)) * (1.0 - smoothstep(0.9, 1.8, h));
            let mud = max(1.0 - smoothstep(0.5, 1.0, ragged), splash * 0.75);
            let crust = vnoise(p * 2.4 + seed + 17.0);
            var mud_c = mix(lin(vec3<f32>(0.24, 0.21, 0.17)), lin(vec3<f32>(0.43, 0.40, 0.36)), smoothstep(0.45, 0.75, crust));
            if summer {
                mud_c = mix(lin(vec3<f32>(0.30, 0.26, 0.20)), lin(vec3<f32>(0.44, 0.39, 0.30)), smoothstep(0.45, 0.8, crust));
            }
            c = mix(c, mud_c * mix(1.0, detail, 0.5), mud * 0.9);
            rough = mix(rough, select(0.6, 0.95, summer), mud);
        }
    }

    pbr_input.material.base_color = vec4<f32>(c, base.a);
    pbr_input.material.perceptual_roughness = rough;
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}

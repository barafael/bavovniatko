// Ground detail below the 1 m texel: furrows and stubble rows showing through thin snow, and grain in the snow.
// The base texture's alpha carries the field's furrow direction (0 = no furrows here).

#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct GroundDetail {
    params: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> ground: GroundDetail;

fn hash(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(127.1, 311.7));
    q += dot(q, q + 19.19);
    return fract(q.x * q.y);
}

fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    let base = pbr_input.material.base_color;
    let code = base.a;
    var c = base.rgb;
    let w = in.world_position.xz;

    if code > 0.002 {
        let ang = code * 3.14159265;
        let dir = vec2<f32>(cos(ang), sin(ang));
        let along = dot(w, dir);
        let across = dot(w, vec2<f32>(-dir.y, dir.x));
        // Fade out before the rows alias into moire.
        let fw = fwidth(along);
        let fade = 1.0 - smoothstep(0.05, 0.2, fw);
        let row = abs(fract(along / 0.75) - 0.5) * 2.0;
        let ridge = 1.0 - smoothstep(0.0, 0.45, row);
        // Thin snow (darker base) lets more of the earth through.
        let lum = dot(c, vec3<f32>(0.3, 0.59, 0.11));
        let thin = 1.0 - smoothstep(0.38, 0.62, lum);
        let stub = ridge * step(0.6, hash(floor(vec2<f32>(across * 2.5, along / 0.75))));
        // Drifted snow hides the rows in patches; they show only where the wind has scoured it thin.
        let drift = smoothstep(0.4, 0.75, vnoise(w / 9.0) * 0.5 + vnoise(w / 31.0 + 3.7) * 0.5);
        let k = (ridge * mix(0.03, 0.28, thin) + stub * mix(0.04, 0.18, thin)) * drift * fade * ground.params.x;
        c = mix(c, vec3<f32>(0.07, 0.055, 0.045), k);
        // At distance, the rows average out into a slightly darker field.
        c = mix(c, c * (1.0 - 0.18 * mix(0.3, 1.0, thin)), 1.0 - fade);
    }
    // Wind-packed grain in the snow.
    let fg = fwidth(w.x);
    let g = (hash(floor(w * 3.0)) - 0.5) * 0.05 * (1.0 - smoothstep(0.1, 0.4, fg)) * ground.params.y;
    c = c * (1.0 + g);

    pbr_input.material.base_color = vec4<f32>(c, 1.0);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}

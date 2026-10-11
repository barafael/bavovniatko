// Ground detail below the 1 m texel: the snow's ragged patch edges, furrows and stubble rows showing through
// thin snow, and grain in the snow. The base texture's alpha carries the field's furrow direction (0 = no
// furrows here). The texture is sampled through a small world-space jitter, so no texel edge is ever straight.

#import bevy_pbr::{
    mesh_view_bindings::fog,
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

// Two octaves of value noise, -0.5..0.5.
fn fbm2(p: vec2<f32>) -> f32 {
    return (vnoise(p) - 0.5) * 0.65 + (vnoise(p * 2.3 + 11.0) - 0.5) * 0.35;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    let w = in.world_position.xz;
    // Sample the painted texel through a jitter of up to 0.35 m, so a texel's square edge wanders.
    var jittered = in;
    let near_tex = 1.0 - smoothstep(0.3, 1.2, fwidth(w.x));
    let jitter = vec2<f32>(fbm2(w * 0.9), fbm2(w * 0.9 + 37.0)) * 0.7 * near_tex / 256.0;
    jittered.uv = in.uv + jitter;
    var pbr_input = pbr_input_from_standard_material(jittered, is_front);
    let base = pbr_input.material.base_color;
    let code = base.a;
    var c = base.rgb;

    if ground.params.z > 0.5 {
        // June: rows of plants where a field was sown (last year's dead sunflowers, this year's few crops), and
        // the mottle of grass and weeds everywhere else.
        if code > 0.002 {
            let ang = code * 3.14159265;
            let dir = vec2<f32>(cos(ang), sin(ang));
            let along = dot(w, dir);
            let across = dot(w, vec2<f32>(-dir.y, dir.x));
            let fade = 1.0 - smoothstep(0.05, 0.2, fwidth(along));
            let row = abs(fract(along / 0.7) - 0.5) * 2.0;
            let ridge = 1.0 - smoothstep(0.0, 0.5, row);
            let plant = ridge * step(0.35, hash(floor(vec2<f32>(across * 3.0, along / 0.7))));
            c = mix(c, vec3<f32>(0.15, 0.21, 0.07), (ridge * 0.22 + plant * 0.25) * fade * ground.params.x);
            c = mix(c, c * 0.9, 1.0 - fade);
        }
        let fg = fwidth(w.x);
        let near = 1.0 - smoothstep(0.1, 0.5, fg);
        let clump = (vnoise(w * 1.3) - 0.5) * 0.18 + (vnoise(w * 0.31 + 7.0) - 0.5) * 0.12;
        let blade = (hash(floor(w * 5.0)) - 0.5) * 0.09 * near;
        c = c * (1.0 + (clump * mix(0.5, 1.0, near) + blade) * ground.params.y);
    } else {
        // The snow's patches below the texel: where the texel says the cover is partial, cut the earth and the
        // snow apart along a ragged world-space edge, keeping the texel's mean. Full snow and full earth, and
        // the painted craters, scorch and tracks, pass through unchanged.
        let lum0 = dot(c, vec3<f32>(0.3, 0.59, 0.11));
        let cover = smoothstep(0.30, 0.72, lum0);
        let partial = cover * (1.0 - cover) * 4.0;
        let grain = fbm2(w * 0.55) + fbm2(w * 2.1) * 0.5;
        let cut = smoothstep(-0.12, 0.12, grain + (cover - 0.5) * 0.6);
        let m = (cut - cover) * partial * near_tex;
        c = mix(c * vec3<f32>(0.62, 0.58, 0.52), min(c * 1.18, vec3<f32>(0.95)), 0.5 + 0.5 * m);
        c = mix(base.rgb, c, abs(m) * 1.5);
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
        // Wind-packed grain in the snow, and clods and crust at a quarter metre.
        let fg = fwidth(w.x);
        let close = 1.0 - smoothstep(0.1, 0.4, fg);
        let g = (hash(floor(w * 3.0)) - 0.5) * 0.05 * close * ground.params.y;
        let crust = (vnoise(w * 4.0) - 0.5) * 0.06 * close;
        c = c * (1.0 + g + crust);
    }
    pbr_input.material.base_color = vec4<f32>(c, 1.0);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    // The diorama has no edge: the last 600 m of ground dissolve into the haze.
    let edge = min(2304.0 - abs(w.x), 1792.0 - abs(w.y));
    let e = 1.0 - smoothstep(0.0, 600.0, edge);
    out.color = vec4<f32>(mix(out.color.rgb, fog.base_color.rgb, e * e), out.color.a);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}

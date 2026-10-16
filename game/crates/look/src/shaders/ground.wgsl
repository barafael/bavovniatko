// Ground detail below the half-metre texel: the snow's ragged patch edges, furrows and stubble rows showing through
// thin snow, and grain in the snow. The base texture's alpha carries the field's furrow direction (0 = no
// furrows here). The texture is sampled through a small world-space jitter, so no texel edge is ever straight.

#import bevy_pbr::{
    mesh_view_bindings::{fog, lights, view},
    mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT,
    shadows::fetch_directional_shadow,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct GroundDetail {
    params: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> ground: GroundDetail;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var noise_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var noise_smp: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(103) var occ_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(104) var occ_smp: sampler;

// The noise comes from a 256-texel tiling texture of random values in four independent channels: one lattice
// point per texel, filtered, is value noise for the price of a read (the shader used to hash every lattice point,
// 45 hashes a pixel, and the ground was most of the frame). `lod` is the mip for the lattice's size on screen, so
// far away the noise fades to its mean instead of aliasing.
const NOISE: f32 = 256.0;

fn vn(p: vec2<f32>, lod: f32) -> vec4<f32> {
    return textureSampleLevel(noise_tex, noise_smp, p / NOISE, lod);
}

// One lattice point's random values (a texel centre: no filtering).
fn lattice(p: vec2<f32>) -> vec4<f32> {
    return textureSampleLevel(noise_tex, noise_smp, (floor(p) + 0.5) / NOISE, 0.0);
}

// The mip for noise at `per_m` lattice points a metre, where a pixel spans `fw` metres.
fn lod_at(fw: f32, per_m: f32) -> f32 {
    return max(log2(max(fw * per_m, 1e-6)), 0.0);
}

// Two octaves of value noise, -0.5..0.5 (channels r and g).
fn fbm2(p: vec2<f32>, lod: f32) -> f32 {
    return (vn(p, lod).r - 0.5) * 0.65 + (vn(p * 2.3 + 11.0, lod + 1.2).g - 0.5) * 0.35;
}

// The same from channels b and a, for a second, independent field at the same place.
fn fbm2b(p: vec2<f32>, lod: f32) -> f32 {
    return (vn(p, lod).b - 0.5) * 0.65 + (vn(p * 2.3 + 11.0, lod + 1.2).a - 0.5) * 0.35;
}

// The occluder map covers the diorama (terrain::occluder_map): r whether a crown stands within 100 m, g and b the
// canopy's underside and top, a its density.
const HALF: vec2<f32> = vec2<f32>(2304.0, 1792.0);
const PI: f32 = 3.14159265;

fn occ(q: vec2<f32>) -> vec4<f32> {
    return textureSampleLevel(occ_tex, occ_smp, (q + HALF) / (2.0 * HALF), 0.0);
}

// How much of the sun reaches this point past the tree belts: away from them (most of the field) one read says
// so; near them, walk the map towards the sun in twelve growing steps (out to about 90 m, a crown's shadow at the
// lowest sun) and take out what each crown in the way blocks. The walk starts at a different fraction of a step
// on every pixel, so the shade's edge comes out soft and grainy, as the feed shows it, rather than banded.
fn sun_open(p: vec3<f32>, to_sun: vec3<f32>, jitter: f32) -> f32 {
    let flat = length(to_sun.xz);
    if flat < 0.01 || occ(p.xz).r < 0.01 {
        return 1.0;
    }
    let dir = to_sun.xz / flat;
    let rise = to_sun.y / flat;
    var open = 1.0;
    var d = 1.0 + jitter * 3.0;
    for (var i = 0; i < 12; i = i + 1) {
        let o = occ(p.xz + dir * d);
        let ray = p.y + 0.4 + d * rise;
        let inside = smoothstep(o.g - 0.5, o.g + 0.5, ray) * (1.0 - smoothstep(o.b - 0.5, o.b + 0.5, ray));
        open *= 1.0 - o.a * inside;
        if open < 0.02 {
            break;
        }
        d += 3.0 + 0.15 * d;
    }
    return open;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    let w = in.world_position.xz;
    // How many metres a pixel spans here: the noise's mips and the detail's fades follow it.
    let fw = fwidth(w.x);
    // Sample the painted texel through a jitter of up to 0.35 m, so a texel's square edge wanders.
    var jittered = in;
    let near_tex = 1.0 - smoothstep(0.3, 1.2, fw);
    let lj = lod_at(fw, 0.9);
    let jitter = vec2<f32>(fbm2(w * 0.9, lj), fbm2b(w * 0.9, lj)) * 0.7 * near_tex / 256.0;
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
            let plant = ridge * step(0.35, lattice(vec2<f32>(across * 3.0, along / 0.7)).b);
            c = mix(c, vec3<f32>(0.15, 0.21, 0.07), (ridge * 0.22 + plant * 0.25) * fade * ground.params.x);
            c = mix(c, c * 0.9, 1.0 - fade);
        }
        let fg = fw;
        let near = 1.0 - smoothstep(0.1, 0.5, fg);
        let clump = (vn(w * 1.3, lod_at(fw, 1.3)).r - 0.5) * 0.18 + (vn(w * 0.31 + 7.0, lod_at(fw, 0.31)).g - 0.5) * 0.12;
        let blade = (lattice(w * 5.0).a - 0.5) * 0.09 * near;
        c = c * (1.0 + (clump * mix(0.5, 1.0, near) + blade) * ground.params.y);
    } else {
        // The snow's patches below the texel: where the texel says the cover is partial, cut the earth and the
        // snow apart along a ragged world-space edge, keeping the texel's mean. Full snow and full earth, and
        // the painted craters, scorch and tracks, pass through unchanged.
        let lum0 = dot(c, vec3<f32>(0.3, 0.59, 0.11));
        let cover = smoothstep(0.30, 0.72, lum0);
        let partial = cover * (1.0 - cover) * 4.0;
        let grain = fbm2(w * 0.55, lod_at(fw, 0.55)) + fbm2b(w * 2.1, lod_at(fw, 2.1)) * 0.5;
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
            let stub = ridge * step(0.6, lattice(vec2<f32>(across * 2.5, along / 0.75)).a);
            // Drifted snow hides the rows in patches; they show only where the wind has scoured it thin.
            let drift = smoothstep(0.4, 0.75, vn(w / 9.0, lod_at(fw, 1.0 / 9.0)).b * 0.5 + vn(w / 31.0 + 3.7, lod_at(fw, 1.0 / 31.0)).a * 0.5);
            let k = (ridge * mix(0.03, 0.28, thin) + stub * mix(0.04, 0.18, thin)) * drift * fade * ground.params.x;
            c = mix(c, vec3<f32>(0.07, 0.055, 0.045), k);
            // At distance, the rows average out into a slightly darker field.
            c = mix(c, c * (1.0 - 0.18 * mix(0.3, 1.0, thin)), 1.0 - fade);
        }
        // Wind-packed grain in the snow, and clods and crust at a quarter metre.
        let fg = fw;
        let close = 1.0 - smoothstep(0.1, 0.4, fg);
        let g = (lattice(w * 3.0).g - 0.5) * 0.05 * close * ground.params.y;
        let crust = (vn(w * 4.0, lod_at(fw, 4.0)).b - 0.5) * 0.06 * close;
        c = c * (1.0 + g + crust);
    }
    pbr_input.material.base_color = vec4<f32>(c, 1.0);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    // The belts' shade and the hills': the trees cast nothing into the shadow cascades, so the ground takes the sun's
    // direct light back out itself where the baked canopy or terrain stands in the way. Bevy's own terms (a rough
    // Lambert lobe, the light's colour and the exposure), times what the shadow map already let through, so a
    // vehicle's shadow inside a belt's is not darkened twice.
    if (ground.params.w < 0.5 || ground.params.w > 1.5) && lights.n_directional_lights > 0u {
        let to_sun = lights.directional_lights[0].direction_to_light.xyz;
        let ndl = max(dot(pbr_input.N, to_sun), 0.0);
        if ndl > 0.0 {
            let open = sun_open(in.world_position.xyz, to_sun, lattice(in.position.xy).a);
            if ground.params.w > 1.5 {
                out.color = vec4<f32>(vec3<f32>(open), 1.0);
                return out;
            }
            if open < 0.999 {
                var mapped = 1.0;
                if (lights.directional_lights[0].flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u {
                    let view_z = dot(vec4<f32>(view.view_from_world[0].z, view.view_from_world[1].z, view.view_from_world[2].z, view.view_from_world[3].z), in.world_position);
                    mapped = fetch_directional_shadow(0u, in.world_position, in.world_normal, view_z, in.position.xy);
                }
                // Bevy's diffuse lobe for the sun exactly (Burley's: at a low sun on rough ground it is about twice
                // plain Lambert), so the shade is as dark as a shadow the map would have drawn.
                let v = pbr_input.V;
                let h = normalize(to_sun + v);
                let ldoth = saturate(dot(to_sun, h));
                let ndv = max(dot(pbr_input.N, v), 0.0001);
                let rough = pbr_input.material.perceptual_roughness * pbr_input.material.perceptual_roughness;
                let f90 = 0.5 + 2.0 * rough * ldoth * ldoth;
                let burley = (1.0 + (f90 - 1.0) * pow(1.0 - ndl, 5.0)) * (1.0 + (f90 - 1.0) * pow(1.0 - ndv, 5.0)) / PI;
                let sun = c * burley * ndl * lights.directional_lights[0].color.rgb * view.exposure;
                out.color = vec4<f32>(max(out.color.rgb - sun * mapped * (1.0 - open), vec3<f32>(0.0)), out.color.a);
            }
        }
    }
    // The diorama has no edge: the last 600 m of ground dissolve into the haze.
    let edge = min(2304.0 - abs(w.x), 1792.0 - abs(w.y));
    let e = 1.0 - smoothstep(0.0, 600.0, edge);
    out.color = vec4<f32>(mix(out.color.rgb, fog.base_color.rgb, e * e), out.color.a);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}

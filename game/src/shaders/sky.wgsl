// The sky dome: a gradient from the horizon's haze to the zenith, a cloud deck projected onto a plane at its
// base height (its cover, its lit top and dark underside, its drift with the wind), and the sun, hidden by the
// deck or glowing through a veil.

#import bevy_pbr::{forward_io::VertexOutput, mesh_view_bindings::view}

struct SkyParams {
    zenith: vec4<f32>,
    horizon: vec4<f32>,
    sun_dir: vec4<f32>,
    sun_col: vec4<f32>,
    cloud: vec4<f32>,
    drift: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sky: SkyParams;

fn hash(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(127.1, 311.7));
    q += dot(q, q + 19.19);
    return fract(q.x * q.y);
}

fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x), mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    return vnoise(p) * 0.5 + vnoise(p * 2.1 + 5.0) * 0.27 + vnoise(p * 4.3 + 11.0) * 0.15 + vnoise(p * 8.9 + 23.0) * 0.08;
}

// The deck's density along a direction: 0 clear, 1 thick, from the noise field on the plane at its base.
fn deck(d: vec3<f32>) -> f32 {
    if d.y < 0.015 {
        return 0.0;
    }
    let t = sky.cloud.y / d.y;
    let p = (d.xz * t + sky.drift.xy) / 1100.0;
    let n = fbm(p);
    let cover = sky.cloud.x;
    return smoothstep(0.62 - cover * 0.5, 0.78 - cover * 0.5, n);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = normalize(in.world_position.xyz - view.world_position);
    let up = max(d.y, 0.0);
    var c = mix(sky.horizon.rgb, sky.zenith.rgb, pow(up, 0.55));
    let sun = normalize(sky.sun_dir.xyz);
    let veil = sky.cloud.w;
    // The deck, fading into the haze towards the horizon; thin where the noise is low, dark where it is thick.
    let dens = deck(d);
    let hfade = smoothstep(0.0, 0.1, d.y);
    let lit = sky.drift.z;
    // Its top catches the light on the sun's side; its underside is the horizon's grey, darker where thick.
    let toward_sun = 0.5 + 0.5 * dot(normalize(vec3<f32>(d.x, 0.0, d.z) + 0.001), normalize(vec3<f32>(sun.x, 0.0, sun.z) + 0.001));
    // A full deck is not flat: its underside lumps and sags, a little lighter where it is thinner.
    var lump = 1.0;
    if d.y > 0.015 {
        let p = (d.xz * (sky.cloud.y / d.y) + sky.drift.xy) / 700.0;
        lump = 0.93 + 0.14 * fbm(p + 3.0);
    }
    let under = sky.horizon.rgb * mix(1.0, sky.cloud.z, dens) * (0.92 + 0.16 * toward_sun * lit) * lump;
    let thin = sky.horizon.rgb * (1.04 + 0.1 * lit);
    let cloud_col = mix(thin, under, smoothstep(0.1, 0.8, dens));
    let opacity = dens * hfade * (1.0 - 0.55 * veil);
    c = mix(c, cloud_col, opacity);
    // The sun: a disc and its glow, hidden by the deck where it is thick, softened to a hazy disc by a veil.
    let sd = dot(d, sun);
    let disc = smoothstep(cos(sky.sun_dir.w * (1.0 + 2.0 * veil)), cos(sky.sun_dir.w * 0.5), sd);
    let glow = pow(max(sd, 0.0), 60.0) * 0.5 + pow(max(sd, 0.0), 10.0) * (0.12 + 0.25 * veil);
    let through = 1.0 - deck(sun) * (1.0 - 0.6 * veil);
    c += sky.sun_col.rgb * sky.sun_col.w * (disc * (1.0 - 0.6 * veil) + glow) * through * smoothstep(-0.02, 0.05, sun.y);
    return vec4<f32>(c, 1.0);
}

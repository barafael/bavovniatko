// The look of the war as most people saw it: a cheap drone camera's compressed, grainy, washed-out video.
// Mode 1 is the OP's binoculars instead: cleaner, colder, with a round field of view when zoomed.

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct FeedEffect {
    time: f32,
    // FeedMode: 0 drone, 1 binoculars, 2 eye, 3 sight.
    mode: u32,
    grain: f32,
    glitch: f32,
    flash: f32,
    zoom: f32,
    down: f32,
    saturation: f32,
}

@group(0) @binding(2) var<uniform> fx: FeedEffect;

fn hash(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(123.34, 456.21));
    q += dot(q, q + 45.32);
    return fract(q.x * q.y);
}

// Film grain needs a hash with no structure: the fract-based one above draws stripes in dark frames.
fn pcg(v: u32) -> u32 {
    let s = v * 747796405u + 2891336453u;
    let w = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
    return (w >> 22u) ^ w;
}

fn grain_at(p: vec2<f32>, frame: f32) -> f32 {
    let h = pcg(u32(p.x) + pcg(u32(p.y) + pcg(u32(frame))));
    return f32(h) / 4294967295.0;
}

fn luma(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(screen_texture));
    var uv = in.uv;
    let px = 1.0 / dims;
    let frame = floor(fx.time * 24.0);

    if fx.mode == 0u {
        // ---- Drone feed ----
        if fx.down > 0.5 {
            let n = hash(floor(uv * dims / 3.0) + frame);
            return vec4<f32>(vec3<f32>(0.05 + 0.06 * n), 1.0);
        }
        // Signal breakup: torn rows and blocky smears.
        let g = fx.glitch;
        if g > 0.0 {
            let row = floor(uv.y * 48.0);
            let h = hash(vec2<f32>(row, floor(fx.time * 9.0)));
            if h < g * 0.35 {
                uv.x += (hash(vec2<f32>(row, frame)) - 0.5) * 0.08 * g;
            }
            let block = floor(uv * dims / 24.0);
            if hash(block + floor(fx.time * 6.0)) < g * 0.12 {
                uv = (block + 0.5) * 24.0 / dims;
            }
        }
        // Compression: soften, then over-sharpen (the halo look of low-bitrate H.264).
        let c0 = textureSample(screen_texture, texture_sampler, uv).rgb;
        let blur = (textureSample(screen_texture, texture_sampler, uv + vec2<f32>(px.x, 0.0) * 1.5).rgb
            + textureSample(screen_texture, texture_sampler, uv - vec2<f32>(px.x, 0.0) * 1.5).rgb
            + textureSample(screen_texture, texture_sampler, uv + vec2<f32>(0.0, px.y) * 1.5).rgb
            + textureSample(screen_texture, texture_sampler, uv - vec2<f32>(0.0, px.y) * 1.5).rgb) * 0.25;
        var c = mix(blur, c0, 0.55);
        c = c + (c - blur) * 0.9;
        // Macroblock quantisation: flatten colour within 8 px blocks a little.
        let bq = (floor(uv * dims / 8.0) + 0.5) * 8.0 / dims;
        let cb = textureSample(screen_texture, texture_sampler, bq).rgb;
        c = mix(c, cb, 0.12);
        // Washed-out, desaturated small-sensor colour with clipped whites.
        let l = luma(c);
        // Washed-out small-sensor colour, hard contrast: snow clips, steel goes near black.
        c = mix(vec3<f32>(l), c, fx.saturation);
        c = (c - 0.5) * 1.18 + 0.52;
        c = clamp(c, vec3<f32>(0.03), vec3<f32>(0.96));
        c = c * vec3<f32>(0.97, 1.0, 1.02);
        // Chroma shift under jamming.
        if g > 0.0 {
            let r = textureSample(screen_texture, texture_sampler, uv + vec2<f32>(px.x * 4.0 * g, 0.0)).r;
            c.r = mix(c.r, r, g);
        }
        // Grain, re-rolled every frame, stronger in the shadows.
        let n = grain_at(floor(uv * dims), frame) - 0.5;
        c += n * (0.035 + 0.035 * (1.0 - l)) * fx.grain;
        // Lens vignette.
        let d = distance(uv, vec2<f32>(0.5));
        c *= 1.0 - smoothstep(0.45, 0.85, d) * 0.45;
        c += vec3<f32>(fx.flash) * 0.7;
        return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
    }

    // ---- Ground: the naked eye (2) and the ATGM sight (3) ----
    if fx.mode >= 2u {
        var c = textureSample(screen_texture, texture_sampler, uv).rgb;
        let d = distance(uv, vec2<f32>(0.5));
        if fx.mode == 3u {
            // Through the sight: a single round eyepiece, slightly cold, a little noise from the optics.
            let l = luma(c);
            c = mix(vec3<f32>(l), c, 0.7) * vec3<f32>(0.95, 1.0, 1.03);
            c = (c - 0.5) * 1.1 + 0.5;
            c += (hash(uv * dims + frame) - 0.5) * 0.03;
            let q = (uv - 0.5) * vec2<f32>(dims.x / dims.y, 1.0);
            c *= 1.0 - smoothstep(0.43, 0.47, length(q));
        } else {
            c += (hash(uv * dims + frame) - 0.5) * 0.02 * fx.grain;
            c *= 1.0 - smoothstep(0.55, 0.95, d) * 0.35;
        }
        c += vec3<f32>(fx.flash);
        return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
    }

    // ---- OP binoculars ----
    var c = textureSample(screen_texture, texture_sampler, uv).rgb;
    let l = luma(c);
    c = mix(vec3<f32>(l), c, 0.6) * vec3<f32>(0.96, 0.99, 1.04);
    c += (hash(uv * dims + frame) - 0.5) * 0.035 * fx.grain;
    // Round field of view once the binoculars come up.
    let aspect = dims.x / dims.y;
    let q = (uv - 0.5) * vec2<f32>(aspect, 1.0);
    let bino = smoothstep(1.1, 2.0, fx.zoom);
    let lens = min(length(q - vec2<f32>(0.22, 0.0)), length(q + vec2<f32>(0.22, 0.0)));
    let mask = 1.0 - smoothstep(0.40, 0.46, lens) * bino;
    c *= mask;
    let d = distance(uv, vec2<f32>(0.5));
    c *= 1.0 - smoothstep(0.5, 0.95, d) * 0.5;
    c += vec3<f32>(fx.flash);
    return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}

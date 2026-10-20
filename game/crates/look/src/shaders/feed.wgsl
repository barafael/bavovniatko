// The look of the war as most people saw it: a small drone camera's compressed, slightly soft, washed video,
// which the jammed link degrades from there.
// Mode 1 is the OP's binoculars instead: cleaner, colder, with a round field of view when zoomed.

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct FeedEffect {
    time: f32,
    // FeedMode: 0 drone, 1 binoculars, 2 eye, 3 sight, 4 raw.
    mode: u32,
    grain: f32,
    glitch: f32,
    flash: f32,
    zoom: f32,
    down: f32,
    saturation: f32,
    bitrate: f32,
    sight: f32,
}

@group(2) @binding(0) var<uniform> fx: FeedEffect;
@group(2) @binding(1) var screen_texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

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
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(screen_texture));
    var uv = in.uv;
    let px = 1.0 / dims;
    let frame = floor(fx.time * 24.0);

    if fx.mode == 4u {
        return textureSample(screen_texture, texture_sampler, uv);
    }
    if fx.mode == 6u {
        // ---- The drone's thermal channel, white hot (`bav_look::thermal`) ----
        // The world is near one temperature: the visible picture flattened to a narrow grey, a little lighter where
        // it was lit. What is hot carries a marker of a colour nothing else is (magenta), which reads as heat: white,
        // with a halo, as an uncooled sensor blooms. 640 x 512, with its fixed-pattern noise.
        let bq = (floor(uv * dims / 2.0) + 0.5) * 2.0 / dims;
        var heat = 0.0;
        for (var i = -2; i <= 2; i++) {
            for (var j = -2; j <= 2; j++) {
                let s = textureSample(screen_texture, texture_sampler, bq + vec2<f32>(f32(i), f32(j)) * px * 2.0).rgb;
                let key = clamp((min(s.r, s.b) - s.g) * 3.0, 0.0, 1.0);
                let w = 1.0 - (abs(f32(i)) + abs(f32(j))) * 0.18;
                heat = max(heat, key * w);
            }
        }
        let l = luma(textureSample(screen_texture, texture_sampler, bq).rgb);
        var t = 0.14 + 0.2 * l;
        t = mix(t, 1.0, heat);
        t += (hash(floor(uv * dims / 2.0)) - 0.5) * 0.012 + (hash(floor(uv * dims / 2.0) + frame) - 0.5) * 0.025;
        let d = distance(uv, vec2<f32>(0.5));
        t *= 1.0 - smoothstep(0.6, 1.0, d) * 0.3;
        return vec4<f32>(vec3<f32>(clamp(t + fx.flash, 0.0, 1.0)), 1.0);
    }
    if fx.mode == 0u {
        // ---- Drone feed ----
        if fx.down > 0.5 {
            let n = hash(floor(uv * dims / 3.0) + frame);
            return vec4<f32>(vec3<f32>(0.05 + 0.06 * n), 1.0);
        }
        // A digital link's ladder (design/next-items.md §2.6): as the bit rate falls the picture goes soft and
        // smeary, then into macroblocks with stale, greenish blocks where the decoder gave up; no snow, that is
        // an analog link.
        let soft = 1.0 - clamp(fx.bitrate, 0.0, 1.0);
        let g = fx.glitch;
        var stale = 0.0;
        if g > 0.0 {
            let row = floor(uv.y * 48.0);
            let h = hash(vec2<f32>(row, floor(fx.time * 9.0)));
            if h < g * 0.2 {
                uv.x += (hash(vec2<f32>(row, frame)) - 0.5) * 0.05 * g;
            }
            let bs = 16.0 + 24.0 * soft;
            let block = floor(uv * dims / bs);
            if hash(block + floor(fx.time * 4.0)) < g * 0.1 {
                uv = (block + 0.5) * bs / dims;
                stale = 1.0;
            }
        }
        // Compression: on a clean link a little soft with the camera's own light sharpening; as the rate falls,
        // softer, then over-sharpened (the halo look of low-bitrate H.264).
        let r = 1.0 + 3.5 * soft;
        let c0 = textureSample(screen_texture, texture_sampler, uv).rgb;
        let blur = (textureSample(screen_texture, texture_sampler, uv + vec2<f32>(px.x, 0.0) * r).rgb
            + textureSample(screen_texture, texture_sampler, uv - vec2<f32>(px.x, 0.0) * r).rgb
            + textureSample(screen_texture, texture_sampler, uv + vec2<f32>(0.0, px.y) * r).rgb
            + textureSample(screen_texture, texture_sampler, uv - vec2<f32>(0.0, px.y) * r).rgb) * 0.25;
        var c = mix(blur, c0, 0.8 - 0.65 * soft);
        c = c + (c - blur) * (0.3 + 0.1 * soft);
        // Macroblock quantisation: flatten colour within blocks, 8 px and barely there clean, up to 24 px starved.
        let mb = 8.0 + 16.0 * soft;
        let bq = (floor(uv * dims / mb) + 0.5) * mb / dims;
        let cb = textureSample(screen_texture, texture_sampler, bq).rgb;
        c = mix(c, cb, 0.04 + 0.68 * soft);
        c = mix(c, vec3<f32>(0.38, 0.5, 0.4) * (0.5 + luma(c)), stale * 0.35);
        // A small sensor's colour: a little washed, the shadows lifted rather than crushed (the camera's auto
        // exposure and tone curve keep a dark hull readable against the field), the whites rolling off before
        // they clip. A starved link loses the shadows again.
        let l = luma(c);
        c = mix(vec3<f32>(l), c, fx.saturation);
        c = max(c, vec3<f32>(0.0));
        c = pow(c, vec3<f32>(0.9 + 0.1 * soft));
        c = (c - 0.5) * (1.07 + 0.1 * soft) + 0.5;
        c = 0.025 + c * 0.95;
        c = min(c, vec3<f32>(0.97));
        c = c * vec3<f32>(0.98, 1.0, 1.015);
        // Chroma shift under jamming.
        if g > 0.0 {
            let r = textureSample(screen_texture, texture_sampler, uv + vec2<f32>(px.x * 4.0 * g, 0.0)).r;
            c.r = mix(c.r, r, g);
        }
        // Sensor noise, re-rolled every frame, stronger in the shadows and as the link starves.
        let n = grain_at(floor(uv * dims), frame) - 0.5;
        c += n * (0.012 + 0.018 * (1.0 - l) + 0.04 * soft) * fx.grain;
        // Lens vignette: a zoom camera's is faint.
        let d = distance(uv, vec2<f32>(0.5));
        c *= 1.0 - smoothstep(0.55, 0.9, d) * 0.18;
        c += vec3<f32>(fx.flash) * 0.7;
        return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
    }

    if fx.mode == 5u {
        // ---- An FPV's analogue goggles: wide, warm, soft, and snow when the signal thins ----
        let g = clamp(fx.glitch, 0.0, 1.0);
        if fx.down > 0.5 {
            let n = hash(floor(uv * dims / 2.0) + vec2<f32>(frame * 1.37, frame * 2.11));
            return vec4<f32>(vec3<f32>(0.25 + 0.5 * n), 1.0);
        }
        // Lost sync: lines slip sideways, and now and then the whole frame rolls.
        let line = floor(uv.y * dims.y / 2.0);
        if hash(vec2<f32>(line, frame)) < g * g * 0.5 {
            uv.x += (hash(vec2<f32>(line + 7.0, frame)) - 0.5) * 0.08 * g;
        }
        if hash(vec2<f32>(frame, 3.0)) < max(g - 0.6, 0.0) * 0.4 {
            uv.y = fract(uv.y + hash(vec2<f32>(frame, 5.0)) * 0.3);
        }
        // A wide lens: a little barrel.
        let cuv = uv - 0.5;
        let suv = 0.5 + cuv * (1.0 - 0.08 * dot(cuv, cuv));
        var c = textureSample(screen_texture, texture_sampler, suv).rgb;
        let b = textureSample(screen_texture, texture_sampler, suv + vec2<f32>(px.x * 2.0, 0.0)).rgb;
        c = mix(c, b, 0.4);
        // Small-camera colour: warm, saturated, contrasty.
        c = (c - 0.5) * 1.15 + 0.5;
        c = c * vec3<f32>(1.06, 1.0, 0.9);
        // Snow: coloured noise, growing as the signal fails; a dropout now and then.
        let q = floor(uv * dims) + vec2<f32>(frame * 1.37, frame * 2.11);
        let n = vec3<f32>(hash(q), hash(q + 17.0), hash(q + 31.0));
        c = mix(c, n, 0.03 + 0.77 * g * g);
        if hash(vec2<f32>(frame, 11.0)) < max(g - 0.75, 0.0) {
            c = n * 0.8;
        }
        // Scanlines, and a vignette.
        c *= 0.92 + 0.08 * sin(uv.y * dims.y * 1.6);
        let d = distance(uv, vec2<f32>(0.5));
        c *= 1.0 - smoothstep(0.4, 0.8, d) * 0.5;
        c += vec3<f32>(fx.flash) * 0.7;
        return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
    }

    // ---- Ground: the naked eye (2) and the ATGM sight (3) ----
    if fx.mode >= 2u {
        var c = textureSample(screen_texture, texture_sampler, uv).rgb;
        let d = distance(uv, vec2<f32>(0.5));
        if fx.mode == 3u {
            // Neither sight is an eyepiece (design/next-items.md §2.6): the Stugna-P's gunner watches the
            // panel's low-resolution 4:3 TV picture; the Javelin's CLU shows a black-and-green thermal. Both
            // sit in a 4:3 frame with black outside it.
            let aspect = dims.x / dims.y;
            let fx43 = (uv.x - 0.5) * aspect / (4.0 / 3.0);
            if abs(fx43) > 0.5 {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
            if fx.sight > 0.5 {
                // The CLU: the scene by its heat, hot light green; our picture has no heat, so dark steel and
                // bodies (the dark things) read hot and the snow cold; coarse, with the detector's noise.
                let bq = (floor(uv * dims / 3.0) + 0.5) * 3.0 / dims;
                let l = pow(luma(textureSample(screen_texture, texture_sampler, bq).rgb), 0.45);
                let heat = clamp(1.15 - l * 1.3, 0.0, 1.0);
                c = vec3<f32>(0.10, 0.95, 0.28) * (0.08 + 0.92 * heat * heat);
                c += (hash(floor(uv * dims / 3.0) + frame) - 0.5) * 0.06;
            } else {
                // The panel: a PAL-class picture, soft, nearly grey, a little noise.
                let bq = (floor(uv * dims / 2.0) + 0.5) * 2.0 / dims;
                c = textureSample(screen_texture, texture_sampler, bq).rgb;
                let l = luma(c);
                c = mix(vec3<f32>(l), c, 0.25) * vec3<f32>(0.97, 1.0, 1.02);
                c = (c - 0.5) * 1.05 + 0.5;
                c += (hash(uv * dims + frame) - 0.5) * 0.04;
            }
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
    c = mix(vec3<f32>(l), c, 0.8) * vec3<f32>(0.97, 0.99, 1.03);
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

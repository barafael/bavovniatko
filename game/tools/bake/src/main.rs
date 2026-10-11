//! Bakes the downloaded vehicle models (`research/assets/files/`, see `research/assets/README.md`) into the
//! game's `.veh` files in `game/assets/vehicles/`.
//!
//! Every model arrives in its own units, axes and pieces. The bake puts it in the game's frame (metres, +Y up,
//! the front towards -Z, the hull centred on the origin with its tracks on y = 0), splits it into the hull and
//! the part that turns (a turret, or a helicopter's rotors), merges the pieces by material, simplifies the
//! meshes to a budget, and shrinks the textures to a mip chain. The game reads the result in one pass, with no
//! glTF loader and no asset server.
//!
//! ```sh
//! cd game/tools/bake && cargo run --release            # all models
//! cargo run --release -- t72 bmp2                      # some
//! ```

mod spec;

use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use gltf::mesh::Mode;
use image::{DynamicImage, RgbaImage, imageops::FilterType};
use spec::{Pick, Spec};

type V3 = [f32; 3];

/// One vertex as the game reads it: position, normal, uv.
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Vert {
    p: V3,
    n: V3,
    uv: [f32; 2],
}

/// Geometry gathered from the source, per (role, source material).
#[derive(Default)]
struct Bucket {
    verts: Vec<Vert>,
    idx: Vec<u32>,
}

const HULL: u8 = 0;
const TURRET: u8 = 1;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let src = root.join("research/assets/files");
    let out = root.join("game/assets/vehicles");
    std::fs::create_dir_all(&out).unwrap();
    let only: Vec<String> = std::env::args().skip(1).collect();
    let mut credits = Vec::new();
    for s in spec::SPECS {
        if !only.is_empty() && !only.iter().any(|o| o == s.id) {
            continue;
        }
        let dir = src.join(s.dir);
        if !dir.join("scene.gltf").exists() {
            eprintln!("{}: {} is missing (run research/assets/fetch.py)", s.id, s.dir);
            continue;
        }
        bake(s, &dir, &out.join(format!("{}.veh", s.id)));
        credits.push((s.id, attribution(&dir)));
    }
    if only.is_empty() {
        let mut f = std::fs::File::create(out.join("CREDITS.md")).unwrap();
        writeln!(f, "# Vehicle models\n\nBaked by `game/tools/bake` from the models in `research/assets/` (see its README for\nthe provenance checks). Each is used under its licence; the bake rescales, splits, simplifies and\nre-encodes it, and changes nothing else.\n").unwrap();
        for (id, a) in credits {
            writeln!(f, "- `{id}.veh`: {a}").unwrap();
        }
    }
}

fn attribution(dir: &Path) -> String {
    let meta = std::fs::read_to_string(dir.join("meta.yaml")).unwrap_or_default();
    meta.lines().find_map(|l| l.strip_prefix("attribution: ")).map(|a| a.trim_matches('"').to_string()).unwrap_or_else(|| "(no attribution in meta.yaml)".into())
}

fn mat4_mul(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // Column-major, as glTF stores it: m[col][row].
    let mut r = [[0.0; 4]; 4];
    for c in 0..4 {
        for row in 0..4 {
            r[c][row] = (0..4).map(|k| a[k][row] * b[c][k]).sum();
        }
    }
    r
}

fn xf_point(m: &[[f32; 4]; 4], p: V3) -> V3 {
    let mut r = [0.0; 3];
    for (row, out) in r.iter_mut().enumerate() {
        *out = m[0][row] * p[0] + m[1][row] * p[1] + m[2][row] * p[2] + m[3][row];
    }
    r
}

/// The normal matrix (inverse transpose of the upper 3x3), and whether the transform mirrors.
fn normal_matrix(m: &[[f32; 4]; 4]) -> ([[f32; 3]; 3], bool) {
    let a = |r: usize, c: usize| m[c][r];
    let det = a(0, 0) * (a(1, 1) * a(2, 2) - a(1, 2) * a(2, 1)) - a(0, 1) * (a(1, 0) * a(2, 2) - a(1, 2) * a(2, 0)) + a(0, 2) * (a(1, 0) * a(2, 1) - a(1, 1) * a(2, 0));
    // Cofactor matrix = det * inverse transpose; the scale does not matter, we renormalise.
    let mut n = [[0.0; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            let (r1, r2) = ((r + 1) % 3, (r + 2) % 3);
            let (c1, c2) = ((c + 1) % 3, (c + 2) % 3);
            n[r][c] = a(r1, c1) * a(r2, c2) - a(r1, c2) * a(r2, c1);
        }
    }
    let s = det.signum();
    for row in &mut n {
        for v in row.iter_mut() {
            *v *= s;
        }
    }
    (n, det < 0.0)
}

fn xf_normal(n: &[[f32; 3]; 3], v: V3) -> V3 {
    norm([n[0][0] * v[0] + n[0][1] * v[1] + n[0][2] * v[2], n[1][0] * v[0] + n[1][1] * v[1] + n[1][2] * v[2], n[2][0] * v[0] + n[2][1] * v[1] + n[2][2] * v[2]])
}

fn norm(v: V3) -> V3 {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 1e-12 { [v[0] / l, v[1] / l, v[2] / l] } else { [0.0, 1.0, 0.0] }
}

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

struct Bounds {
    lo: V3,
    hi: V3,
}

impl Bounds {
    fn new() -> Self {
        Bounds { lo: [f32::MAX; 3], hi: [f32::MIN; 3] }
    }
    fn add(&mut self, p: V3) {
        for k in 0..3 {
            self.lo[k] = self.lo[k].min(p[k]);
            self.hi[k] = self.hi[k].max(p[k]);
        }
    }
    fn fmt(&self) -> String {
        format!("x {:.2}..{:.2}  y {:.2}..{:.2}  z {:.2}..{:.2}", self.lo[0], self.hi[0], self.lo[1], self.hi[1], self.lo[2], self.hi[2])
    }
}

fn bake(s: &Spec, dir: &Path, out: &Path) {
    let (doc, buffers, images) = gltf::import(dir.join("scene.gltf")).unwrap_or_else(|e| panic!("{}: {e}", s.id));
    println!("== {} ({})", s.id, s.dir);

    // The spec's frame: the source rotated so the front is -Z, and scaled to metres. Picks are written in it.
    let (sy, cy) = s.yaw.to_radians().sin_cos();
    let up: [[f32; 4]; 4] = if s.z_up { [[1.0, 0.0, 0.0, 0.0], [0.0, 0.0, -1.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]] } else { IDENT };
    let yaw: [[f32; 4]; 4] = [[cy, 0.0, -sy, 0.0], [0.0, 1.0, 0.0, 0.0], [sy, 0.0, cy, 0.0], [0.0, 0.0, 0.0, 1.0]];
    let u = s.units;
    let scale: [[f32; 4]; 4] = [[u, 0.0, 0.0, 0.0], [0.0, u, 0.0, 0.0], [0.0, 0.0, u, 0.0], [0.0, 0.0, 0.0, 1.0]];
    let base = mat4_mul(&scale, &mat4_mul(&yaw, &up));

    let mut buckets: HashMap<(u8, usize), Bucket> = HashMap::new();
    let scene = doc.default_scene().unwrap_or_else(|| doc.scenes().next().unwrap());
    let mut stack: Vec<(gltf::Node, [[f32; 4]; 4], Option<u8>)> = scene.nodes().map(|n| (n, base, None)).collect();
    while let Some((node, parent, inherited)) = stack.pop() {
        let world = mat4_mul(&parent, &node.transform().matrix());
        let i = node.index();
        if s.drop.contains(&i) {
            continue;
        }
        // A pick by node holds for the node's children too.
        let forced = if s.turret_nodes.contains(&i) { Some(TURRET) } else if s.hull_nodes.contains(&i) { Some(HULL) } else { inherited };
        for c in node.children() {
            stack.push((c, world, forced));
        }
        let Some(mesh) = node.mesh() else { continue };
        let (nm, mirrored) = normal_matrix(&world);
        let mut nb = Bounds::new();
        for prim in mesh.primitives() {
            if prim.mode() != Mode::Triangles {
                continue;
            }
            let r = prim.reader(|b| Some(&buffers[b.index()]));
            let Some(pos) = r.read_positions() else { continue };
            let pos: Vec<V3> = pos.map(|p| xf_point(&world, p)).collect();
            let nrm: Option<Vec<V3>> = r.read_normals().map(|n| n.map(|v| xf_normal(&nm, v)).collect());
            let uv: Vec<[f32; 2]> = r.read_tex_coords(0).map(|t| t.into_f32().collect()).unwrap_or_else(|| vec![[0.0; 2]; pos.len()]);
            let mut idx: Vec<u32> = r.read_indices().map(|i| i.into_u32().collect()).unwrap_or_else(|| (0..pos.len() as u32).collect());
            if mirrored {
                for t in idx.chunks_mut(3) {
                    t.swap(1, 2);
                }
            }
            for p in &pos {
                nb.add(*p);
            }
            let nrm = nrm.unwrap_or_else(|| smooth_normals(&pos, &idx));
            let mat = prim.material().index().unwrap_or(usize::MAX);
            // Split by connected piece: a hatch, a road wheel, the turret shell each go one way whole.
            for (role, tris) in split(s, forced, &pos, &idx) {
                let b = buckets.entry((role, mat)).or_default();
                let mut remap: HashMap<u32, u32> = HashMap::new();
                for t in tris {
                    for k in 0..3 {
                        let v = idx[t * 3 + k];
                        let n = *remap.entry(v).or_insert_with(|| {
                            b.verts.push(Vert { p: pos[v as usize], n: nrm[v as usize], uv: uv[v as usize] });
                            (b.verts.len() - 1) as u32
                        });
                        b.idx.push(n);
                    }
                }
            }
        }
        if s.verbose {
            println!("  node {i:3} {:30} {}", node.name().unwrap_or("").chars().take(30).collect::<String>(), nb.fmt());
        }
    }
    if s.turret_none {
        buckets.retain(|(role, _), _| *role == HULL);
    }

    // Normalise: centre the hull, tracks on the ground, and scale to the real hull length.
    let mut hb = Bounds::new();
    let mut tb = Bounds::new();
    for ((role, _), b) in &buckets {
        for v in &b.verts {
            if *role == HULL { hb.add(v.p) } else { tb.add(v.p) }
        }
    }
    let k = s.length.map(|l| l / (hb.hi[2] - hb.lo[2])).unwrap_or(1.0);
    let off = [-(hb.lo[0] + hb.hi[0]) * 0.5, -hb.lo[1], -(hb.lo[2] + hb.hi[2]) * 0.5];
    let place = |p: V3| [(p[0] + off[0]) * k, (p[1] + off[1]) * k, (p[2] + off[2]) * k];
    // The turret ring: the spec's, or the middle of the turret's footprint; at the turret's lowest point.
    let ring = match s.pick {
        Pick::Ring { x, z, .. } => [x, tb.lo[1], z],
        Pick::Nodes => {
            let (c, _) = area_centroid(buckets.iter().filter(|((r, _), _)| *r == TURRET).map(|(_, b)| b));
            [c[0], tb.lo[1], c[2]]
        }
    };
    let turret_at = if tb.lo[1] < f32::MAX { place(ring) } else { [0.0, (hb.hi[1] + off[1]) * k, 0.0] };
    for ((role, _), b) in buckets.iter_mut() {
        for v in &mut b.verts {
            v.p = place(v.p);
            if *role == TURRET {
                v.p = sub(v.p, turret_at);
            }
        }
    }
    // The muzzle: the turret's most forward point.
    let muzzle = buckets.iter().filter(|((r, _), _)| *r == TURRET).flat_map(|(_, b)| b.verts.iter()).map(|v| v.p).min_by(|a, b| a[2].total_cmp(&b[2])).unwrap_or([0.0, 0.4, -3.0]);
    let hull_lo = place(hb.lo);
    let hull_hi = place(hb.hi);

    // Simplify to the budget, sharing it out by triangle count.
    let total: usize = buckets.values().map(|b| b.idx.len() / 3).sum();
    let ratio = (s.max_tris as f32 / total as f32).min(1.0);
    let mut kept = 0;
    for b in buckets.values_mut() {
        if ratio < 1.0 && b.idx.len() > 3 * 64 {
            simplify(b, ratio);
        }
        kept += b.idx.len() / 3;
    }
    println!("  hull {}  turret at ({:.2}, {:.2}, {:.2})  muzzle ({:.2}, {:.2}, {:.2})", Bounds { lo: hull_lo, hi: hull_hi }.fmt(), turret_at[0], turret_at[1], turret_at[2], muzzle[0], muzzle[1], muzzle[2]);
    println!("  {total} triangles -> {kept}");

    // Materials, in the order the parts use them.
    let mut mat_ids: Vec<usize> = buckets.keys().map(|(_, m)| *m).collect();
    mat_ids.sort();
    mat_ids.dedup();
    let mut f = Vec::new();
    f.extend_from_slice(b"VEH1");
    for v in [turret_at, muzzle, hull_lo, hull_hi] {
        put_v3(&mut f, v);
    }
    put_u32(&mut f, mat_ids.len() as u32);
    let mut tex_bytes = 0;
    for &m in &mat_ids {
        tex_bytes += write_material(&mut f, s, &doc, &images, m);
    }
    let mut parts: Vec<_> = buckets.into_iter().filter(|(_, b)| !b.idx.is_empty()).collect();
    parts.sort_by_key(|(k, _)| *k);
    put_u32(&mut f, parts.len() as u32);
    for ((role, m), b) in &parts {
        f.push(*role);
        put_u32(&mut f, mat_ids.iter().position(|x| x == m).unwrap() as u32);
        put_u32(&mut f, b.verts.len() as u32);
        for v in &b.verts {
            put_v3(&mut f, v.p);
            put_v3(&mut f, v.n);
            put_f32(&mut f, v.uv[0]);
            put_f32(&mut f, v.uv[1]);
        }
        put_u32(&mut f, b.idx.len() as u32);
        for i in &b.idx {
            put_u32(&mut f, *i);
        }
    }
    std::fs::write(out, &f).unwrap();
    println!("  {} materials, {} parts, {:.1} MB ({:.1} MB textures)", mat_ids.len(), parts.len(), f.len() as f32 / 1e6, tex_bytes as f32 / 1e6);
}

const IDENT: [[f32; 4]; 4] = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];

/// Which triangles go to the hull and which to the turret, decided per connected piece.
fn split(s: &Spec, forced: Option<u8>, pos: &[V3], idx: &[u32]) -> Vec<(u8, Vec<usize>)> {
    let ntri = idx.len() / 3;
    if let Some(r) = forced {
        return vec![(r, (0..ntri).collect())];
    }
    let Pick::Ring { x, z, y, r, gun } = s.pick else {
        return vec![(HULL, (0..ntri).collect())];
    };
    // Union-find over shared vertices, and over vertices at the same position (pieces split at UV seams).
    let mut parent: Vec<u32> = (0..pos.len() as u32).collect();
    fn find(p: &mut [u32], mut a: u32) -> u32 {
        while p[a as usize] != a {
            p[a as usize] = p[p[a as usize] as usize];
            a = p[a as usize];
        }
        a
    }
    let mut at: HashMap<[i32; 3], u32> = HashMap::new();
    for (i, p) in pos.iter().enumerate() {
        let key = [(p[0] * 1e4) as i32, (p[1] * 1e4) as i32, (p[2] * 1e4) as i32];
        let o = *at.entry(key).or_insert(i as u32);
        let (a, b) = (find(&mut parent, o), find(&mut parent, i as u32));
        parent[a as usize] = b;
    }
    for t in idx.chunks(3) {
        for k in 1..3 {
            let (a, b) = (find(&mut parent, t[0]), find(&mut parent, t[k]));
            parent[a as usize] = b;
        }
    }
    let mut pieces: HashMap<u32, (Vec<usize>, Bounds)> = HashMap::new();
    for t in 0..ntri {
        let root = find(&mut parent, idx[t * 3]);
        let e = pieces.entry(root).or_insert_with(|| (Vec::new(), Bounds::new()));
        e.0.push(t);
        for k in 0..3 {
            e.1.add(pos[idx[t * 3 + k] as usize]);
        }
    }
    let mut turret = Vec::new();
    let mut hull = Vec::new();
    // BAKE_PIECES=<y>: list the pieces reaching above y, to write a ring from.
    let show: Option<f32> = std::env::var("BAKE_PIECES").ok().and_then(|v| v.parse().ok());
    for (_, (tris, b)) in pieces {
        if show.is_some_and(|y0| b.hi[1] > y0) {
            println!("    piece {:5} tris  {}", tris.len(), b.fmt());
        }
        let c = [(b.lo[0] + b.hi[0]) * 0.5, (b.lo[1] + b.hi[1]) * 0.5, (b.lo[2] + b.hi[2]) * 0.5];
        let above = b.lo[1] > y - 0.05;
        let in_ring = ((c[0] - x).powi(2) + (c[2] - z).powi(2)).sqrt() < r;
        // The gun and its mantlet: a narrow piece reaching forward of the ring.
        let gun_piece = gun > 0.0 && (c[0] - x).abs() < gun && b.lo[2] < z - r * 0.5 && b.hi[0] - b.lo[0] < gun * 2.0;
        if above && (in_ring || gun_piece) { turret.extend(tris) } else { hull.extend(tris) }
    }
    vec![(HULL, hull), (TURRET, turret)]
}

fn area_centroid<'a>(bs: impl Iterator<Item = &'a Bucket>) -> (V3, f32) {
    let mut acc = [0.0f64; 3];
    let mut area = 0.0f64;
    for b in bs {
        for t in b.idx.chunks(3) {
            let (p0, p1, p2) = (b.verts[t[0] as usize].p, b.verts[t[1] as usize].p, b.verts[t[2] as usize].p);
            let c = cross(sub(p1, p0), sub(p2, p0));
            let a = 0.5 * ((c[0] * c[0] + c[1] * c[1] + c[2] * c[2]) as f64).sqrt();
            for k in 0..3 {
                acc[k] += a * (p0[k] + p1[k] + p2[k]) as f64 / 3.0;
            }
            area += a;
        }
    }
    if area == 0.0 {
        return ([0.0; 3], 0.0);
    }
    ([(acc[0] / area) as f32, (acc[1] / area) as f32, (acc[2] / area) as f32], area as f32)
}

fn smooth_normals(pos: &[V3], idx: &[u32]) -> Vec<V3> {
    let mut n = vec![[0.0f32; 3]; pos.len()];
    for t in idx.chunks(3) {
        let c = cross(sub(pos[t[1] as usize], pos[t[0] as usize]), sub(pos[t[2] as usize], pos[t[0] as usize]));
        for &v in t {
            for k in 0..3 {
                n[v as usize][k] += c[k];
            }
        }
    }
    n.into_iter().map(norm).collect()
}

fn simplify(b: &mut Bucket, ratio: f32) {
    let target = ((b.idx.len() as f32 * ratio) as usize / 3 * 3).max(3 * 32);
    let bytes = meshopt::typed_to_bytes(&b.verts);
    let adapter = meshopt::VertexDataAdapter::new(bytes, std::mem::size_of::<Vert>(), 0).unwrap();
    // UVs weigh in, so seams and painted detail hold up longer than flat panels.
    let attrs: Vec<f32> = b.verts.iter().flat_map(|v| [v.uv[0], v.uv[1], v.n[0], v.n[1], v.n[2]]).collect();
    let idx = meshopt::simplify_with_attributes_and_locks(&b.idx, &adapter, &attrs, &[0.5, 0.5, 0.3, 0.3, 0.3], 5 * 4, &vec![false; b.verts.len()], target, 0.02, meshopt::SimplifyOptions::None, None);
    let mut remap = vec![u32::MAX; b.verts.len()];
    let mut verts = Vec::new();
    b.idx = idx
        .into_iter()
        .map(|i| {
            if remap[i as usize] == u32::MAX {
                remap[i as usize] = verts.len() as u32;
                verts.push(b.verts[i as usize]);
            }
            remap[i as usize]
        })
        .collect();
    b.verts = verts;
}

/// One material: colour, alpha, two-sidedness, and its base-colour texture as a mip chain of encoded levels.
fn write_material(f: &mut Vec<u8>, s: &Spec, doc: &gltf::Document, images: &[gltf::image::Data], m: usize) -> usize {
    let (color, alpha, cutoff, double, tex) = if m == usize::MAX {
        ([0.5, 0.5, 0.5, 1.0], 0u8, 0.5, false, None)
    } else {
        let mat = doc.materials().nth(m).unwrap();
        let pbr = mat.pbr_metallic_roughness();
        let mut color = pbr.base_color_factor();
        let mut tex = pbr.base_color_texture().map(|t| t.texture().source().index());
        if let Some(sg) = mat.pbr_specular_glossiness() {
            color = sg.diffuse_factor();
            tex = tex.or(sg.diffuse_texture().map(|t| t.texture().source().index()));
        }
        let alpha = match mat.alpha_mode() {
            gltf::material::AlphaMode::Opaque => 0,
            gltf::material::AlphaMode::Mask => 1,
            gltf::material::AlphaMode::Blend => 2,
        };
        (color, alpha, mat.alpha_cutoff().unwrap_or(0.5), mat.double_sided(), tex)
    };
    let color = s.tint.map(|t| [color[0] * t[0], color[1] * t[1], color[2] * t[2], color[3]]).unwrap_or(color);
    for c in color {
        put_f32(f, c);
    }
    f.push(alpha);
    put_f32(f, cutoff);
    f.push(double as u8);
    let Some(img) = tex.map(|i| to_rgba(&images[i])) else {
        put_u32(f, 0);
        return 0;
    };
    let mut img = DynamicImage::ImageRgba8(img);
    let side = img.width().max(img.height());
    if side > s.tex {
        let k = s.tex as f32 / side as f32;
        img = img.resize_exact(((img.width() as f32 * k) as u32).max(1), ((img.height() as f32 * k) as u32).max(1), FilterType::Lanczos3);
    }
    let mut levels = Vec::new();
    loop {
        levels.push(img.clone());
        if img.width() == 1 && img.height() == 1 {
            break;
        }
        img = img.resize_exact((img.width() / 2).max(1), (img.height() / 2).max(1), FilterType::Triangle);
    }
    put_u32(f, levels.len() as u32);
    let mut bytes = 0;
    for l in levels {
        let mut enc = Vec::new();
        if alpha == 0 {
            // Opaque: JPEG, a tenth of the size.
            let rgb = l.to_rgb8();
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut enc, 88).encode_image(&rgb).unwrap();
            f.push(1);
        } else {
            l.write_to(&mut std::io::Cursor::new(&mut enc), image::ImageFormat::Png).unwrap();
            f.push(0);
        }
        put_u32(f, enc.len() as u32);
        f.extend_from_slice(&enc);
        bytes += enc.len();
    }
    bytes
}

fn to_rgba(d: &gltf::image::Data) -> RgbaImage {
    use gltf::image::Format::*;
    let (w, h) = (d.width, d.height);
    let px: Vec<u8> = match d.format {
        R8G8B8A8 => d.pixels.clone(),
        R8G8B8 => d.pixels.chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        R8 => d.pixels.iter().flat_map(|&c| [c, c, c, 255]).collect(),
        R8G8 => d.pixels.chunks(2).flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        R16G16B16A16 => d.pixels.chunks(2).map(|c| c[1]).collect(),
        R16G16B16 => d.pixels.chunks(6).flat_map(|c| [c[1], c[3], c[5], 255]).collect(),
        other => panic!("image format {other:?}"),
    };
    RgbaImage::from_raw(w, h, px).unwrap()
}

fn put_u32(f: &mut Vec<u8>, v: u32) {
    f.extend_from_slice(&v.to_le_bytes());
}

fn put_f32(f: &mut Vec<u8>, v: f32) {
    f.extend_from_slice(&v.to_le_bytes());
}

fn put_v3(f: &mut Vec<u8>, v: V3) {
    for c in v {
        put_f32(f, c);
    }
}

//! A diorama as data: one RON file per diorama in `game/dioramas/`, read at startup. It holds what the place is
//! (where on the map, the date and the clock, who attacks, the season), the shape of its ground (a sum of height
//! terms), and its layout: named curves and lines, the roads, shelterbelts, approach routes, minefields, the
//! positions and pre-registered points, the old wrecks, and the lines that matter to the fight.
//!
//! The files are built into the binary; `BAV_DATA=<dir>` reads `<dir>/<name>.ron` instead, so a layout can be
//! changed without a rebuild. A file that does not parse stops the game with the file, line and column.
//!
//! The builders below evaluate every form in the order and with the operations the hand-written layouts used,
//! so a diorama moved into a file plays the same day, to the bit.

use crate::diorama::{Diorama, Side, diorama};
use crate::geo::{Barrier, Belt, Layout, Polyline, Rect, Road, RoadKind, Route, ShoulderField, cut_gaps, fbm, smoothstep};
use bevy::math::Vec2;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// A point on the map: (x, z) in metres, +x east, +z south.
pub type P = (f32, f32);

fn v((x, z): P) -> Vec2 {
    Vec2::new(x, z)
}

#[derive(Deserialize, Debug)]
pub struct DioramaData {
    /// The place, in English and Ukrainian, as the title and the HUD name it.
    pub place: (String, String),
    /// The date as the title gives it ("6 February 2023") and as the tally shows it ("06 FEB 2023").
    pub date: String,
    pub tally_date: String,
    /// Clock time (minutes after midnight) when the day starts, and game minutes per real second.
    pub day_start_min: f32,
    pub compression: f32,
    pub attacker: Side,
    pub summer: bool,
    /// The net's callsigns, in English and Ukrainian (`crate::text::fill`).
    pub callsigns: Callsigns,
    /// The map's anchor: the latitude and longitude at (x, z), and metres per degree of longitude there.
    pub anchor: Anchor,
    /// The sun's day: the day of the year, and local time's offset from UTC (hours). None: the place keeps its own
    /// hand-made curve.
    #[serde(default)]
    pub sun: Option<(u32, f32)>,
    /// Whether the player's drone has a thermal channel here (a Mavic 2 Enterprise Advanced, key I): the hot things
    /// then carry their heat (`bav_look::thermal`).
    #[serde(default)]
    pub thermal: bool,
    /// What the day says that is this place's own.
    pub words: Words,
    /// Where the drone starts.
    pub camera: P,
    /// Where the observers are and their eye's height above the ground, where the set does not decide it.
    #[serde(default)]
    pub op: Option<(P, f32)>,
    /// What the observers' fire is adjusted from: their error grows with the distance from it.
    pub observed_from: ObservedFrom,
    /// Whether the ground is worn by weeks of war already: old craters along the belts and scattered over the
    /// fields (not on a war's first morning).
    #[serde(default = "yes")]
    pub worn: bool,
    /// A band of old shelling pocking no-man's-land: how many craters, from this z, over that depth.
    #[serde(default)]
    pub shelled: Option<(u32, f32, f32)>,
    /// The ground's height above sea level: these terms, summed in order.
    pub ground: Vec<HeightTerm>,
    /// Named curves z(x), for lines sampled along them, valleys, and the set.
    pub curves: Vec<(String, Curve)>,
    /// Named lines: roads, routes, the river.
    pub lines: Vec<(String, LineSpec)>,
    pub roads: Vec<RoadSpec>,
    /// Shelterbelts, in order (the order seeds their natural gaps).
    pub belts: Vec<BeltSpec>,
    pub routes: Vec<RouteSpec>,
    #[serde(default)]
    pub shoulder_fields: Vec<ShoulderSpec>,
    /// Belts of mines in open field: a line and its width.
    #[serde(default)]
    pub field_mines: Vec<(Vec<P>, f32)>,
    #[serde(default)]
    pub barriers: Vec<BarrierSpec>,
    /// The ground drawn from a map (tools/osm): its lines, roads and tree rows join the hand-drawn ones.
    #[serde(default)]
    pub osm: OsmSpec,
    /// The line that is the river, if there is one.
    #[serde(default)]
    pub river: Option<String>,
    #[serde(default)]
    pub town: Option<(P, P)>,
    #[serde(default)]
    pub dachas: Option<(P, P)>,
    #[serde(default)]
    pub pavlivka: Option<(P, P)>,
    #[serde(default)]
    pub mykilske: Option<(P, P)>,
    #[serde(default)]
    pub ua_positions: Vec<P>,
    #[serde(default)]
    pub ru_positions: Vec<P>,
    /// Hides for a Ukrainian tank firing on coordinates from a drone pilot: where, and which way it faces.
    #[serde(default)]
    pub hides: Vec<(P, P)>,
    /// Where the player's columns come in (design/command.md): a name in English and Ukrainian, and the place.
    #[serde(default)]
    pub road_heads: Vec<RoadHeadSpec>,
    /// The grey zone, from this z to that (the enemy is to +z): nothing of ours is placed in it or beyond.
    pub grey_zone: (f32, f32),
    #[serde(default)]
    pub trps: Vec<PointSpec>,
    #[serde(default)]
    pub old_wrecks: Vec<(PointSpec, f32)>,
    /// Which way the attack goes along z: -1 north, +1 south.
    pub advance: f32,
    pub foothold_z: f32,
    pub defence_z: f32,
    /// The defenders' trench, where there is one painted across the field.
    #[serde(default)]
    pub trench_z: Option<f32>,
    /// The attackers' start line, where there is one (the forward tree line they leave from).
    #[serde(default)]
    pub start_z: Option<f32>,
    pub home: P,
    pub jam_z: f32,
    pub mortar_band: (f32, f32),
}

/// What tools/osm/osm2ron.py makes of a map: lines (roads, taxiways, railways), the roads on them, tree rows, land
/// cover in painting order, buildings, and which lines are railways.
#[derive(Deserialize, Debug, Default)]
pub struct OsmSpec {
    pub lines: Vec<(String, LineSpec)>,
    pub roads: Vec<RoadSpec>,
    pub rails: Vec<String>,
    /// Each runway's centreline ends and width, before the box clips it.
    #[serde(default)]
    pub runways: Vec<(P, P, f32)>,
    pub belts: Vec<BeltSpec>,
    pub areas: Vec<(crate::geo::AreaKind, Vec<P>)>,
    pub buildings: Vec<BuildingSpec>,
}

#[derive(Deserialize, Debug)]
pub struct BuildingSpec {
    pub kind: crate::geo::BuildingKind,
    pub height: f32,
    pub foot: Vec<P>,
}

fn yes() -> bool {
    true
}

/// The copy that is one place's: the briefing, the observers' post, the lines on the net that name its ground, and
/// the debrief's close.
#[derive(Deserialize, Debug)]
pub struct Words {
    pub briefing: String,
    /// The observers' post as the HUD names it, and the line when it is lost.
    pub op_label: String,
    pub op_lost: String,
    /// Where our ATGM teams fire from, as the hit feed says it.
    pub forward_belt: String,
    /// A mine under a plow that stays on its tracks.
    pub plow_mine: String,
    /// Mines under the tracks: one of these, at random.
    pub mine_lines: Vec<String>,
    /// The brigade's guns on a registered point `{n}`: in Ukrainian and English (no Ukrainian: the net's own table).
    pub registered: (String, String),
    /// The debrief: the lines about what came after, and the keys.
    pub closing: Vec<String>,
    pub keys: String,
    pub sources: Vec<SourceSpec>,
}

/// A source the day rests on.
#[derive(Deserialize, Debug)]
pub struct SourceSpec {
    pub publisher: String,
    pub date: String,
    pub title: String,
    pub url: String,
}

/// Where adjusted fire is observed from.
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ObservedFrom {
    /// A line across the map at this z (a town's edge).
    LineZ(f32),
    /// The player's own home.
    Home,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoadHeadSpec {
    pub name: (String, String),
    pub at: P,
}

/// Who is who on the net: the player, the guns, the observation post, the brigade's command post.
#[derive(Deserialize, Debug)]
pub struct Callsigns {
    pub player: (String, String),
    pub guns: (String, String),
    pub op: (String, String),
    pub command: (String, String),
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Anchor {
    pub lat: f64,
    pub lon: f64,
    pub x: f64,
    pub z: f64,
    pub m_per_lon: f64,
}

/// One term of the ground's height.
#[derive(Deserialize, Debug, Clone)]
pub enum HeightTerm {
    /// The starting height: `base + amp * ramp(z)^power`.
    Ramp { base: f32, amp: f32, power: f32, ramp: RampKind },
    /// A ridge along a line of constant z: `+ amp * exp(-((z - z0) / width)²)`.
    RidgeZ { z: f32, width: f32, amp: f32 },
    /// Value noise: `+ fbm(x / scale, z / scale, octaves, seed) * amp`.
    Noise { scale: f32, octaves: u32, seed: u32, amp: f32 },
    /// A valley along a named curve z(x): `- Σ amp * exp(-(d / width)²)` for each (amp, width), d the distance
    /// across the valley from the curve.
    Valley { curve: String, profile: Vec<(f32, f32)> },
    /// A gully running along z, its floor at `x0 + amp * sin((z - z_offset) / period + phase)`:
    /// `- depth * fade(z) * exp(-((x - floor) / width)²)`.
    Gully { x0: f32, amp: f32, period: f32, z_offset: f32, phase: f32, width: f32, depth: f32, fade: Fade },
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum RampKind {
    /// `1 - smoothstep(z0, z1, z)`: high in the north, falling away south.
    Falling(f32, f32),
    /// `((z - z0) / (z1 - z0)).clamp(0, 1)`: rising towards the south.
    Rising(f32, f32),
}

/// How a gully's depth changes along it: `In` deepens over (z0, z1), `Out` fades over it.
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Fade {
    In(f32, f32),
    Out(f32, f32),
}

/// A curve z(x): `base + slope * x + Σ amp * sin(x / period + phase)`.
#[derive(Deserialize, Debug, Clone)]
pub struct Curve {
    pub base: f32,
    #[serde(default)]
    pub slope: f32,
    #[serde(default)]
    pub waves: Vec<(f32, f32, f32)>,
}

impl Curve {
    pub fn at(&self, x: f32) -> f32 {
        let mut z = self.base;
        if self.slope != 0.0 {
            z += self.slope * x;
        }
        for &(amp, period, phase) in &self.waves {
            z += amp * (x / period + phase).sin();
        }
        z
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum LineSpec {
    /// Straight between the points.
    Points(Vec<P>),
    /// The points, rounded by this many corner-cutting passes.
    Smoothed(Vec<P>, usize),
    /// Along a named curve, at n + 1 points from x0 across span.
    Sampled { curve: String, x0: f32, span: f32, n: u32 },
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoadSpec {
    pub line: String,
    pub width: f32,
    pub kind: RoadKind,
}

#[derive(Deserialize, Debug, Clone)]
pub enum BeltSpec {
    /// East-west belts across the box at these z, each with its width, waving `amp * sin(x / period + i *
    /// phase_step)` (i the row's index), at n + 1 points from x0 across span.
    Rows { rows: Vec<(f32, f32)>, amp: f32, period: f32, phase_step: f32, x0: f32, span: f32, n: u32 },
    /// Straight belts, each between two points, all this wide.
    Lines { lines: Vec<(P, P)>, width: f32 },
}

#[derive(Deserialize, Debug, Clone)]
pub struct RouteSpec {
    pub line: String,
    /// Where the columns fan out: the point on the line nearest this.
    pub deploy: P,
    pub objective: P,
}

/// A minefield along a road's shoulders: from where the road first climbs past `from_z`, to `end_margin`
/// metres short of its end, between `inner` and `outer` metres off the centreline, one mine per `per` m².
#[derive(Deserialize, Debug, Clone)]
pub struct ShoulderSpec {
    pub road: usize,
    pub from_z: f32,
    pub end_margin: f32,
    pub inner: f32,
    pub outer: f32,
    pub per: f32,
}

/// A barrier minefield from z0 to z1: one anti-tank mine per `at_per` m² (`stacked` of them two high), one
/// anti-personnel mine per `ap_per` m².
#[derive(Deserialize, Debug, Clone)]
pub struct BarrierSpec {
    pub z0: f32,
    pub z1: f32,
    pub at_per: f32,
    pub stacked: f32,
    pub ap_per: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub enum PointSpec {
    At(P),
    /// The point on a named line nearest this one.
    OnLine(String, P),
    /// ...and from there, moved by the offset.
    OffLine(String, P, P),
}

/// The built-in files, by diorama. A diorama of several files (the sub-chapters of one place share its map) is read
/// as one: their fields, in this order, make one record (`merge`).
fn builtin(d: Diorama) -> &'static [(&'static str, &'static str)] {
    const HOSTOMEL_MAP: (&str, &str) = ("hostomel/map.ron", include_str!("../../../dioramas/hostomel/map.ron"));
    const HOSTOMEL_OSM: (&str, &str) = ("hostomel/osm.ron", include_str!("../../../dioramas/hostomel/osm.ron"));
    match d {
        Diorama::Vuhledar => &[("vuhledar.ron", include_str!("../../../dioramas/vuhledar.ron"))],
        Diorama::Robotyne => &[("robotyne.ron", include_str!("../../../dioramas/robotyne.ron"))],
        Diorama::HostomelLanding => &[HOSTOMEL_MAP, HOSTOMEL_OSM, ("hostomel/landing.ron", include_str!("../../../dioramas/hostomel/landing.ron"))],
        Diorama::HostomelCounter => &[HOSTOMEL_MAP, HOSTOMEL_OSM, ("hostomel/counter.ron", include_str!("../../../dioramas/hostomel/counter.ron"))],
    }
}

/// RON without its comments (`//` to the line's end, `/* */`), the strings left alone (`"..."` with escapes, and raw
/// `r#"..."#`, which the briefings are).
fn uncomment(t: &str) -> String {
    let b = t.as_bytes();
    let mut out = String::with_capacity(t.len());
    let mut i = 0;
    let mut from = 0;
    while i < b.len() {
        match b[i] {
            b'r' if b.get(i + 1) == Some(&b'#') || (b.get(i + 1) == Some(&b'"') && (i == 0 || !b[i - 1].is_ascii_alphanumeric())) => {
                let hashes = b[i + 1..].iter().take_while(|&&c| c == b'#').count();
                if b.get(i + 1 + hashes) != Some(&b'"') {
                    i += 1;
                    continue;
                }
                let end = format!("\"{}", "#".repeat(hashes));
                let start = i + 2 + hashes;
                i = t[start..].find(&end).map_or(b.len(), |k| start + k + end.len());
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
            }
            b'/' if b.get(i + 1) == Some(&b'/') => {
                out.push_str(&t[from..i]);
                i = t[i..].find('\n').map_or(b.len(), |k| i + k);
                from = i;
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                out.push_str(&t[from..i]);
                i = t[i..].find("*/").map_or(b.len(), |k| i + k + 2);
                from = i;
            }
            _ => i += 1,
        }
    }
    out.push_str(&t[from.min(b.len())..]);
    out
}

/// Several files' records as one: each file is a record `( ... )` after its comments, and their fields are joined in
/// order. One file is returned as it is, so its errors point at its own lines.
fn merge(texts: &[String]) -> String {
    if let [one] = texts {
        return one.clone();
    }
    let body = |t: &str| {
        // Comments out first (they may hold brackets, or follow the last field), then the record's brackets off.
        let t = uncomment(t);
        let inner = t.trim();
        let (open, close) = (inner.find('(').expect("a diorama file without its record"), inner.rfind(')').expect("a diorama file without its record"));
        inner[open + 1..close].trim_end().trim_end_matches(',').to_string()
    };
    format!("({})", texts.iter().map(|t| body(t)).collect::<Vec<_>>().join(",\n"))
}

static SOURCE: OnceLock<(String, String)> = OnceLock::new();

/// The file of the diorama on the table: its name (or path) and its text. Each crate reads its own part of it:
/// this module the place and the layout, `bav_sim` the forces and the day's script.
pub fn source() -> (&'static str, &'static str) {
    let (name, text) = SOURCE.get_or_init(|| {
        let files = builtin(diorama());
        let names = files.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(" + ");
        let texts: Vec<String> = match std::env::var("BAV_DATA").ok().filter(|d| !d.is_empty()) {
            Some(dir) => files
                .iter()
                .map(|(name, _)| {
                    let path = std::path::Path::new(&dir).join(name);
                    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
                })
                .collect(),
            None => files.iter().map(|(_, t)| t.to_string()).collect(),
        };
        (names, merge(&texts))
    });
    (name, text)
}

/// Parse this crate's or another's part of the diorama's file; a file that does not parse stops the game with
/// the file, line and column.
pub fn read<T: serde::de::DeserializeOwned>() -> T {
    let (name, text) = source();
    ron::from_str(text).unwrap_or_else(|e| panic!("{name}: {e}"))
}

static DATA: OnceLock<DioramaData> = OnceLock::new();

/// The diorama on the table, as its file describes it.
pub fn data() -> &'static DioramaData {
    DATA.get_or_init(read)
}

/// A named curve of the diorama on the table.
pub fn curve(name: &str) -> &'static Curve {
    data().curves.iter().find(|(n, _)| n == name).map(|(_, c)| c).unwrap_or_else(|| panic!("no curve {name:?}"))
}

/// The ground's height terms, with the valleys' curves looked up once.
pub enum Term {
    Ramp { base: f32, amp: f32, power: f32, ramp: RampKind },
    RidgeZ { z: f32, width: f32, amp: f32 },
    Noise { scale: f32, octaves: u32, seed: u32, amp: f32 },
    Valley { curve: Curve, profile: Vec<(f32, f32)> },
    Gully { x0: f32, amp: f32, period: f32, z_offset: f32, phase: f32, width: f32, depth: f32, fade: Fade },
}

static GROUND: OnceLock<Vec<Term>> = OnceLock::new();

/// Ground height in metres above sea level at (x, z).
pub fn height(x: f32, z: f32) -> f32 {
    let terms = GROUND.get_or_init(|| {
        data()
            .ground
            .iter()
            .map(|t| match t.clone() {
                HeightTerm::Ramp { base, amp, power, ramp } => Term::Ramp { base, amp, power, ramp },
                HeightTerm::RidgeZ { z, width, amp } => Term::RidgeZ { z, width, amp },
                HeightTerm::Noise { scale, octaves, seed, amp } => Term::Noise { scale, octaves, seed, amp },
                HeightTerm::Valley { curve: c, profile } => Term::Valley { curve: curve(&c).clone(), profile },
                HeightTerm::Gully { x0, amp, period, z_offset, phase, width, depth, fade } => Term::Gully { x0, amp, period, z_offset, phase, width, depth, fade },
            })
            .collect()
    });
    let mut h = 0.0;
    for t in terms {
        match t {
            Term::Ramp { base, amp, power, ramp } => {
                let r = match *ramp {
                    RampKind::Falling(z0, z1) => 1.0 - smoothstep(z0, z1, z),
                    RampKind::Rising(z0, z1) => ((z - z0) / (z1 - z0)).clamp(0.0, 1.0),
                };
                h = base + amp * r.powf(*power);
            }
            Term::RidgeZ { z: z0, width, amp } => h += amp * (-((z - z0) / width).powi(2)).exp(),
            Term::Noise { scale, octaves, seed, amp } => h += fbm(x / scale, z / scale, *octaves, *seed) * amp,
            Term::Valley { curve, profile } => {
                let d = (z - curve.at(x)).abs();
                let mut cut = 0.0;
                for &(amp, width) in profile {
                    cut += amp * (-(d / width).powi(2)).exp();
                }
                h -= cut;
            }
            Term::Gully { x0, amp, period, z_offset, phase, width, depth, fade } => {
                let floor = x0 + amp * ((z - z_offset) / period + phase).sin();
                let f = match *fade {
                    Fade::In(z0, z1) => smoothstep(z0, z1, z),
                    Fade::Out(z0, z1) => 1.0 - smoothstep(z0, z1, z),
                };
                let depth = depth * f;
                h -= depth * (-((x - floor) / width).powi(2)).exp();
            }
        }
    }
    h
}

/// Arc length where a road first climbs (northbound) past `z`.
fn s_at_z(road: &Polyline, z: f32) -> f32 {
    let n = (road.length() / 4.0) as usize;
    (0..=n).map(|k| k as f32 * 4.0).find(|&s| road.point(s).y <= z).unwrap_or(0.0)
}

/// The layout the file describes.
pub fn layout(d: &DioramaData) -> Layout {
    let mut lines: HashMap<&str, Polyline> = HashMap::new();
    for (name, spec) in d.lines.iter().chain(&d.osm.lines) {
        let line = match spec {
            LineSpec::Points(pts) => Polyline::new(pts.iter().copied().map(v).collect()),
            LineSpec::Smoothed(pts, k) => Polyline::smoothed(pts.iter().copied().map(v).collect(), *k),
            LineSpec::Sampled { curve: c, x0, span, n } => {
                let c = d.curves.iter().find(|(cn, _)| cn == c).map(|(_, c)| c).unwrap_or_else(|| panic!("no curve {c:?}"));
                Polyline::new(
                    (0..=*n)
                        .map(|i| {
                            let x = x0 + i as f32 * span / *n as f32;
                            Vec2::new(x, c.at(x))
                        })
                        .collect(),
                )
            }
        };
        lines.insert(name, line);
    }
    let line = |name: &str| lines.get(name).unwrap_or_else(|| panic!("no line {name:?}"));
    let roads: Vec<Road> = d.roads.iter().chain(&d.osm.roads).map(|r| Road { line: line(&r.line).clone(), width: r.width, kind: r.kind }).collect();
    let mut belts = Vec::new();
    for spec in d.belts.iter().chain(&d.osm.belts) {
        match spec {
            BeltSpec::Rows { rows, amp, period, phase_step, x0, span, n } => {
                for (i, &(z, width)) in rows.iter().enumerate() {
                    let wob = |x: f32| z + amp * (x / period + i as f32 * phase_step).sin();
                    let line = Polyline::new(
                        (0..=*n)
                            .map(|k| {
                                let x = x0 + k as f32 * span / *n as f32;
                                Vec2::new(x, wob(x))
                            })
                            .collect(),
                    );
                    belts.push(Belt { line, width, gaps: vec![] });
                }
            }
            BeltSpec::Lines { lines: ls, width } => {
                for &(a, b) in ls {
                    belts.push(Belt { line: Polyline::new(vec![v(a), v(b)]), width: *width, gaps: vec![] });
                }
            }
        }
    }
    cut_gaps(&mut belts, &roads);
    let routes = d
        .routes
        .iter()
        .map(|r| {
            let l = line(&r.line);
            Route { line: l.clone(), deploy_s: l.project(v(r.deploy)).1, objective: v(r.objective), hold: false }
        })
        .collect();
    let shoulder_fields = d
        .shoulder_fields
        .iter()
        .map(|f| {
            let road = &roads[f.road].line;
            ShoulderField { road: f.road, s0: s_at_z(road, f.from_z), s1: road.length() - f.end_margin, inner: f.inner, outer: f.outer, density: 1.0 / f.per }
        })
        .collect();
    let point = |p: &PointSpec| match p {
        PointSpec::At(p) => v(*p),
        PointSpec::OnLine(name, p) => {
            let l = line(name);
            l.point(l.project(v(*p)).1)
        }
        PointSpec::OffLine(name, p, off) => {
            let l = line(name);
            l.point(l.project(v(*p)).1) + v(*off)
        }
    };
    let rect = |r: Option<(P, P)>| r.map_or(Rect { min: Vec2::ZERO, max: Vec2::ZERO }, |(a, b)| Rect { min: v(a), max: v(b) });
    Layout {
        roads,
        belts,
        routes,
        shoulder_fields,
        field_mines: d.field_mines.iter().map(|(pts, w)| (Polyline::new(pts.iter().copied().map(v).collect()), *w)).collect(),
        barriers: d.barriers.iter().map(|b| Barrier { z0: b.z0, z1: b.z1, at: 1.0 / b.at_per, stacked: b.stacked, ap: 1.0 / b.ap_per }).collect(),
        areas: d.osm.areas.iter().map(|(k, pts)| crate::geo::Area::new(*k, pts.iter().copied().map(v).collect())).collect(),
        buildings: d.osm.buildings.iter().map(|b| crate::geo::Building { kind: b.kind, height: b.height, foot: b.foot.iter().copied().map(v).collect() }).collect(),
        runways: d.osm.runways.iter().map(|&(a, b, w)| (v(a), v(b), w)).collect(),
        rails: d.osm.rails.iter().map(|r| line(r).clone()).collect(),
        river: d.river.as_deref().map(|r| line(r).clone()),
        town: rect(d.town),
        dachas: rect(d.dachas),
        pavlivka: rect(d.pavlivka),
        mykilske: rect(d.mykilske),
        ua_positions: d.ua_positions.iter().copied().map(v).collect(),
        ru_positions: d.ru_positions.iter().copied().map(v).collect(),
        trps: d.trps.iter().map(point).collect(),
        old_wrecks: d.old_wrecks.iter().map(|(p, yaw)| (point(p), *yaw)).collect(),
        advance: d.advance,
        foothold_z: d.foothold_z,
        defence_z: d.defence_z,
        trench_z: d.trench_z,
        start_z: d.start_z,
        home: v(d.home),
        jam_z: d.jam_z,
        mortar_band: d.mortar_band,
    }
}

#[cfg(test)]
mod merge_tests {
    use super::{merge, uncomment};

    #[test]
    fn comments_go_and_strings_stay() {
        let t = "( a: \"https://x.org/y\", // a comment (with a bracket)\n b: r#\"raw // kept\"#, /* c ) */ c: 1 // last\n) // after )";
        let u = uncomment(t);
        assert!(u.contains("https://x.org/y") && u.contains("raw // kept"));
        assert!(!u.contains("a comment") && !u.contains("c )") && !u.contains("after"));
    }

    #[test]
    fn files_join_as_one_record() {
        let a = "// head\n(\n    x: 1 // no comma, then a comment\n)\n".to_string();
        let b = "(\n    y: \"(\",\n)\n".to_string();
        let m = merge(&[a, b]);
        #[derive(serde::Deserialize)]
        struct Xy {
            x: u32,
            y: String,
        }
        let v: Xy = ron::from_str(&m).unwrap();
        assert_eq!((v.x, v.y.as_str()), (1, "("));
    }
}

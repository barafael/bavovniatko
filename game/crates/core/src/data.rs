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
    /// The season the ground, the trees and the weather are painted for.
    pub season: Season,
    /// The net's callsigns, in English and Ukrainian (`crate::text::fill`).
    pub callsigns: Callsigns,
    /// The map's anchor: the latitude and longitude at (x, z), metres per degree of longitude there, and which way
    /// the map is turned.
    pub anchor: Anchor,
    /// The sun's day: the day of the year, and local time's offset from UTC (hours). None: the place keeps its own
    /// hand-made curve.
    #[serde(default)]
    pub sun: Option<(u32, f32)>,
    /// The date the clock starts on (year, month, day), where the moon matters (a night part): the moon is worked
    /// out for it (`sun::moon`). None: no moon is drawn.
    #[serde(default)]
    pub moon: Option<(i32, u32, u32)>,
    /// Whether the player's drone has a thermal channel here (a Mavic 2 Enterprise Advanced, key I): the hot things
    /// then carry their heat (`bav_look::thermal`).
    #[serde(default)]
    pub thermal: bool,
    /// What is burning on the field when the day starts, and lights the dark: fires at these points, each so many
    /// flames across. Where there are any, a building a heavy bomb lands on catches fire too (`bav_look::fx`).
    #[serde(default)]
    pub fires: Vec<(P, u32)>,
    /// An overcast day, where the place lights itself from the computed sun (`sun`): a diffuse key and no hard
    /// shadows (the sky switch `BAV_SKY` overrides it).
    #[serde(default)]
    pub overcast: bool,
    /// The fog through the day: how far one sees through it (m) at clock times (minutes after midnight), between
    /// them in a straight line, the first and last held before and after. The picture draws it and the battle sees
    /// through it (`bav_sim::fight::Seeing`). Empty: clear air.
    #[serde(default)]
    pub fog: Vec<(f32, f32)>,
    /// The day's wind (m/s along x, y, z), where it is not the season's: the drone's drift, the smoke, the drifts.
    #[serde(default)]
    pub wind: Option<(f32, f32, f32)>,
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
    /// Which of the map's belts of `field_mines` a part has, by number (none: all of them). Avdiivka's night mines
    /// lay on the approaches in October and were not on the village's own approach in January.
    #[serde(default)]
    pub mines_laid: Option<Vec<usize>>,
    #[serde(default)]
    pub barriers: Vec<BarrierSpec>,
    /// The ground drawn from a map (tools/osm): its lines, roads and tree rows join the hand-drawn ones.
    #[serde(default)]
    pub osm: OsmSpec,
    /// Houses the map does not have, raised along the streets inside these outlines (`HouseSpec`): Myrnohrad's
    /// villages and its private sector, whose houses OpenStreetMap leaves out.
    #[serde(default)]
    pub houses: Vec<HouseSpec>,
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
    /// The grey zone, from this z to that (the enemy is to +z): nothing of ours is placed in it or beyond. None: no
    /// front across the field (Hostomel's fight is for places, not a line), so all of it is ours to place on.
    #[serde(default)]
    pub grey_zone: Option<(f32, f32)>,
    /// The front as outlines, where it is not one line across the field (Myrnohrad's pocket, with the enemy on three
    /// sides): the grey zone's and the enemy's ground (`field::Pocket`). None: built from `grey_zone`.
    #[serde(default)]
    pub pocket: Option<PocketSpec>,
    #[serde(default)]
    pub trps: Vec<PointSpec>,
    #[serde(default)]
    pub old_wrecks: Vec<(PointSpec, f32)>,
    /// Which way the attack goes along z: -1 north, +1 south.
    pub advance: f32,
    #[serde(default)]
    pub foothold_z: Option<f32>,
    #[serde(default)]
    pub defence_z: Option<f32>,
    /// The defenders' trench, where there is one painted across the field.
    #[serde(default)]
    pub trench_z: Option<f32>,
    /// The attackers' start line, where there is one (the forward tree line they leave from).
    #[serde(default)]
    pub start_z: Option<f32>,
    pub home: P,
    /// Where the drone team sits in this part, in place of the map's `home` and `operator` (a place whose parts share
    /// one map and whose team moves between them: Myrnohrad's in the frost, out of a basement that is now theirs).
    #[serde(default)]
    pub team: Option<P>,
    #[serde(default)]
    pub jam_z: Option<f32>,
    #[serde(default)]
    pub mortar_band: Option<(f32, f32)>,
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

/// A stretch of houses the map leaves out: on both sides of every street inside `within`, one every `every` metres
/// or so, set back from the street, a share `two` of them of two storeys, where no building of the map stands near
/// and on no other street. The places and sizes are drawn from `seed`, so they stand where they stood every day.
#[derive(Deserialize, Debug, Clone)]
pub struct HouseSpec {
    pub within: Vec<P>,
    pub every: f32,
    #[serde(default)]
    pub two: f32,
    pub seed: u32,
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
    /// A mine under a plow that stays on its tracks (none: no plows that day).
    #[serde(default)]
    pub plow_mine: Option<String>,
    /// Mines under the tracks: one of these, at random.
    pub mine_lines: Vec<String>,
    /// The defenders' guns on a registered point `{n}`, as our net hears it: in Ukrainian and English (no Ukrainian:
    /// the net's own table). Where the defenders are theirs (Mala Tokmachka) it is our warning.
    #[serde(default)]
    pub registered: Option<(String, String)>,
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

/// The grey zone and the enemy's ground, each a list of outlines; the rest of the box is ours.
#[derive(Deserialize, Debug, Clone)]
pub struct PocketSpec {
    pub grey: Vec<Vec<P>>,
    pub theirs: Vec<Vec<P>>,
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
    /// The compass bearing of the map's +z, in degrees: 180 (south, the default) keeps +x east, as the first
    /// dioramas were drawn; a map turned to face its attack (Avdiivka, where the columns come from the north-east)
    /// gives the bearing they come from, so the column game still runs along z.
    #[serde(default = "south")]
    pub bearing: f64,
}

fn south() -> f64 {
    180.0
}

/// The season a diorama is painted for: its ground, trees, dust or snow, and wind.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Winter,
    Summer,
    Autumn,
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
    /// A spoil heap: `+ height * (1 - smoothstep(top, radius, d))` for d the distance from `at`, a flat top out to
    /// `top`, then the face falling to the ground at `radius`, with terraces every `terrace` metres of height
    /// (0: a smooth face).
    Mound { at: P, radius: f32, top: f32, height: f32, terrace: f32 },
    /// A spoil heap on the map's own outline: the `Spoil` area of the map (`osm`) that holds `inside` rises to
    /// `height` over `rim` metres in from its edge, in terraces every `terrace` metres of height (0: a smooth
    /// face), the face and the benches roughened by noise so no two stretches of it are alike.
    Heap { inside: P, height: f32, rim: f32, terrace: f32 },
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
    /// Another line (the map's own or the OSM file's), walked the other way.
    Reversed(String),
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

/// The built-in files, by diorama (`dioramas/index.ron`). A diorama of several files (the sub-chapters of one place
/// share its map) is read as one: their fields, in this order, make one record (`merge`).
fn builtin(d: Diorama) -> &'static [(&'static str, &'static str)] {
    d.files()
}

static SHOTS: OnceLock<Option<(String, String)>> = OnceLock::new();

/// The shots file of the diorama on the table, if it has one: its name and its text (`BAV_DATA=<dir>` reads it from
/// there, as it does the diorama's files).
pub fn shots_source() -> Option<(&'static str, &'static str)> {
    SHOTS
        .get_or_init(|| {
            let (name, text) = diorama().shots()?;
            Some(match std::env::var("BAV_DATA").ok().filter(|d| !d.is_empty()) {
                Some(dir) => {
                    let path = std::path::Path::new(&dir).join(name);
                    (name.to_string(), std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
                }
                None => (name.to_string(), text.to_string()),
            })
        })
        .as_ref()
        .map(|(n, t)| (n.as_str(), t.as_str()))
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

/// A diorama's built-in files as the one record they make (for checks over every diorama; the game reads
/// `source`).
pub fn builtin_text(d: Diorama) -> String {
    merge(&builtin(d).iter().map(|(_, t)| t.to_string()).collect::<Vec<_>>())
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
/// the file, line and column. An optional field is written as its value (`trench_z: 702.0`), not `Some(702.0)`.
pub fn read<T: serde::de::DeserializeOwned>() -> T {
    let (name, text) = source();
    parse(text).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// RON as the dioramas are written: with implicit `Some`.
pub fn parse<T: serde::de::DeserializeOwned>(text: &str) -> ron::error::SpannedResult<T> {
    ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME).from_str(text)
}

static DATA: OnceLock<DioramaData> = OnceLock::new();

/// The diorama on the table, as its file describes it.
pub fn data() -> &'static DioramaData {
    DATA.get_or_init(read)
}

/// How far one sees through the day's fog at this clock time (minutes after midnight), where the file gives one
/// (`fog`) and `BAV_FOG=off` does not lift it.
pub fn fog_at(minutes: f32) -> Option<f32> {
    if crate::dev::dev().fog_off {
        return None;
    }
    let fog = &data().fog;
    let (first, last) = (fog.first()?, fog.last()?);
    if minutes <= first.0 {
        return Some(first.1);
    }
    if minutes >= last.0 {
        return Some(last.1);
    }
    let k = fog.windows(2).find(|w| minutes < w[1].0)?;
    let t = (minutes - k[0].0) / (k[1].0 - k[0].0);
    Some(k[0].1 + (k[1].1 - k[0].1) * t)
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
    Mound { at: Vec2, radius: f32, top: f32, height: f32, terrace: f32 },
    Heap { poly: Vec<Vec2>, min: Vec2, max: Vec2, height: f32, rim: f32, terrace: f32 },
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
                HeightTerm::Mound { at, radius, top, height, terrace } => Term::Mound { at: v(at), radius, top, height, terrace },
                HeightTerm::Heap { inside, height, rim, terrace } => {
                    let at = v(inside);
                    let poly: Vec<Vec2> = data()
                        .osm
                        .areas
                        .iter()
                        .filter(|(k, _)| *k == crate::geo::AreaKind::Spoil)
                        .map(|(_, pts)| pts.iter().copied().map(v).collect::<Vec<Vec2>>())
                        .find(|poly| crate::geo::inside(poly, at))
                        .unwrap_or_else(|| panic!("no spoil heap on the map holds ({}, {})", at.x, at.y));
                    let min = poly.iter().fold(Vec2::splat(f32::MAX), |a, p| a.min(*p));
                    let max = poly.iter().fold(Vec2::splat(f32::MIN), |a, p| a.max(*p));
                    Term::Heap { poly, min, max, height, rim, terrace }
                }
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
            Term::Mound { at, radius, top, height, terrace } => {
                let d = Vec2::new(x, z).distance(*at);
                h += terraced(height * (1.0 - smoothstep(*top, *radius, d)), *height, *terrace, 0.0);
            }
            Term::Heap { poly, min, max, height, rim, terrace } => {
                let p = Vec2::new(x, z);
                if p.x < min.x - 40.0 || p.y < min.y - 40.0 || p.x > max.x + 40.0 || p.y > max.y + 40.0 {
                    continue;
                }
                let n = poly.len();
                let edge = (0..n).map(|i| crate::geo::segment_distance(p, poly[i], poly[(i + 1) % n])).fold(f32::MAX, f32::min);
                // The tip's edge is never where the map drew it to the metre: the foot wanders in and out.
                let wander = fbm(x / 70.0, z / 70.0, 2, 61) * 25.0;
                let d = if crate::geo::inside(poly, p) { edge + wander } else { wander - edge };
                if d <= 0.0 {
                    continue;
                }
                let rise = height * smoothstep(0.0, *rim, d);
                h += terraced(rise, *height, *terrace, fbm(x / 35.0, z / 35.0, 2, 62) * 2.5);
            }
        }
    }
    h
}

/// A rise up a spoil heap's face in benches: every `terrace` metres of height a flat few metres, then the next face;
/// `wobble` shifts where the benches fall so they are not rings. Nothing below the ground or above the top.
fn terraced(rise: f32, height: f32, terrace: f32, wobble: f32) -> f32 {
    if terrace <= 0.0 || rise <= 0.0 || rise >= height - 0.5 {
        return rise.clamp(0.0, height);
    }
    let r = (rise + wobble).clamp(0.0, height);
    let k = (r / terrace).floor();
    let frac = r / terrace - k;
    ((k + smoothstep(0.3, 1.0, frac)) * terrace).clamp(0.0, height)
}

/// The houses of a stretch the map leaves out (`HouseSpec`): along each street that runs inside the outline, on
/// both sides, a plot every `every` metres or so; on it a house of 8-11 by 7-10 m facing the street, 9-13 m back
/// from its edge, where it is clear of every other street, of the map's own buildings and of the houses already
/// placed.
fn houses(spec: &HouseSpec, roads: &[Road], mapped: &[crate::geo::Building], areas: &[crate::geo::Area]) -> Vec<crate::geo::Building> {
    use crate::geo::{AreaKind, BuildingKind, hash01, in_bounds, inside};
    let within: Vec<Vec2> = spec.within.iter().copied().map(v).collect();
    let mapped: Vec<Vec2> = mapped.iter().map(|b| b.foot.iter().copied().sum::<Vec2>() / b.foot.len().max(1) as f32).collect();
    let mut out: Vec<crate::geo::Building> = Vec::new();
    let mut placed: HashMap<(i32, i32), Vec<Vec2>> = HashMap::new();
    let cell = |p: Vec2| ((p.x / 20.0).floor() as i32, (p.y / 20.0).floor() as i32);
    let mut n = 0u32;
    for (ri, road) in roads.iter().enumerate() {
        // Not along the highway (its verges are fields and garages, its houses on the streets off it).
        if road.width > 8.0 {
            continue;
        }
        let len = road.line.length();
        let mut s = spec.every * 0.5;
        while s < len {
            let (p, t) = (road.line.point(s), road.line.tangent(s));
            for side in [-1.0, 1.0] {
                n += 1;
                let h = |k: u32| hash01(n.wrapping_mul(7919).wrapping_add(ri as u32 * 31), spec.seed.wrapping_mul(101).wrapping_add(k));
                let (w, deep) = (8.0 + 3.0 * h(1), 7.0 + 3.0 * h(2));
                let back = road.width * 0.5 + 9.0 + 4.0 * h(3) + deep * 0.5;
                let normal = t.perp() * side;
                let at = p + normal * back + t * (h(4) - 0.5) * 4.0;
                let clear_of_streets = roads.iter().enumerate().all(|(rj, r)| rj == ri || r.line.project(at).0 > r.width * 0.5 + deep * 0.5 + 4.0);
                let (ci, cj) = cell(at);
                let near_house = (-1..=1).any(|di| (-1..=1).any(|dj| placed.get(&(ci + di, cj + dj)).is_some_and(|v| v.iter().any(|q| q.distance(at) < 13.0))));
                // Not in the works, on a heap, in the water, the woods or a cemetery.
                let open = |a: &&crate::geo::Area| !matches!(a.kind, AreaKind::Farmland | AreaKind::Allotments | AreaKind::Residential) && a.contains(at);
                if !inside(&within, at) || !in_bounds(at) || !clear_of_streets || near_house || mapped.iter().any(|q| q.distance(at) < 22.0) || areas.iter().any(|a| open(&a)) {
                    continue;
                }
                // Turned a few degrees off the street, as they were built.
                let turn = (h(5) - 0.5) * 0.12;
                let (a, b) = (t.rotate(Vec2::from_angle(turn)), normal.rotate(Vec2::from_angle(turn)));
                let foot = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].iter().map(|&(u, k)| at + a * u * w * 0.5 + b * k * deep * 0.5).collect();
                let storeys = if h(6) < spec.two { 2.0 } else { 1.0 };
                out.push(crate::geo::Building { kind: BuildingKind::House, height: storeys * 2.8 + 0.3 + h(7) * 0.4, foot });
                placed.entry((ci, cj)).or_default().push(at);
            }
            s += spec.every * (0.85 + 0.3 * hash01(n, spec.seed.wrapping_add(17)));
        }
    }
    out
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
            // Resolved below, once every line it could name is built.
            LineSpec::Reversed(_) => continue,
        };
        lines.insert(name, line);
    }
    for (name, spec) in d.lines.iter().chain(&d.osm.lines) {
        if let LineSpec::Reversed(of) = spec {
            let of = lines.get(of.as_str()).unwrap_or_else(|| panic!("{name:?} reverses no line {of:?}"));
            let line = Polyline::new(of.pts.iter().rev().copied().collect());
            lines.insert(name, line);
        }
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
    let areas: Vec<crate::geo::Area> = d.osm.areas.iter().map(|(k, pts)| crate::geo::Area::new(*k, pts.iter().copied().map(v).collect())).collect();
    let mut buildings: Vec<crate::geo::Building> = d.osm.buildings.iter().map(|b| crate::geo::Building { kind: b.kind, height: b.height, foot: b.foot.iter().copied().map(v).collect() }).collect();
    for spec in &d.houses {
        let more = houses(spec, &roads, &buildings, &areas);
        buildings.extend(more);
    }
    let rect = |r: Option<(P, P)>| r.map_or(Rect { min: Vec2::ZERO, max: Vec2::ZERO }, |(a, b)| Rect { min: v(a), max: v(b) });
    Layout {
        roads,
        belts,
        routes,
        shoulder_fields,
        field_mines: d.field_mines.iter().enumerate().filter(|(i, _)| d.mines_laid.as_ref().is_none_or(|laid| laid.contains(i))).map(|(_, (pts, w))| (Polyline::new(pts.iter().copied().map(v).collect()), *w)).collect(),
        barriers: d.barriers.iter().map(|b| Barrier { z0: b.z0, z1: b.z1, at: 1.0 / b.at_per, stacked: b.stacked, ap: 1.0 / b.ap_per }).collect(),
        areas,
        buildings,
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
        home: v(d.team.unwrap_or(d.home)),
        jam_z: d.jam_z,
        mortar_band: d.mortar_band,
    }
}

#[cfg(test)]
mod merge_tests {
    use super::{merge, uncomment};

    /// Every diorama's files are laid out as fmtron lays them (game/fmt.ron: 120 wide): `fmtron dioramas` fixes one
    /// that is not.
    #[test]
    fn the_dioramas_are_formatted() {
        let config = fmtron::Config::default().with_max_width(120);
        let index = [("index.ron", include_str!("../../../dioramas/index.ron"))];
        for d in crate::diorama::Diorama::ALL {
            for (name, text) in super::builtin(d).iter().chain(&d.shots()).chain(&index) {
                // (The library ends without a line break; the command line, which wrote the files, adds one.)
                let formatted = fmtron::format_ron(text, &config).unwrap_or_else(|e| panic!("{name}: {e}")) + "\n";
                assert!(formatted == *text, "{name} is not formatted: run `fmtron dioramas` in game/");
            }
        }
    }

    /// Every diorama reads, and its layout builds: every line, road and area it names is there.
    #[test]
    fn every_diorama_reads_and_lays_out() {
        for d in crate::diorama::Diorama::ALL {
            let data: super::DioramaData = super::parse(&super::builtin_text(d)).unwrap_or_else(|e| panic!("{d}: {e}"));
            let lay = super::layout(&data);
            assert!(!lay.routes.is_empty() || !lay.ua_positions.is_empty(), "{d}: neither routes nor positions");
        }
    }

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

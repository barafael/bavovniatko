//! Mala Tokmachka's ground: the slope from the N08 up to the crest, the tracks and tree lines, the barriers and
//! the lines (db/research/chapters/c09-robotyne.md §3). The box is the same size as Vuhledar's. +X is east, +Z
//! south: the attack climbs from the N08 highway in the north (about 70 m) to the crest in the south (about
//! 110 m), and the fields between are a grid of tree lines.

use super::{Barrier, Belt, HALF_X, Layout, Polyline, Rect, Road, RoadKind, Route, fbm, smoothstep, v};
use bevy::math::Vec2;

/// The Russian forward line: the tree line on the crest, and its trench along the northern edge.
pub const RU_LINE_Z: f32 = 720.0;
pub const TRENCH_Z: f32 = 702.0;
/// The Ukrainian forward tree line, where the ATGM teams overwatch the attack.
pub const UA_LINE_Z: f32 = -900.0;

/// The N08 highway, Polohy–Voskresenka, running east-south-east across the north of the box.
pub fn n08_z(x: f32) -> f32 {
    -1380.0 + 0.137 * x
}

pub fn height(x: f32, z: f32) -> f32 {
    // The long open slope up from the Kinska valley to the crest: the attackers climb under observation, so
    // the slope is concave to the crest (no dead ground below it), and levels off behind the trench.
    let t = ((z + 2000.0) / 2700.0).clamp(0.0, 1.0);
    let mut h = 68.0 + 42.0 * t.powf(1.6);
    h += 4.0 * (-((z - 850.0) / 320.0).powi(2)).exp();
    // Long low swells, and a little roughness.
    h += fbm(x / 1200.0, z / 1200.0, 3, 17) * 4.5;
    h += fbm(x / 240.0, z / 240.0, 3, 19) * 0.8;
    // A balka running north off the slope on the east side, and a shallower one in the west.
    let gx = 1350.0 + 120.0 * (z / 500.0).sin();
    let depth = 8.0 * (1.0 - smoothstep(-500.0, 700.0, z));
    h -= depth * (-((x - gx) / 60.0).powi(2)).exp();
    let gx2 = -1400.0 + 90.0 * (z / 380.0 + 1.0).sin();
    h -= 4.0 * (1.0 - smoothstep(-900.0, -100.0, z)) * (-((x - gx2) / 45.0).powi(2)).exp();
    h
}

pub fn build_layout() -> Layout {
    let n08 = Polyline::new((0..=24).map(|i| {
        let x = -HALF_X - 60.0 + i as f32 * (2.0 * HALF_X + 120.0) / 24.0;
        v(x, n08_z(x))
    }).collect());
    // The two tracks south, each along the east side of a north-south tree line: "a wide muddy track between
    // hedgerows" (Guardian). Dry on 8 June: dust, not mud.
    let west = Polyline::smoothed(vec![v(-470.0, -1795.0), v(-492.0, -1450.0), v(-503.0, -1000.0), v(-498.0, -500.0), v(-494.0, 0.0), v(-489.0, 400.0), v(-482.0, 700.0), v(-470.0, 1000.0)], 3);
    let east = Polyline::smoothed(vec![v(704.0, -1795.0), v(690.0, -1300.0), v(676.0, -800.0), v(667.0, -300.0), v(661.0, 200.0), v(656.0, 600.0), v(651.0, 720.0), v(640.0, 1000.0)], 3);
    // Field tracks along the Ukrainian forward tree line and behind the Russian one.
    let lateral = Polyline::new(vec![v(-2350.0, UA_LINE_Z - 14.0), v(2350.0, UA_LINE_Z - 14.0)]);
    let rear = Polyline::new(vec![v(-2350.0, 1000.0), v(2350.0, 1000.0)]);
    let roads = vec![
        Road { line: n08, width: 9.0, kind: RoadKind::Asphalt },
        Road { line: west.clone(), width: 6.0, kind: RoadKind::Dirt },
        Road { line: east.clone(), width: 6.0, kind: RoadKind::Dirt },
        Road { line: lateral, width: 3.5, kind: RoadKind::Track },
        Road { line: rear, width: 3.5, kind: RoadKind::Track },
    ];

    // Shelterbelts: main belts about 0.55 km apart east-west, cross-belts about 1.15 km apart (dossier §3, the
    // OSM and Sentinel-2 sample box around Robotyne).
    let mut belts = Vec::new();
    for (i, z) in [-1450.0f32, UA_LINE_Z, -350.0, 200.0, RU_LINE_Z, 1270.0].into_iter().enumerate() {
        let wob = |x: f32| z + 7.0 * (x / 340.0 + i as f32 * 1.7).sin();
        let line = Polyline::new((0..=40).map(|k| {
            let x = -HALF_X + 40.0 + k as f32 * (2.0 * HALF_X - 80.0) / 40.0;
            v(x, wob(x))
        }).collect());
        // The crest belt is the Russian position: wider, thicker.
        let width = if z == RU_LINE_Z { 22.0 } else { 13.0 + 4.0 * (i % 2) as f32 };
        belts.push(Belt { line, width, gaps: vec![] });
    }
    for x in [-1650.0f32, -520.0, 640.0, 1780.0] {
        let line = Polyline::new(vec![v(x, -1700.0), v(x - 12.0, 1650.0)]);
        belts.push(Belt { line, width: 12.0, gaps: vec![] });
    }
    super::cut_gaps(&mut belts, &roads);

    // The columns deploy only once through the barrier, at the trench: they never got that far.
    let routes = vec![
        Route { line: west.clone(), deploy_s: west.project(v(-485.0, 600.0)).1, objective: v(-470.0, TRENCH_Z) },
        Route { line: east.clone(), deploy_s: east.project(v(655.0, 600.0)).1, objective: v(650.0, TRENCH_Z) },
    ];

    // The Surovikin line's first belt: a forward field of older and remotely laid mines, then the barrier proper,
    // about 480 m deep (RUSI: 120 m growing to about 500 m), anti-tank mines stacked two high, anti-personnel
    // mines among them, across the tracks as well as the fields.
    let barriers = vec![
        Barrier { z0: -430.0, z1: -250.0, at: 1.0 / 260.0, stacked: 0.0, ap: 1.0 / 400.0 },
        Barrier { z0: 110.0, z1: 590.0, at: 1.0 / 95.0, stacked: 0.45, ap: 1.0 / 70.0 },
    ];

    let ua_positions = vec![
        // ATGM teams in the forward tree line, at the heads of the two tracks (where the belts' gaps let them see
        // up the lanes) and between, watching the crest 1.6 km away.
        v(-520.0, UA_LINE_Z + 4.0), v(640.0, UA_LINE_Z + 5.0), v(260.0, UA_LINE_Z + 3.0),
        // Riflemen covering the start line.
        v(-180.0, UA_LINE_Z + 5.0), v(-1350.0, UA_LINE_Z + 4.0),
    ];
    let ru_positions = vec![
        // The 291st's first line: platoon positions along the crest trench, Kornet teams at the heads of the
        // tracks, looking straight down them through the belts' gaps.
        v(-1500.0, TRENCH_Z), v(-930.0, TRENCH_Z - 2.0), v(-468.0, TRENCH_Z), v(180.0, TRENCH_Z - 3.0),
        v(666.0, TRENCH_Z), v(1350.0, TRENCH_Z - 1.0), v(1900.0, TRENCH_Z),
    ];
    // The Russian guns have every lane registered: where the tracks cross the minefields.
    let trps = vec![
        west.point(west.project(v(-497.0, -340.0)).1),
        west.point(west.project(v(-492.0, 160.0)).1),
        west.point(west.project(v(-488.0, 420.0)).1),
        east.point(east.project(v(666.0, -340.0)).1),
        east.point(east.project(v(661.0, 160.0)).1),
        east.point(east.project(v(657.0, 420.0)).1),
    ];
    // Last year's wrecks, from the Russian advance in 2022, still in the fields.
    let old_wrecks = vec![(v(-1180.0, -180.0), 2.4), (v(1480.0, 330.0), 0.9), (v(150.0, -620.0), 4.1)];

    let none = Rect { min: Vec2::ZERO, max: Vec2::ZERO };
    Layout {
        roads,
        belts,
        routes,
        shoulder_fields: Vec::new(),
        field_mines: Vec::new(),
        barriers,
        river: None,
        town: none,
        dachas: none,
        pavlivka: none,
        mykilske: none,
        ua_positions,
        ru_positions,
        trps,
        old_wrecks,
        advance: 1.0,
        foothold_z: TRENCH_Z - 40.0,
        defence_z: TRENCH_Z,
        home: v(-200.0, -1600.0),
        jam_z: 100.0,
        mortar_band: (-600.0, TRENCH_Z),
    }
}

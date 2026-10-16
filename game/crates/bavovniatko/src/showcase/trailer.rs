//! The trailer's tours (`BAV_TOUR=trailer`, design/trailer.md): about five minutes of both dioramas, the film's
//! day and everything the dioramas can show besides: the observer's roof, RAAM and HIMARS at Vuhledar; their
//! jammers by night and one shelled out, the Ka-52 tell, the FPV through its goggles and the Bradleys sent for
//! the wounded south of Mala Tokmachka. The film's rules hold (250–600 m off, 100–200 m up, fov 7–14, the horizon
//! out of frame, cards as hard cuts). The times are from the timeline runs of 2026-10-16 and the scouts after them.

use super::{Aim, Card, Shot, shot};
use crate::fire::Strike;
use bav_core::vocab::Target::{MenNear, Point, Vehicle as Column, Wave};
use bevy::prelude::*;

pub(super) fn shots() -> Vec<Shot> {
    let f = shot;
    let card = |title: &'static str, line: &'static str, note: &'static str, secs: f32| Card { title, line, note, secs };
    let d = Aim::Drone;
    if crate::diorama::diorama() == crate::diorama::Diorama::Robotyne {
        let start_z = crate::geo::layout().start_z.expect("Mala Tokmachka has a start line");
        return vec![
            // Their EW, in the dark behind the crest: a jammer's mast and its red lamp; the link frays on the display.
            Shot {
                card: Some(card("Four months later. The other side of the minefield.", "South of МАЛА ТОКМАЧКА (MALA TOKMACHKA), 8 June 2023", "The 47th Mechanised Brigade goes forward into the Russian minefields. Now the columns are ours.", 6.0)),
                ..f(76.0, 7.0, (-420.0, 520.0, 170.0), (-410.0, 540.0, 170.0), d(Point(Vec2::new(-350.0, 950.0))), (9.0, 7.5))
            },
            // First light: the first group moves off down the western track.
            f(132.0, 9.0, (-400.0, -1150.0, 160.0), (-410.0, -1115.0, 160.0), d(Column(3)), (14.0, 10.0)),
            // Eye level at the start line: the Leopard 2R with its plow passes the overwatch.
            f(152.0, 10.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Ground(Vec2::new(-520.0, start_z), false), (54.0, 54.0)),
            // The plow in the forward minefield, and the Kornets from the crest.
            f(296.0, 11.0, (-420.0, -540.0, 140.0), (-430.0, -510.0, 140.0), d(Column(0)), (8.0, 7.0)),
            // 155 mm on the jammer; its lamp goes out. Then the satellites come back clean: the Ka-52s are a minute out.
            Shot {
                fire_at: &[(334.0, Strike::He, (-350.0, 950.0))],
                ..f(359.0, 13.0, (-430.0, 560.0, 170.0), (-420.0, 580.0, 170.0), d(Point(Vec2::new(-350.0, 950.0))), (9.0, 8.0))
            },
            // The second group, eleven, on the eastern track.
            Shot {
                card: Some(card("05:29", "The second group, eleven vehicles, on the eastern track.", "", 2.5)),
                ..f(400.0, 8.0, (720.0, -700.0, 150.0), (700.0, -660.0, 150.0), d(Wave(2, 0)), (12.0, 9.0))
            },
            // Stopped vehicles under the Ka-52s' missiles; crews out.
            f(452.0, 11.0, (-560.0, -430.0, 150.0), (-570.0, -400.0, 150.0), d(Column(1)), (9.0, 7.0)),
            // A Bradley sent in for the wounded: it stops at the men (519), puts down its smoke, takes them on.
            Shot {
                card: Some(card("06:22", "Four Bradleys go back in for the men.", "", 2.5)),
                ..f(507.0, 14.0, (-420.0, -560.0, 130.0), (-430.0, -530.0, 130.0), d(Wave(3, 0)), (10.0, 8.0))
            },
            // The FPV: launched from the team behind the start line, out over the column towards the crest...
            Shot {
                card: Some(card("06:47", "One strike drone for the morning.", "", 2.5)),
                ..f(590.0, 8.0, (-560.0, -380.0, 110.0), (-560.0, -380.0, 110.0), Aim::Fpv(Vec2::new(-150.0, 724.0)), (8.0, 8.0))
            },
            // ...and the dive onto a dug-in T-72 on the crest (in at 652).
            f(642.0, 12.0, (-560.0, -380.0, 110.0), (-560.0, -380.0, 110.0), Aim::Fpv(Vec2::new(-150.0, 724.0)), (8.0, 8.0)),
            // The pull-back, from the observers on the water tower: the track, the smoke, what is left on it. (From the
            // drone the men walking back are under the Bradleys' smoke.)
            Shot {
                card: Some(card("08:50", "Everyone back to the start line.", "", 2.5)),
                ..f(1012.0, 10.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Op(Vec2::new(-495.0, -560.0)), (10.0, 8.0))
            },
            // Late morning: the row of wrecks along the track, still smoking.
            Shot { hud: false, ..f(1300.0, 9.0, (-600.0, -520.0, 200.0), (-590.0, -480.0, 200.0), d(Point(Vec2::new(-492.0, -330.0))), (14.0, 12.0)) },
            // The ledger.
            Shot { debrief: true, hud: false, ..f(1308.0, 7.0, (-590.0, -480.0, 200.0), (-590.0, -480.0, 200.0), d(Point(Vec2::new(-492.0, -330.0))), (12.0, 12.0)) },
        ];
    }
    vec![
        // Dawn: the January wrecks on the dacha road, near-overhead.
        Shot {
            card: Some(card("ВУГЛЕДАР (VUHLEDAR)", "6 February 2023", "Russian columns attack the 72nd Mechanised Brigade. The roads are left clear; the fields beside them are mined.", 5.0)),
            hud: false,
            ..f(6.0, 7.0, (1270.0, 250.0, 140.0), (1250.0, 215.0, 140.0), d(Point(Vec2::new(1228.0, 203.0))), (12.0, 9.0))
        },
        // The column in single file behind its smoke; the net's report and the first jamming.
        f(15.0, 8.0, (1130.0, 70.0, 170.0), (1110.0, 10.0, 170.0), d(Column(2)), (14.0, 10.0)),
        // The Stugna-P sight on the lead tank, the launch at +4 s, the missile all the way.
        Shot { launch: Some(4.0), ..f(62.0, 12.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Ground(Vec2::new(930.0, -658.0), true), (54.0, 54.0)) },
        // The kill zone, wide: the column stacks at the deploy point.
        f(76.0, 9.0, (1110.0, -380.0, 170.0), (1090.0, -410.0, 170.0), d(Column(1)), (11.0, 8.0)),
        // The kill zone, close: mines, crews in the snow, 155 mm landing among them. A shot's missions are called
        // while it is the next shot, so not before the last one ends; the column is about 1200,90 then.
        Shot { fire_at: &[(86.0, Strike::He, (1200.0, 90.0))], ..f(109.0, 10.0, (1080.0, -60.0, 150.0), (1070.0, -80.0, 150.0), d(Point(Vec2::new(1200.0, 90.0))), (9.0, 8.0)) },
        // TOS-1A on the town's southern blocks.
        f(163.0, 8.0, (-110.0, -900.0, 160.0), (-130.0, -925.0, 160.0), d(Point(Vec2::new(-290.0, -1170.0))), (20.0, 16.0)),
        // The second column passes the morning's wrecks at the middle belt.
        Shot {
            card: Some(card("08:40", "A second column. The same road.", "", 2.5)),
            ..f(470.0, 9.0, (1190.0, -50.0, 150.0), (1170.0, -100.0, 150.0), d(Wave(2, 0)), (10.0, 8.0))
        },
        // RAAM: mines scattered onto the lane in front of it (they land 511–521); and HIMARS called on its
        // stalled tail, 55 s in the air.
        Shot {
            fire_at: &[(480.0, Strike::Raam, (1233.0, 150.0)), (505.0, Strike::Himars, (1335.0, 350.0))],
            ..f(511.0, 10.0, (1110.0, 40.0, 180.0), (1120.0, 55.0, 180.0), d(Point(Vec2::new(1233.0, 150.0))), (12.0, 10.0))
        },
        // HIMARS on the stalled tail of the column.
        f(559.0, 10.0, (1200.0, 180.0, 180.0), (1210.0, 195.0, 180.0), d(Point(Vec2::new(1335.0, 350.0))), (13.0, 11.0)),
        // The column's lead drives onto the scattered mines.
        f(578.0, 12.0, (1120.0, 20.0, 170.0), (1115.0, 10.0, 170.0), d(Wave(2, 0)), (11.0, 9.0)),
        // Alga's BMP-1s in the shoulder mines on the Pavlivka road.
        Shot {
            card: Some(card("10:20", "The Alga battalion, from Павлівка (Pavlivka).", "", 2.5)),
            ..f(790.0, 9.0, (-740.0, -240.0, 150.0), (-760.0, -280.0, 150.0), d(Wave(3, 1)), (10.0, 7.0))
        },
        // From the observers on the nine-storey roof: the Pavlivka road below the town.
        f(806.0, 7.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Op(Vec2::new(-820.0, -230.0)), (10.0, 8.0)),
        // Flown: the operator pushes in over the third column at its wrecks.
        Shot {
            fly: &[(3.0, 9.0, "w+"), (9.0, 11.0, "z")],
            ..f(1036.0, 12.0, (1250.0, -80.0, 160.0), (0.0, 0.0, 0.0), Aim::Fly(Wave(4, 0)), (20.0, 20.0))
        },
        // Infantry on foot under the mortars.
        Shot {
            card: Some(card("13:20", "No more armour. Infantry, on foot.", "", 2.5)),
            ..f(1190.0, 9.0, (-900.0, 300.0, 100.0), (-910.0, 275.0, 100.0), d(MenNear(Vec2::new(-950.0, 250.0))), (9.0, 7.0))
        },
        // Late afternoon over the dacha road: everything that came down it is still there.
        Shot { hud: false, ..f(1440.0, 8.0, (1060.0, -630.0, 220.0), (1090.0, -570.0, 220.0), d(Point(Vec2::new(1040.0, -330.0))), (22.0, 18.0)) },
        // The ledger.
        Shot { debrief: true, hud: false, ..f(1448.0, 7.0, (1090.0, -570.0, 220.0), (1090.0, -570.0, 220.0), d(Point(Vec2::new(1040.0, -330.0))), (18.0, 18.0)) },
    ]
}
